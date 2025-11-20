package handler

import (
	"net/http"

	"git.kundeng.us/phoenix/textsender-models/pkg/user"

	"git.kundeng.us/phoenix/textsender-auth/internal/store"
	"git.kundeng.us/phoenix/textsender-auth/internal/utility"
)

type ServiceCreationRequest struct {
	Username   string `json:"username"`
	Passphrase string `json:"passphrase"`
}

type ServiceCreationResponse struct {
	Message string              `json:"message"`
	Data    []*user.ServiceUser `json:"data"`
}

type ServiceHandler struct {
	ServiceStore store.ServiceStore
}

func NewServiceHandler(serviceStore store.ServiceStore) *ServiceHandler {
	return &ServiceHandler{ServiceStore: serviceStore}
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
