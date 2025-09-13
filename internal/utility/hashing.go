package utility

import (
	"golang.org/x/crypto/bcrypt"
)

type HashMash struct {
	Password string
}

func (h HashMash) HashPassword() (string, error) {
	bytes, err := bcrypt.GenerateFromPassword([]byte(h.Password), bcrypt.DefaultCost)
	return string(bytes), err
}

func (h HashMash) CheckPasswordHash(password string, hash string) bool {
	err := bcrypt.CompareHashAndPassword([]byte(hash), []byte(password))
	return err == nil
}

func (h *HashMash) SetPassword(password string) {
	h.Password = password
}
