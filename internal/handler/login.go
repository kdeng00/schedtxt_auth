package handler

import (
	"fmt"
	"net/http"

	"git.kundeng.us/phoenix/textsender-models/pkg/token"

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
	Data    []token.Login `json:"data"`
}

type LoginHandler struct {
	UserStore model.UserStore
}

func NewLoginHandler(userStore model.UserStore) *LoginHandler {
	return &LoginHandler{UserStore: userStore}
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
					if myToken, err := tokGen.GenerateToken(*user); err != nil {
						fmt.Println(err.Error())
						statusCode = http.StatusInternalServerError
						resp.Message = "Error generating token"
					} else {
						statusCode = http.StatusOK
						resp.Data = append(resp.Data, *myToken)
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
