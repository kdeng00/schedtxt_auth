package handler

import (
	"log"
	"net/http"
	"time"

	"git.kundeng.us/phoenix/textsender-models/tx0/token"
	"git.kundeng.us/phoenix/textsender-models/tx0/user"
	"github.com/google/uuid"

	"git.kundeng.us/phoenix/textsender-auth/internal/config"
	"git.kundeng.us/phoenix/textsender-auth/internal/store"
	"git.kundeng.us/phoenix/textsender-auth/internal/utility"
)

type LoginHandler struct {
	Config    *config.Config
	UserStore store.UserStore
}

func NewLoginHandler(cfg *config.Config, userStore store.UserStore) *LoginHandler {
	return &LoginHandler{Config: cfg, UserStore: userStore}
}

type LoginAccount struct {
	Username string `json:"username"`
	Password string `json:"password"`
}

type LoginResponse struct {
	Message string        `json:"message"`
	Data    []token.Login `json:"data"`
}

// Login godoc
// @Summary      Login
// @Description  Login and be given an access token (requires JWT)
// @Tags         users
// @Accept       json
// @Produce      json
// @Security     BearerAuth
// @Param        request body      LoginAccount true  "Data to obtain a token"
// @Success      200  {object}  LoginResponse
// @Failure      400  {object}  LoginResponse
// @Failure      500  {object}  LoginResponse
// @Router       /login [post]
func (l *LoginHandler) Login(w http.ResponseWriter, r *http.Request) {
	var req LoginAccount
	if err := ExtractFromRequest(r, &req); err != nil {
		http.Error(w, "Invalid JSON: "+err.Error(), http.StatusBadRequest)
	}
	defer r.Body.Close()

	var statusCode int
	var resp LoginResponse

	ctx := r.Context()

	if exists, err := l.UserStore.UserExists(ctx, req.Username); err != nil {
		log.Println("Error:", err)
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
				hashing := utility.HashMash{}
				if err := hashing.SetPassword(req.Password); err != nil {
					statusCode = http.StatusInternalServerError
					resp.Message = err.Error()
				} else {
					if hashing.CheckPasswordHash(req.Password, user.Password) {
						lastLogin := time.Now()
						var tokGen utility.TokenGenerator
						secretKey := config.GetSecretKey()
						tokGen.SetSecretKey(secretKey)
						if myToken, err := tokGen.GenerateToken(*user); err != nil {
							log.Println(err.Error())
							statusCode = http.StatusInternalServerError
							resp.Message = "Error generating token"
						} else {
							log.Println("Updating user's last login")
							if rowsAffected, err := l.UserStore.UpdateLastLogin(ctx, user.Id, lastLogin); err != nil {
								statusCode = http.StatusInternalServerError
								resp.Message = err.Error()
							} else {
								log.Println("Rows updated:", rowsAffected)
								statusCode = http.StatusOK
								resp.Data = append(resp.Data, *myToken)
								resp.Message = "Successful"
							}
						}
					} else {
						statusCode = http.StatusNotFound
						resp.Message = "User not found"
					}
				}
			}
		}
	}

	RespondWithJson(w, statusCode, &resp)
}

type UpdatePasswordRequest struct {
	UserId            uuid.UUID `json:"user_id"`
	CurrentPassword   string    `json:"current_password"`
	UpdatedPassword   string    `json:"updated_password"`
	ConfirmedPassword string    `json:"confirmed_password"`
}

type UpdatePasswordResponse struct {
	Message string      `json:"message"`
	Data    []user.User `json:"data"`
}

// UpdatePassword godoc
// @Summary      Update Password
// @Description  Update the password of a regular account (requires JWT)
// @Tags         users
// @Accept       json
// @Produce      json
// @Security     BearerAuth
// @Param        request body      UpdatePasswordRequest true  "Needed data to update password"
// @Success      200  {object}  UpdatePasswordResponse
// @Failure      400  {object}  UpdatePasswordResponse
// @Failure      500  {object}  UpdatePasswordResponse
// @Router       /user/password/update [patch]
func (l *LoginHandler) UpdatePassword(w http.ResponseWriter, r *http.Request) {
	var req UpdatePasswordRequest
	if err := ExtractFromRequest(r, &req); err != nil {
		http.Error(w, "Invalid JSON: "+err.Error(), http.StatusBadRequest)
	}
	defer r.Body.Close()

	var statusCode int
	var resp UpdatePasswordResponse

	ctx := r.Context()

	if usr, err := l.UserStore.GetUserByID(ctx, req.UserId); err != nil {
		log.Println("Error:", err)
		statusCode = http.StatusInternalServerError
		resp.Message = err.Error()
	} else {
		hashing := utility.HashMash{}
		if err := hashing.SetPassword(req.CurrentPassword); err != nil {
			statusCode = http.StatusInternalServerError
			resp.Message = err.Error()
		} else {
			if hashing.CheckPasswordHash(req.CurrentPassword, usr.Password) {
				if req.UpdatedPassword == req.ConfirmedPassword {
					// Hash password
					err := hashing.SetPassword(req.UpdatedPassword)
					hashedPassword, err := hashing.HashPassword()
					if err != nil {
						statusCode = http.StatusInternalServerError
						resp.Message = err.Error()
					} else {
						// Update user password
						usr.Password = hashedPassword
						// Save user in DB
						if rowsAffected, err := l.UserStore.UpdatePassword(ctx, usr.Id, usr.Password); err != nil {
							statusCode = http.StatusInternalServerError
							resp.Message = err.Error()
						} else {
							log.Println("Rows affected:", rowsAffected)
							statusCode = http.StatusOK
							resp.Message = "Successful"
							resp.Data = append(resp.Data, *usr)
						}
					}
				} else {
					statusCode = http.StatusBadRequest
					resp.Message = "Passwords do not match"
				}
			} else {
				statusCode = http.StatusBadRequest
				resp.Message = "User not found"
			}
		}
	}

	RespondWithJson(w, statusCode, &resp)
}
