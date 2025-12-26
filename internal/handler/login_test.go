package handler

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"

	"git.kundeng.us/phoenix/textsender-auth/internal/handler/endpoint"
	"git.kundeng.us/phoenix/textsender-auth/internal/store/mock"
)

func TestLogin(t *testing.T) {
	cfg := GetConfig()
	mockstore := mock.NewMockUserStore()
	handler := NewLoginHandler(cfg, mockstore)

	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	testUser, unhashedPassword, err := createUser(ctx, mockstore)
	assert.NoError(t, err, "Error Creating user")

	loginUser := LoginAccount{Username: testUser.Username, Password: *unhashedPassword}
	jsonValue, _ := json.Marshal(loginUser)

	req, _ := http.NewRequest("POST", endpoint.Login, strings.NewReader(string(jsonValue)))
	rr := httptest.NewRecorder()

	handler.Login(rr, req)

	assert.Equal(t, http.StatusOK, rr.Code)

	var response LoginResponse
	err = json.Unmarshal(rr.Body.Bytes(), &response)
	assert.NoError(t, err)

	assert.NotEmpty(t, response.Data, "An access token should have been returned")
}

func TestUpdatePassword(t *testing.T) {
	cfg := GetConfig()
	mockstore := mock.NewMockUserStore()
	handler := NewLoginHandler(cfg, mockstore)
	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	testUser, unhashedPassword, err := createUser(ctx, mockstore)
	assert.NoError(t, err, "Error Creating user")
	assert.NotNil(t, testUser, "User should not be nil")

	updatedPassword := "TakeATrip2yonder!"
	newPassword := UpdatePasswordRequest{UserId: testUser.Id, CurrentPassword: *unhashedPassword, UpdatedPassword: updatedPassword, ConfirmedPassword: updatedPassword}
	jsonValue, _ := json.Marshal(newPassword)

	req, _ := http.NewRequest("PATCH", endpoint.UpdatePassword, strings.NewReader(string(jsonValue)))
	rr := httptest.NewRecorder()

	handler.UpdatePassword(rr, req)
	assert.Equal(t, http.StatusOK, rr.Code)

	var response UpdatePasswordResponse
	err = json.Unmarshal(rr.Body.Bytes(), &response)
	assert.NoError(t, err)
}
