package main

import (
	"context"
	"flag"
	"fmt"
	"os"
	"path"
	"testing"

	"github.com/gorilla/mux"
	"github.com/joho/godotenv"

	"git.kundeng.us/phoenix/textsender-auth/internal/config"
	"git.kundeng.us/phoenix/textsender-auth/internal/db"
	"git.kundeng.us/phoenix/textsender-auth/internal/handler"
	"git.kundeng.us/phoenix/textsender-auth/internal/handler/endpoint"
	"git.kundeng.us/phoenix/textsender-auth/internal/model"
	"git.kundeng.us/phoenix/textsender-auth/internal/store"
)

var testRouter *mux.Router

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

	userStore := model.NewUserStore(database.Pool)
	serviceStore := store.NewServiceStore(database.Pool)
	userHandler := handler.NewUserHandler(userStore)
	loginHandler := handler.NewLoginHandler(userStore)
	serviceHandler := handler.NewServiceHandler(serviceStore)

	testRouter = mux.NewRouter()
	testRouter.HandleFunc(endpoint.Register, userHandler.Register).Methods("POST")
	testRouter.HandleFunc(endpoint.Login, loginHandler.Login).Methods("POST")
	testRouter.HandleFunc(endpoint.CreateServiceUser, serviceHandler.Register).Methods("POST")
	testRouter.HandleFunc(endpoint.LoginServiceUser, serviceHandler.Login).Methods("POST")

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
		DBConnString: dbConnString,
		ServerPort:   *port,
		ResetDB:      *resetDb,
	}
}

func resetTestDB(t *testing.T) {
	t.Helper()
	_, err := db.Pool.Exec(context.Background(), "DELETE FROM users")
	if err != nil {
		t.Fatalf("Failed to reset test database: %v", err)
	}
}
