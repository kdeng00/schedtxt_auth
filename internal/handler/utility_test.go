package handler

import (
	"log"
	"os"
	"path"

	"git.kundeng.us/phoenix/textsender-models/tx0/user"
	"github.com/joho/godotenv"

	"git.kundeng.us/phoenix/textsender-auth/internal/config"
)

func GetTestUser() user.User {
	return user.User{Username: "ghost", PhoneNumber: "+1234567890", Password: "dfgdffddfd"}
}

func GetConfig() *config.Config {
	err := godotenv.Load()
	if err != nil {
		cwd, _ := os.Getwd()
		envPath := path.Join(cwd, "../..", ".env")
		if err = godotenv.Load(envPath); err != nil {
			prevPath := path.Join(envPath, "../..", ".env")
			if err = godotenv.Load(prevPath); err != nil {
				log.Fatal("Error loading .env file")
			}
		}
	}

	unpackedConnString := config.UnpackDBConnString()
	dbConnString := unpackedConnString.Parse()

	return &config.Config{
		DBConnString:       dbConnString,
		ServerPort:         config.Port,
		ResetDB:            false,
		EnableRegistration: config.CheckRegistration(),
	}
}
