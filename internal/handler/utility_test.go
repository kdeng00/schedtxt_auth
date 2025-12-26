package handler

import (
	"context"
	"fmt"
	"log"
	"os"
	"path"

	"git.kundeng.us/phoenix/textsender-models/tx0/user"
	"github.com/joho/godotenv"

	"git.kundeng.us/phoenix/textsender-auth/internal/config"
	"git.kundeng.us/phoenix/textsender-auth/internal/store/mock"
	"git.kundeng.us/phoenix/textsender-auth/internal/utility"
)

func GetTestUser() user.User {
	return user.User{Username: "ghost", PhoneNumber: "+1234567890", Password: "Dfgdffd343dfd!"}
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

func createUser(ctx context.Context, userStore *mock.MockUserStore) (*user.User, *string, error) {
	testUser := GetTestUser()
	unhashedPassword := testUser.Password
	hashing := utility.HashMash{}
	if err := hashing.SetPassword(testUser.Password); err != nil {
		return nil, nil, fmt.Errorf("Error setting password: %v", err)
	}

	hashedPassword, err := hashing.HashPassword()
	if err != nil {
		return nil, nil, err
	}

	testUser.Password = hashedPassword
	userStore.CreateUser(ctx, &testUser)

	return &testUser, &unhashedPassword, nil
}
