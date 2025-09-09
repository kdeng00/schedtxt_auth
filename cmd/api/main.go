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

	"git.kundeng.us/phoenix/textsender-auth/internal/config"
	"git.kundeng.us/phoenix/textsender-auth/internal/db"
	"git.kundeng.us/phoenix/textsender-auth/internal/handler"
	"git.kundeng.us/phoenix/textsender-auth/internal/handler/endpoint"
	mdleware "git.kundeng.us/phoenix/textsender-auth/internal/middleware"
	"git.kundeng.us/phoenix/textsender-auth/internal/model"
)

func main() {
	fmt.Println("textsender-auth")

	cfg := config.Load()

	db, err := db.NewDatabase(cfg.GetDBConnString())
	if err != nil {
		log.Fatalf("Failed to connect to database: %v", err)
	}
	defer db.Close()

	ctx := context.Background()

	if cfg.ResetDB {
		log.Println("Resetting database")
		if err := db.ResetDatabase(ctx); err != nil {
			log.Fatalf("Failed to reset database: %v", err)
		}
		log.Println("Database reset completed. Exiting.")
		return
	}

	// Services
	userStore := model.NewUserStore(db.Pool)
	userHandler := handler.NewUserHandler(userStore)

	router := chi.NewRouter()

	router.Use(middleware.Logger)
	router.Use(middleware.Recoverer)
	router.Use(middleware.Timeout(60 * time.Second))
	router.Use(mdleware.JSONContentType)

	router.Post(endpoint.Register, userHandler.Register)

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
