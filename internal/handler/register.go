package handler

import (
	"fmt"
	"net/http"

	"git.kundeng.us/phoenix/textsender-models/pkg/user"
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

// Register godoc
// @Summary      Register user
// @Description  Create a user that can send texts (requires JWT)
// @Tags         users
// @Accept       json
// @Produce      json
// @Security     BearerAuth
// @Param        request body      RegisterUser true  "Data to add user"
// @Success      200  {object}  RegisterResponse
// @Failure      400  {object}  RegisterResponse
// @Failure      500  {object}  RegisterResponse
// @Router       /register [post]
func (u *UserHandler) Register(w http.ResponseWriter, r *http.Request) {
	var req RegisterUser
	err := ExtractFromRequest(r, &req)
	if err != nil {
		http.Error(w, "Invalid JSON: "+err.Error(), http.StatusBadRequest)
		return
	}

	defer r.Body.Close()

	var statusCode int
	var resp RegisterResponse
	user := user.User{Username: req.Username, Password: req.Password, PhoneNumber: req.PhoneNumber}

	fmt.Println("Username:", user.Username)

	ctx := r.Context()

	if exists, err := u.UserStore.UserExists(ctx, user.Username); err != nil {
		fmt.Printf("Error: %v", err)
		statusCode = http.StatusInternalServerError
		resp.Message = err.Error()
	} else {
		if exists {
			// User already exists
			statusCode = http.StatusBadRequest
			resp.Message = "Failure in creating User"
		} else {
			hashing := utility.HashMash{Password: user.Password}
			if hashedPassword, err := hashing.HashPassword(); err != nil {
				statusCode = http.StatusInternalServerError
				resp.Message = err.Error()
			} else {
				user.Password = hashedPassword
				err := u.UserStore.CreateUser(ctx, &user)
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

	}

	RespondWithJson(w, statusCode, &resp)
}
