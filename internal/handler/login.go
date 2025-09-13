package handler

import (
	"fmt"
	"net/http"

	"git.kundeng.us/phoenix/textsender-auth/internal/config"
	"git.kundeng.us/phoenix/textsender-auth/internal/model"
	"git.kundeng.us/phoenix/textsender-auth/internal/utility"
)

type LoginAccount struct {
	Username string `json:"username"`
	Password string `json:"password"`
}

type LoginResponse struct {
	Message string        `json:"message"`
	Data    []model.Login `json:"data"`
}

type LoginHandler struct {
	UserStore model.UserStore
}

func NewLoginHandler(userStore model.UserStore) *LoginHandler {
	return &LoginHandler{UserStore: userStore}
}

func (l *LoginHandler) Login(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req LoginAccount
	if err := ExtractFromRequest(r, &req); err != nil {
		http.Error(w, "Invalid JSON: "+err.Error(), http.StatusBadRequest)
	}
	defer r.Body.Close()

	var statusCode int
	var resp LoginResponse

	ctx := r.Context()

	if exists, err := l.UserStore.UserExists(ctx, req.Username); err != nil {
		fmt.Printf("Error: %v", err)
		statusCode = http.StatusInternalServerError
		resp.Message = err.Error()
	} else {
		if !exists {
			statusCode = http.StatusBadRequest
			resp.Message = "Failure in user check"
		} else {
			if user, err := l.UserStore.GetUserByUsername(ctx, req.Username); err != nil {
				statusCode = http.StatusInternalServerError
				resp.Message = err.Error()
			} else {
				hashing := utility.HashMash{Password: req.Password}
				if hashing.CheckPasswordHash(req.Password, user.Password) {
					var tokGen utility.TokenGenerator
					secretKey := config.GetSecretKey()
					tokGen.SetSecretKey(secretKey)
					if token, err := tokGen.GenerateToken(*user); err != nil {
						fmt.Println(err.Error())
						statusCode = http.StatusInternalServerError
						resp.Message = "Error generating token"
					} else {
						statusCode = http.StatusOK
						resp.Data = append(resp.Data, *token)
						resp.Message = "Successful"
					}
				} else {
					statusCode = http.StatusNotFound
					resp.Message = "User not found"
				}
			}
		}
	}

	RespondWithJson(w, statusCode, &resp)
}
