package utility

import (
	"time"

	"github.com/golang-jwt/jwt/v5"

	"git.kundeng.us/phoenix/textsender-auth/internal/model"
)

type TokenGenerator struct {
	SecretKey []byte
}

func (t *TokenGenerator) SetSecretKey(secretKey string) {
	t.SecretKey = []byte(secretKey)
}

func (t *TokenGenerator) GenerateToken(user model.User) (*model.Login, error) {
	claims := t.generateClaims(user, "regular")

	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	if tokenString, err := token.SignedString(t.SecretKey); err != nil {
		return nil, err
	} else {
		return &model.Login{AccessToken: tokenString}, nil
	}
}

func (t *TokenGenerator) generateClaims(user model.User, role string) model.Claims {
	return model.Claims{
		UserId: user.Id.String(),
		Role:   role,
		RegisteredClaims: jwt.RegisteredClaims{
			Issuer:    "textsender-auth",
			ExpiresAt: jwt.NewNumericDate(time.Now().Add(24 * time.Hour)),
			IssuedAt:  jwt.NewNumericDate(time.Now()),
		},
	}
}
