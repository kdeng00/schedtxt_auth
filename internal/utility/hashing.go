package utility

import (
	"fmt"
	"strings"
	"unicode"

	"golang.org/x/crypto/bcrypt"
)

type HashMash struct {
	password string
}

func (h *HashMash) HashPassword() (string, error) {
	bytes, err := bcrypt.GenerateFromPassword([]byte(h.password), bcrypt.DefaultCost)
	return string(bytes), err
}

func (h *HashMash) CheckPasswordHash(password string, hash string) bool {
	err := bcrypt.CompareHashAndPassword([]byte(hash), []byte(password))
	return err == nil
}

func (h *HashMash) SetPassword(password string) error {
	if len(password) < 8 {
		return fmt.Errorf("Password length is not enought")
	} else if len(password) > 32 {
		return fmt.Errorf("Password length is too long")
	} else {
		specialCharacters := "!@#$%^&*?"
		numbers := "0123456789"
		if strings.ContainsAny(password, specialCharacters) && strings.ContainsAny(password, numbers) {
			var hasAtleastOneUpper, hasAtleastOneLower bool

			for _, c := range password {
				if unicode.IsUpper(c) {
					hasAtleastOneUpper = true
				} else if unicode.IsLower(c) {
					hasAtleastOneLower = true
				}

				if hasAtleastOneLower && hasAtleastOneUpper {
					break
				}
			}

			if hasAtleastOneUpper && hasAtleastOneLower {
				h.password = password
				return nil
			} else {
				if !hasAtleastOneUpper {
					return fmt.Errorf("Password requires at least one upper case letter")
				} else {
					return fmt.Errorf("Password requires at least one lower case letter")
				}
			}
		} else {
			return fmt.Errorf("Password should contain special characters and numbers")
		}
	}
}
