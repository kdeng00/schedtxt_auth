package services

import (
	"time"

	txtmodels_token "git.kundeng.us/phoenix/textsender-models/tx0/token"
	txtmodels_user "git.kundeng.us/phoenix/textsender-models/tx0/user"
	"github.com/golang-jwt/jwt/v5"
)

type JWTService struct {
	secretKey []byte
}

func NewJWTService(secretKey string) *JWTService {
	return &JWTService{
		secretKey: []byte(secretKey),
	}
}

func (s *JWTService) ValidateToken(tokenString string) (*txtmodels_user.User, error) {
	// TODO: Include more user information in the claims to populate user
	claims := &txtmodels_token.Claims{}

	token, err := jwt.ParseWithClaims(tokenString, claims, func(token *jwt.Token) (interface{}, error) {
		// Validate the signing method
		if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, jwt.ErrSignatureInvalid
		}
		return s.secretKey, nil
	})

	if err != nil {
		return nil, err
	}

	if !token.Valid {
		return nil, jwt.ErrSignatureInvalid
	}

	// Check token expiration
	if time.Now().After(claims.ExpiresAt.Time) {
		return nil, jwt.ErrTokenExpired
	}

	return &txtmodels_user.User{
		Id: claims.UserId,
	}, nil
}
