package utility

import (
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/golang-jwt/jwt/v5"

	"git.kundeng.us/phoenix/textsender-auth/internal/config"
	"git.kundeng.us/phoenix/textsender-models/pkg/token"
	"git.kundeng.us/phoenix/textsender-models/pkg/user"
)

const ROLE_TYPE = "regular"
const TOKEN_TYPE = "Bearer"

type TokenGenerator struct {
	SecretKey []byte
	hourOffset time.Duration
}

func (t *TokenGenerator) SetSecretKey(secretKey string) {
	t.SecretKey = []byte(secretKey)
}

func (t *TokenGenerator) SetHourOffset(offset time.Duration) error {
	if offset < 48 {
		t.hourOffset = offset
		return nil
	} else {
		return fmt.Errorf("No change")
	}
}

func (t *TokenGenerator) GenerateToken(usr any) (*token.Login, error) {
	issuedAt := time.Now()
	if t.hourOffset == 0 {
		t.hourOffset = 4
	}

	expirationTime := time.Now().Add(t.hourOffset * time.Hour)

	if claims, err := t.generateClaims(usr, TOKEN_TYPE, issuedAt, expirationTime); err != nil {
		return nil, fmt.Errorf("Error generating claims: %v", err)
	} else {
		myToken := jwt.NewWithClaims(jwt.SigningMethodHS256, *claims)
		if tokenString, err := myToken.SignedString(t.SecretKey); err != nil {
			return nil, err
		} else {
			return &token.Login{AccessToken: tokenString, TokenType: TOKEN_TYPE, ExpiresIn: expirationTime.Unix()}, nil
		}
	}
}

func (t *TokenGenerator) VerifyToken(accessToken string) (bool, error) {
	clms := &token.Claims{}

	if tken, err := t.parseTokenWithClaims(accessToken, clms); err != nil {
		return false, nil
	} else {
		if tken.Valid {
			if clms != nil {
				if clms.UserId != uuid.Nil {
					return true, nil
				} else {
					return false, fmt.Errorf("User Id was not set")
				}
			} else {
				return false, fmt.Errorf("Claims not parsed")
			}
		} else {
			return false, fmt.Errorf("Invalid access token")
		}
	}
}

func (t *TokenGenerator) ExtractIdFromToken(accessToken string) (uuid.UUID, error) {
	clms := &token.Claims{}

	if tken, err := t.parseTokenWithClaims(accessToken, clms); err != nil {
		return uuid.Nil, nil
	} else {
		if tken.Valid {
			if clms != nil {
				if clms.UserId != uuid.Nil {
					return clms.UserId, nil
				} else {
					return uuid.Nil, fmt.Errorf("User Id was not set")
				}
			} else {
				return uuid.Nil, fmt.Errorf("Claims not parsed")
			}
		} else {
			return uuid.Nil, fmt.Errorf("Invalid access token")
		}
	}
}


func (t *TokenGenerator) parseTokenWithClaims(accessToken string, claims *token.Claims) (*jwt.Token, error) {
	tken, err := jwt.ParseWithClaims(accessToken, claims, func(tken *jwt.Token) (any, error) {
		if _, ok := tken.Method.(*jwt.SigningMethodHMAC); !ok {
		    return nil, fmt.Errorf("unexpected signing method: %v", tken.Header["alg"])
		}
		return t.SecretKey, nil
	}, jwt.WithValidMethods([]string{jwt.SigningMethodHS256.Alg()}))

	if err != nil {
		return nil, err
	} else {
		return tken, nil
	}
}

func (t *TokenGenerator) generateClaims(usr any, role string, issuedAt time.Time, expiredAt time.Time) (*token.Claims, error) {
	var id uuid.UUID
	switch val := usr.(type) {
	case user.User:
		id = val.Id
	case *user.User:
		id = val.Id
	case user.ServiceUser:
		id = val.Id
	case *user.ServiceUser:
		id = val.Id
	default:
		return nil, fmt.Errorf("Invalid type")
	}

	return &token.Claims{
		UserId: id,
		Role:   role,
		RegisteredClaims: jwt.RegisteredClaims{
			Issuer:    config.App_Name,
			ExpiresAt: jwt.NewNumericDate(expiredAt),
			IssuedAt:  jwt.NewNumericDate(issuedAt),
		},
	}, nil
}
