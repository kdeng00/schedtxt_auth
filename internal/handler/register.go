package handler

import (
	"fmt"
	"log"
	"net/http"

	"git.kundeng.us/phoenix/textsender-models/tx0/user"
	"github.com/google/uuid"

	"git.kundeng.us/phoenix/textsender-auth/internal/config"
	"git.kundeng.us/phoenix/textsender-auth/internal/store"
	"git.kundeng.us/phoenix/textsender-auth/internal/utility"
)

type UserHandler struct {
	Config    *config.Config
	UserStore store.UserStore
}

func NewUserHandler(cfg *config.Config, userStore store.UserStore) *UserHandler {
	return &UserHandler{Config: cfg, UserStore: userStore}
}

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
// @Failure      403  {object}  RegisterResponse
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
	if !u.Config.EnableRegistration {
		statusCode = http.StatusForbidden
		resp.Message = "Registration disabled"
		RespondWithJson(w, statusCode, &resp)
		return
	}
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
			hashing := utility.HashMash{}
			if err := hashing.SetPassword(req.Password); err != nil {
				statusCode = http.StatusInternalServerError
				resp.Message = err.Error()
			} else {
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

	}

	RespondWithJson(w, statusCode, &resp)
}

type UpdateNameRequest struct {
	Firstname *string   `json:"first_name,omitempty"`
	Lastname  *string   `json:"last_name,omitempty"`
	UserId    uuid.UUID `json:"user_id"`
}

type UpdateNameResponse struct {
	Message string       `json:"message"`
	Data    []*user.User `json:"data"`
}

// UpdateName godoc
// @Summary      Update name of user
// @Description  Update the first or last name of a user (requires JWT)
// @Tags         users
// @Accept       json
// @Produce      json
// @Security     BearerAuth
// @Param        request body      UpdateNameRequest true  "Data to update name of user"
// @Success      200  {object}  UpdateNameResponse
// @Failure      400  {object}  UpdateNameResponse
// @Failure      403  {object}  UpdateNameResponse
// @Failure      500  {object}  UpdateNameResponse
// @Router       /user/name/update [patch]
func (u *UserHandler) UpdateName(w http.ResponseWriter, r *http.Request) {
	var req UpdateNameRequest
	err := ExtractFromRequest(r, &req)
	if err != nil {
		http.Error(w, "Invalid JSON: "+err.Error(), http.StatusBadRequest)
		return
	}

	defer r.Body.Close()

	var statusCode int
	var resp UpdateNameResponse
	updateFirstname := req.Firstname != nil && len(*req.Firstname) > 0
	updateLastname := req.Lastname != nil && len(*req.Lastname) > 0

	if req.UserId == uuid.Nil {
		statusCode = http.StatusBadRequest
		resp.Message = "User Id not provided"
	} else if !updateFirstname && !updateLastname {
		statusCode = http.StatusBadRequest
		resp.Message = "No name provided"
	} else {
		ctx := r.Context()
		if usr, err := u.UserStore.GetUserByID(ctx, req.UserId); err != nil {
			log.Println("Error:", err)
			statusCode = http.StatusInternalServerError
			resp.Message = err.Error()
		} else {
			if usr == nil {
				statusCode = http.StatusNotFound
				resp.Message = "User not found"
			} else {
				// Add query to update names
				if rowsAffected, err := u.UserStore.UpdateName(ctx, req.Firstname, req.Lastname, usr); err != nil {
					statusCode = http.StatusInternalServerError
					resp.Message = err.Error()
				} else {
					log.Println("Rows updated:", rowsAffected)
					statusCode = http.StatusOK
					resp.Message = "Successful"
					resp.Data = append(resp.Data, usr)
				}
			}
		}
	}

	RespondWithJson(w, statusCode, &resp)
}
