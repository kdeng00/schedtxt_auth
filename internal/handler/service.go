package handler

import (
	"net/http"

	"git.kundeng.us/phoenix/textsender-models/tx0/token"
	"git.kundeng.us/phoenix/textsender-models/tx0/user"

	"git.kundeng.us/phoenix/textsender-auth/internal/config"
	"git.kundeng.us/phoenix/textsender-auth/internal/store"
	"git.kundeng.us/phoenix/textsender-auth/internal/utility"
)

type ServiceHandler struct {
	Config       *config.Config
	ServiceStore store.ServiceStore
}

func NewServiceHandler(cfg *config.Config, serviceStore store.ServiceStore) *ServiceHandler {
	return &ServiceHandler{Config: cfg, ServiceStore: serviceStore}
}

type ServiceCreationRequest struct {
	Username   string `json:"username"`
	Passphrase string `json:"passphrase"`
}

type ServiceCreationResponse struct {
	Message string              `json:"message"`
	Data    []*user.ServiceUser `json:"data"`
}

// Register godoc
// @Summary      Register service user
// @Description  Create a service user that can send texts (requires JWT)
// @Tags         service users
// @Accept       json
// @Produce      json
// @Security     BearerAuth
// @Param        request body      ServiceCreationRequest true  "Data to add user"
// @Success      200  {object}  ServiceCreationResponse
// @Failure      400  {object}  ServiceCreationResponse
// @Failure      403  {object}  ServiceCreationResponse
// @Failure      500  {object}  ServiceCreationResponse
// @Router       /service/register [post]
func (s *ServiceHandler) Register(w http.ResponseWriter, r *http.Request) {
	var req ServiceCreationRequest
	if err := ExtractFromRequest(r, &req); err != nil {
		http.Error(w, "Invalid JSON: "+err.Error(), http.StatusBadRequest)
		return
	}

	defer r.Body.Close()

	var statusCode int
	var resp ServiceCreationResponse
	if !s.Config.EnableRegistration {
		statusCode = http.StatusForbidden
		resp.Message = "Registration disabled"
		RespondWithJson(w, statusCode, &resp)
		return
	}

	ctx := r.Context()
	if exists, err := s.ServiceStore.CheckWithUsername(ctx, req.Username); err != nil {
		statusCode = http.StatusInternalServerError
		resp.Message = err.Error()
	} else {
		if exists {
			statusCode = http.StatusBadRequest
			resp.Message = "Service user already exists"
		} else {
			hashing := utility.HashMash{Password: req.Passphrase}
			if hashedPassword, err := hashing.HashPassword(); err != nil {
				statusCode = http.StatusInternalServerError
				resp.Message = err.Error()
			} else {
				serviceUser := user.ServiceUser{Username: req.Username, Passphrase: hashedPassword}
				if err := s.ServiceStore.Create(ctx, &serviceUser); err != nil {
					statusCode = http.StatusInternalServerError
					resp.Message = err.Error()
				} else {
					statusCode = http.StatusCreated
					resp.Message = "Successful"
					resp.Data = append(resp.Data, &serviceUser)
				}
			}
		}
	}

	RespondWithJson(w, statusCode, &resp)
}

type ServiceLoginRequest struct {
	Username   string `json:"username"`
	Passphrase string `json:"passphrase"`
}

type ServiceLoginResponse struct {
	Message string         `json:"message"`
	Data    []*token.Login `json:"data"`
}

// Login godoc
// @Summary      Service login
// @Description  Servce login and be given an access token (requires JWT)
// @Tags         service users
// @Accept       json
// @Produce      json
// @Security     BearerAuth
// @Param        request body      ServiceLoginRequest true  "Data to obtain a service token"
// @Success      200  {object}  ServiceLoginResponse
// @Failure      500  {object}  ServiceLoginResponse
// @Router       /service/login [post]
func (s *ServiceHandler) Login(w http.ResponseWriter, r *http.Request) {
	var req ServiceLoginRequest
	if err := ExtractFromRequest(r, &req); err != nil {
		http.Error(w, "Invalid JSON: "+err.Error(), http.StatusBadRequest)
	}
	defer r.Body.Close()

	var statusCode int
	var resp ServiceLoginResponse

	if len(req.Username) == 0 || len(req.Passphrase) == 0 {
		statusCode = http.StatusBadRequest
		resp.Message = "Invalid request"
		RespondWithJson(w, statusCode, &resp)
		return
	}

	ctx := r.Context()

	if serviceUser, err := s.ServiceStore.GetWithUsername(ctx, req.Username); err != nil {
		statusCode = http.StatusInternalServerError
		resp.Message = err.Error()
	} else {
		if serviceUser == nil {
			statusCode = http.StatusNotFound
			resp.Message = "Not found"
		} else {
			hashing := utility.HashMash{Password: req.Passphrase}
			if !hashing.CheckPasswordHash(req.Passphrase, serviceUser.Passphrase) {
				statusCode = http.StatusInternalServerError
				resp.Message = "Not valid"
			} else {
				var tokGen utility.TokenGenerator
				tokGen.SetHourOffset(8)
				secretKey := config.GetSecretKey()
				tokGen.SetSecretKey(secretKey)

				if myToken, err := tokGen.GenerateToken(*serviceUser); err != nil {
					statusCode = http.StatusInternalServerError
					resp.Message = err.Error()
				} else {
					statusCode = http.StatusOK
					resp.Data = append(resp.Data, myToken)
					resp.Message = "Successful"
				}
			}
		}
	}

	RespondWithJson(w, statusCode, &resp)
}
