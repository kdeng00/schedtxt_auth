package handler

import "net/http"
import "encoding/json"
import "fmt"

import (
	"github.com/google/uuid"

	"git.kundeng.us/phoenix/textsender-auth/internal/model"
	"git.kundeng.us/phoenix/textsender-auth/internal/utility"
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

	user, err := extractUserFromReq(r)
	if err != nil {
		http.Error(w, "Invalid JSON: "+err.Error(), http.StatusBadRequest)
		return
	}

	defer r.Body.Close()

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
		hashing := utility.HashMash{user.Password}
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

	respondWithJson(w, statusCode, &resp)
}

func extractUserFromReq(r *http.Request) (user model.User, myError error) {
	var usr RegisterUser
	err := json.NewDecoder(r.Body).Decode(&usr)
	if err != nil {
		return user, err
	}

	return model.User{PhoneNumber: usr.PhoneNumber, Username: usr.Username, Password: usr.Password}, nil
}

func respondWithJson(w http.ResponseWriter, statusCode int, data interface{}) {
	w.Header().Set("Content-type", "application/json")
	w.WriteHeader(statusCode)
	json.NewEncoder(w).Encode(data)
}
