package model

import (
	"github.com/golang-jwt/jwt/v5"
)

type Claims struct {
	UserId string `json:"user_id"`
	Role   string `json:"role"`
	jwt.RegisteredClaims
}

type Login struct {
	AccessToken string `json:"access_token"`
}
