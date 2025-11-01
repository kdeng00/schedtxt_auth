package utility

import (
	"time"

	"github.com/golang-jwt/jwt/v5"
	// "github.com/google/uuid"

	"git.kundeng.us/phoenix/textsender-auth/internal/config"
	"git.kundeng.us/phoenix/textsender-models/pkg/token"
	"git.kundeng.us/phoenix/textsender-models/pkg/user"
)

const ROLE_TYPE = "regular"
const TOKEN_TYPE = "Bearer"

type TokenGenerator struct {
	SecretKey []byte
}

func (t *TokenGenerator) SetSecretKey(secretKey string) {
	t.SecretKey = []byte(secretKey)
}

func (t *TokenGenerator) GenerateToken(user user.User) (*token.Login, error) {
	issuedAt := time.Now()
	expirationTime := time.Now().Add(4 * time.Hour)
	claims := t.generateClaims(user, TOKEN_TYPE, issuedAt, expirationTime)

	myToken := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	if tokenString, err := myToken.SignedString(t.SecretKey); err != nil {
		return nil, err
	} else {
		return &token.Login{AccessToken: tokenString, TokenType: TOKEN_TYPE, ExpiresIn: expirationTime.Unix()}, nil
	}
}

func (t *TokenGenerator) generateClaims(user user.User, role string, issuedAt time.Time, expiredAt time.Time) token.Claims {
	return token.Claims{
		// UserId: user.Id.String(),
		UserId: user.Id,
		Role:   role,
		RegisteredClaims: jwt.RegisteredClaims{
			Issuer:    config.App_Name,
			ExpiresAt: jwt.NewNumericDate(expiredAt),
			IssuedAt:  jwt.NewNumericDate(issuedAt),
		},
	}
}
