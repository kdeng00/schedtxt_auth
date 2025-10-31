package utility

import (
	"time"

	"github.com/golang-jwt/jwt/v5"

	"git.kundeng.us/phoenix/textsender-auth/internal/config"
	"git.kundeng.us/phoenix/textsender-auth/internal/model"
	textmod "git.kundeng.us/phoenix/textsender-models"
)

const ROLE_TYPE = "regular"
const TOKEN_TYPE = "Bearer"

type TokenGenerator struct {
	SecretKey []byte
}

func (t *TokenGenerator) SetSecretKey(secretKey string) {
	t.SecretKey = []byte(secretKey)
}

func (t *TokenGenerator) GenerateToken(user model.User) (*textmod.token.Login, error) {
	issuedAt := time.Now()
	expirationTime := time.Now().Add(4 * time.Hour)
	claims := t.generateClaims(user, TOKEN_TYPE, issuedAt, expirationTime)

	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	if tokenString, err := token.SignedString(t.SecretKey); err != nil {
		return nil, err
	} else {
		return &token.Login{AccessToken: tokenString, TokenType: TOKEN_TYPE, ExpiresIn: expirationTime.Unix()}, nil
	}
}

func (t *TokenGenerator) generateClaims(user model.User, role string, issuedAt time.Time, expiredAt time.Time) textmod.token.Claims {
	return textmod.token.Claims{
		UserId: user.Id.String(),
		Role:   role,
		RegisteredClaims: jwt.RegisteredClaims{
			Issuer:    config.App_Name,
			ExpiresAt: jwt.NewNumericDate(expiredAt),
			IssuedAt:  jwt.NewNumericDate(issuedAt),
		},
	}
}
