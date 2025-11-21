package handler

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"git.kundeng.us/phoenix/textsender-models/pkg/user"
	"github.com/stretchr/testify/assert"

	"git.kundeng.us/phoenix/textsender-auth/internal/handler/endpoint"
	"git.kundeng.us/phoenix/textsender-auth/internal/store/mock"
	"git.kundeng.us/phoenix/textsender-auth/internal/utility"
)

func TestCreateServiceUserWithMock(t *testing.T) {
	mockStore := mock.NewMockServiceUserStore()
	handler := NewServiceHandler(mockStore)

	testService := ServiceCreationRequest{Username: "swoon", Passphrase: "ewrewr329n12y3x2!2"}
	jsonValue, err := json.Marshal(testService)
	assert.NoError(t, err, "Error marshaling request")

	req, _ := http.NewRequest("POST", endpoint.CreateServiceUser, strings.NewReader(string(jsonValue)))
	rr := httptest.NewRecorder()

	handler.Register(rr, req)
	assert.Equal(t, http.StatusCreated, rr.Code)

	var response ServiceCreationResponse
	err = json.Unmarshal(rr.Body.Bytes(), &response)
	assert.NoError(t, err)
}

func TestLoginServiceUserWithMock(t *testing.T) {
	var serviceUser user.ServiceUser
	var hashedPassword string
	var err error
	unhashed := "9328nr29nudx3292m320!"
	hashing := utility.HashMash{Password: unhashed}
	if hashedPassword, err = hashing.HashPassword(); err != nil {
		assert.NoError(t, err, "Error hashing password: %v", err)
	} else {
		serviceUser.Passphrase = hashedPassword
	}
	serviceUser.Username = "swoon"
	ctx := t.Context()
	mockStore := mock.NewMockServiceUserStore()

	if err := mockStore.Create(ctx, &serviceUser); err != nil {
		assert.NoError(t, err, "Error creating service user: %v", err)
	}

	handler := NewServiceHandler(mockStore)
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
}
