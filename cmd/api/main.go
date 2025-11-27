package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"github.com/swaggo/http-swagger/v2"

	_ "git.kundeng.us/phoenix/textsender-auth/docs"
	"git.kundeng.us/phoenix/textsender-auth/internal/config"
	database "git.kundeng.us/phoenix/textsender-auth/internal/db"
	"git.kundeng.us/phoenix/textsender-auth/internal/handler"
	"git.kundeng.us/phoenix/textsender-auth/internal/handler/endpoint"
	mdleware "git.kundeng.us/phoenix/textsender-auth/internal/middleware"
	"git.kundeng.us/phoenix/textsender-auth/internal/store"
)

// @title           textsender-auth
// @version         1.0
// @description     Auth API to send text messages

// @host      localhost:9080
// @BasePath  /api/v1

// @securityDefinitions.apikey  BearerAuth
// @in                          header
// @name                        Authorization
// @description                 JWT Bearer Token
func main() {
	cfg := config.Load()
	if cfg == nil {
		fmt.Println("Error initializing config")
		os.Exit(-1)
	}

	db, err := database.NewDatabase(cfg.GetDBConnString())
	if err != nil {
		log.Fatalf("Failed to connect to database: %v", err)
	}
	defer db.Close()

	ctx := context.Background()

	if cfg.ResetDB {
		if err := db.ResetDatabase(ctx); err != nil {
			log.Fatalf("Failed to reset database: %v", err)
		} else {
			log.Println("Resetting database")
			log.Println("Database reset completed. Exiting.")
		}
		return
	} else {
		if exists, err := database.TableExists(ctx, db.Pool, "users"); err == nil {
			if !exists {
				if err = db.ResetDatabase(ctx); err != nil {
					fmt.Println("Error:", err)
				} else {
					fmt.Println("Database reset")
				}
			}
		} else {
			fmt.Println("Error:", err)
		}
	}

	// Services
	userStore := store.NewUserStore(db.Pool)
	serviceStore := store.NewServiceStore(db.Pool)

	userHandler := handler.NewUserHandler(cfg, userStore)
	loginHandler := handler.NewLoginHandler(cfg, userStore)
	serviceHandler := handler.NewServiceHandler(cfg, serviceStore)
	refreshHandler := handler.NewRefreshHandler(cfg, userStore, serviceStore)

	router := chi.NewRouter()

	router.Use(middleware.Logger)
	router.Use(middleware.Recoverer)
	router.Use(middleware.Timeout(60 * time.Second))
	router.Use(mdleware.JSONContentType)

	router.Method("Post", endpoint.Register, http.HandlerFunc(userHandler.Register))
	router.Method("Post", endpoint.Login, http.HandlerFunc(loginHandler.Login))
	router.Method("Post", endpoint.CreateServiceUser, http.HandlerFunc(serviceHandler.Register))
	router.Method("Post", endpoint.LoginServiceUser, http.HandlerFunc(serviceHandler.Login))
	router.Method("Post", endpoint.TokenRefresh, http.HandlerFunc(refreshHandler.Refresh))

	router.Method("GET", "/swagger/*", httpSwagger.Handler(
		httpSwagger.URL(fmt.Sprintf("http://localhost:%s/swagger/doc.json", config.Port)),
	))

	// Start server
	server := &http.Server{
		Addr:         ":" + cfg.ServerPort,
		Handler:      router,
		ReadTimeout:  15 * time.Second,
		WriteTimeout: 15 * time.Second,
		IdleTimeout:  60 * time.Second,
	}

	// Graceful shutdown
	go func() {
		log.Printf("Server starting on port %s", cfg.ServerPort)
		if err := server.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			log.Fatalf("Server failed to start: %v", err)
		}
	}()

	// Wait for interrupt signal
	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
	<-quit
	log.Println("Shutting down server...")

	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	if err := server.Shutdown(ctx); err != nil {
		log.Fatalf("Server forced to shutdown: %v", err)
	}

	log.Println("Server exited")
}
