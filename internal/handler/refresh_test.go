package handler

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"git.kundeng.us/phoenix/textsender-models/tx0/user"
	"github.com/stretchr/testify/assert"

	"git.kundeng.us/phoenix/textsender-auth/internal/handler/endpoint"
	"git.kundeng.us/phoenix/textsender-auth/internal/store/mock"
	"git.kundeng.us/phoenix/textsender-auth/internal/utility"
)

func TestRefreshTokenWithMock(t *testing.T) {
	var serviceUser user.ServiceUser
	var hashedPassword string
	var err error
	unhashed := "A9328nr29nudx3292m320!"
	hashing := utility.HashMash{}
	if err := hashing.SetPassword(unhashed); err != nil {
		assert.NoError(t, err, "Error setting password")
	}

	if hashedPassword, err = hashing.HashPassword(); err != nil {
		assert.NoError(t, err, "Error hashing password: %v", err)
	} else {
		serviceUser.Passphrase = hashedPassword
	}
	serviceUser.Username = "swoon"
	ctx := t.Context()
	mockStore := mock.NewMockServiceUserStore()
	userStore := mock.NewMockUserStore()

	if err := mockStore.Create(ctx, &serviceUser); err != nil {
		assert.NoError(t, err, "Error creating service user: %v", err)
	}

	cfg := GetConfig()
	handler := NewServiceHandler(cfg, mockStore)
	testService := ServiceLoginRequest{Username: serviceUser.Username, Passphrase: unhashed}
	jsonValue, err := json.Marshal(testService)
	assert.NoError(t, err, "Error marshaling request")

	req, _ := http.NewRequest("POST", endpoint.LoginServiceUser, strings.NewReader(string(jsonValue)))
	rr := httptest.NewRecorder()

	handler.Login(rr, req)
	assert.Equal(t, http.StatusOK, rr.Code)

	var response ServiceLoginResponse
	err = json.Unmarshal(rr.Body.Bytes(), &response)
	assert.NoError(t, err)

	accessToken := response.Data[0].AccessToken

	testReq := RefreshRequest{AccessToken: accessToken}
	jsonValue, err = json.Marshal(testReq)
	assert.NoError(t, err, "Error marshaling request")

	newHandler := NewRefreshHandler(cfg, userStore, mockStore)
	req, _ = http.NewRequest("POST", endpoint.TokenRefresh, strings.NewReader(string(jsonValue)))
	rr = httptest.NewRecorder()

	newHandler.Refresh(rr, req)
	assert.Equal(t, http.StatusOK, rr.Code)
}
