package main

import (
	"context"
	"flag"
	"fmt"
	"net/http"
	"os"
	"path"
	"testing"

	"github.com/go-chi/chi/v5"
	"github.com/joho/godotenv"

	"git.kundeng.us/phoenix/textsender-auth/internal/config"
	"git.kundeng.us/phoenix/textsender-auth/internal/db"
	"git.kundeng.us/phoenix/textsender-auth/internal/handler"
	"git.kundeng.us/phoenix/textsender-auth/internal/handler/endpoint"
	mdleware "git.kundeng.us/phoenix/textsender-auth/internal/middleware"
	"git.kundeng.us/phoenix/textsender-auth/internal/services"
	"git.kundeng.us/phoenix/textsender-auth/internal/store"
)

var testRouter *chi.Mux

func TestMain(m *testing.M) {
	cfg := load()

	database, err := db.NewDatabase(cfg.GetDBConnString())
	if err != nil {
		fmt.Println(err.Error())
		panic("Failed to initialize database")
	}
	defer database.Close()

	ctx := context.Background()
	err = database.ResetDatabase(ctx)
	if err != nil {
		fmt.Println(err.Error())
		panic("Failed to initialize database")
	}

	userStore := store.NewUserStore(database.Pool)
	serviceStore := store.NewServiceStore(database.Pool)
	userHandler := handler.NewUserHandler(cfg, userStore)
	loginHandler := handler.NewLoginHandler(cfg, userStore)
	serviceHandler := handler.NewServiceHandler(cfg, serviceStore)
	refreshHandler := handler.NewRefreshHandler(cfg, userStore, serviceStore)

	testRouter = chi.NewRouter()
	jwtService := services.NewJWTService(config.GetSecretKey())
	testRouter.Method("POST", endpoint.Register, http.HandlerFunc(userHandler.Register))
	testRouter.Method("POST", endpoint.Login, http.HandlerFunc(loginHandler.Login))
	testRouter.Method("POST", endpoint.CreateServiceUser, http.HandlerFunc(serviceHandler.Register))
	testRouter.Method("POST", endpoint.LoginServiceUser, http.HandlerFunc(serviceHandler.Login))
	testRouter.Method("POST", endpoint.TokenRefresh, http.HandlerFunc(refreshHandler.Refresh))
	testRouter.Method("PATCH", endpoint.UpdatePassword, mdleware.AuthMiddleware(jwtService)(http.HandlerFunc(loginHandler.UpdatePassword)))

	code := m.Run()
	os.Exit(code)
}

func load() *config.Config {
	resetDb := flag.Bool("reset-db", false, "Reset the database schema and exit")
	port := flag.String("port", config.Port, "Server port")
	flag.Parse()

	cwd, _ := os.Getwd()
	envPath := path.Join(cwd, ".env")

	err := godotenv.Load(envPath)
	if err != nil {
		envPath = path.Join(cwd, "../..", ".env")
		if err := godotenv.Load(envPath); err != nil {
			panic("Error loading .env file: " + err.Error())
		}
	}

	unpackedConnString := config.UnpackDBConnString()
	dbConnString := unpackedConnString.Parse()

	return &config.Config{
		DBConnString:       dbConnString,
		ServerPort:         *port,
		ResetDB:            *resetDb,
		EnableRegistration: config.CheckRegistration(),
	}
}

func resetTestDB(t *testing.T) {
	t.Helper()
	_, err := db.Pool.Exec(context.Background(), "DELETE FROM users")
	if err != nil {
		t.Fatalf("Failed to reset test database: %v", err)
	}
}
