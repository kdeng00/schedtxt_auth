package handler

import (
	"fmt"
	"github.com/google/uuid"
	"net/http"

	"git.kundeng.us/phoenix/textsender-auth/internal/model"
	"git.kundeng.us/phoenix/textsender-auth/internal/utility"
	"git.kundeng.us/phoenix/textsender-models/pkg/user"
)

type RegisterUser struct {
	PhoneNumber string `json:"phone_number"`
	Username    string `json:"username"`
	Password    string `json:"password"`
}

type RegisterResponseItem struct {
	Id          uuid.UUID `json:"id"`
	PhoneNumber string    `json:"phone_number"`
	Username    string    `json:"username"`
}

type RegisterResponse struct {
	Message string                 `json:"message"`
	Data    []RegisterResponseItem `json:"data"`
}

type UserHandler struct {
	UserStore model.UserStore
}

func NewUserHandler(userStore model.UserStore) *UserHandler {
	return &UserHandler{UserStore: userStore}
}

func (h *UserHandler) Register(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req RegisterUser
	err := ExtractFromRequest(r, &req)
	if err != nil {
		http.Error(w, "Invalid JSON: "+err.Error(), http.StatusBadRequest)
		return
	}

	defer r.Body.Close()

	user := user.User{Username: req.Username, Password: req.Password, PhoneNumber: req.PhoneNumber}

	var statusCode int
	resp := RegisterResponse{}

	fmt.Println("Username:", user.Username)

	ctx := r.Context()

	exists, err := h.UserStore.UserExists(ctx, user.Username)
	if err != nil {
		fmt.Printf("Error: %v", err)
		statusCode = http.StatusInternalServerError
		resp.Message = err.Error()
	}

	if exists {
		// User already exists
		statusCode = http.StatusBadRequest
		resp.Message = "Failure in creating User"
	} else {
		hashing := utility.HashMash{Password: user.Password}
		hashedPassword, err := hashing.HashPassword()
		if err != nil {
			statusCode = http.StatusInternalServerError
			resp.Message = err.Error()
		} else {
			user.Password = hashedPassword
			err := h.UserStore.CreateUser(ctx, &user)
			if err != nil {
				statusCode = http.StatusInternalServerError
				resp.Message = err.Error()
			} else {
				resp.Message = "Successful"
				statusCode = http.StatusOK
				resp.Data = append(resp.Data, RegisterResponseItem{Id: user.Id, PhoneNumber: user.PhoneNumber, Username: user.Username})
			}
		}
	}

	RespondWithJson(w, statusCode, &resp)
}
