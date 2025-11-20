package handler

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"

	"git.kundeng.us/phoenix/textsender-auth/internal/handler/endpoint"
	"git.kundeng.us/phoenix/textsender-auth/internal/store/mock"
)

func TestCreateUserWithMock(t *testing.T) {
	mockstore := mock.NewMockUserStore()
	handler := NewUserHandler(mockstore)

	testUser := GetTestUser()
	jsonValue, _ := json.Marshal(testUser)

	req, _ := http.NewRequest("POST", endpoint.Register, strings.NewReader(string(jsonValue)))
	rr := httptest.NewRecorder()

	handler.Register(rr, req)

	assert.Equal(t, http.StatusOK, rr.Code)

	var response RegisterResponse
	err := json.Unmarshal(rr.Body.Bytes(), &response)
	assert.NoError(t, err)

	assert.NotNil(t, response.Data[0].Id, "Id should not be nil")
}
