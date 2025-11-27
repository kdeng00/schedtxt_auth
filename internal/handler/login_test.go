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
	"git.kundeng.us/phoenix/textsender-auth/internal/utility"
)

func TestLogin(t *testing.T) {
	cfg := GetConfig()
	mockstore := mock.NewMockUserStore()
	handler := NewLoginHandler(cfg, mockstore)

	testUser := GetTestUser()
	unhashedPassword := testUser.Password
	hashing := utility.HashMash{Password: testUser.Password}
	hashedPassword, err := hashing.HashPassword()
	assert.NoError(t, err)

	testUser.Password = hashedPassword

	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	mockstore.CreateUser(ctx, &testUser)

	loginUser := LoginAccount{Username: testUser.Username, Password: unhashedPassword}
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
