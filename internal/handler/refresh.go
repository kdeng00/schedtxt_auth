package handler

import (
	"net/http"

	"git.kundeng.us/phoenix/textsender-models/pkg/token"

	"git.kundeng.us/phoenix/textsender-auth/internal/config"
	"git.kundeng.us/phoenix/textsender-auth/internal/model"
	"git.kundeng.us/phoenix/textsender-auth/internal/store"
	"git.kundeng.us/phoenix/textsender-auth/internal/utility"
)

type RefreshHandler struct {
	UserStore    model.UserStore
	ServiceStore store.ServiceStore
}

func NewRefreshHandler(userStore model.UserStore, serviceStore store.ServiceStore) *RefreshHandler {
	return &RefreshHandler{UserStore: userStore, ServiceStore: serviceStore}
}

type RefreshRequest struct {
	AccessToken string `json:"access_token"`
}

type RefreshResponse struct {
	Message string         `json:"message"`
	Data    []*token.Login `json:"data"`
}

// Refresh godoc
// @Summary      Obtain a refresh token
// @Description  Refresh token endpoint (requires JWT)
// @Tags         refresh
// @Accept       json
// @Produce      json
// @Security     BearerAuth
// @Param        request body      RefreshRequest true  "Data to refresh token"
// @Success      200  {object}  RefreshResponse
// @Failure      400  {object}  RefreshResponse
// @Failure      500  {object}  RefreshResponse
// @Router       /token/refresh [post]
func (rh *RefreshHandler) Refresh(w http.ResponseWriter, r *http.Request) {
	var req RefreshRequest
	if err := ExtractFromRequest(r, &req); err != nil {
		http.Error(w, "Invalid JSON: "+err.Error(), http.StatusBadRequest)
	}
	defer r.Body.Close()

	var statusCode int
	var resp RefreshResponse

	secretKey := config.GetSecretKey()
	tokGen := utility.TokenGenerator{}
	tokGen.SetSecretKey(secretKey)
	tokGen.SetHourOffset(12)
	if verified, err := tokGen.VerifyToken(req.AccessToken); err != nil {
		statusCode = http.StatusInternalServerError
		resp.Message = err.Error()
	} else {
		if verified {
			if id, err := tokGen.ExtractIdFromToken(req.AccessToken); err != nil {
				statusCode = http.StatusInternalServerError
				resp.Message = err.Error()
			} else {
				ctx := r.Context()
				if usr, err := rh.UserStore.GetUserByID(ctx, id); err != nil || usr == nil {
					if serviceUsr, err := rh.ServiceStore.GetWithId(ctx, id); err != nil || serviceUsr == nil {
						statusCode = http.StatusInternalServerError
						resp.Message = err.Error()
					} else {
						if myToken, err := tokGen.GenerateToken(serviceUsr); err != nil {
							statusCode = http.StatusInternalServerError
							resp.Message = err.Error()
						} else {
							statusCode = http.StatusOK
							resp.Data = append(resp.Data, myToken)
							resp.Message = "Successful"
						}
					}
				} else {
					if myToken, err := tokGen.GenerateToken(usr); err != nil {
						statusCode = http.StatusInternalServerError
						resp.Message = err.Error()
					} else {
						statusCode = http.StatusOK
						resp.Data = append(resp.Data, myToken)
						resp.Message = "Successful"
					}
				}
			}
		} else {
			statusCode = http.StatusBadRequest
			resp.Message = "Unverified"
		}
	}

	RespondWithJson(w, statusCode, &resp)
}
