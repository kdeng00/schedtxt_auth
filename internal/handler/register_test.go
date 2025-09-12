package handler

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"git.kundeng.us/phoenix/textsender-auth/internal/db"
	"git.kundeng.us/phoenix/textsender-auth/internal/handler/endpoint"
	"git.kundeng.us/phoenix/textsender-auth/internal/model"

	"github.com/stretchr/testify/assert"
)

func TestCreateUserWithMock(t *testing.T) {
	mockstore := NewMockUserStore()
	handler := NewUserHandler(mockstore)

	testUser := model.User{Username: "ghost", PhoneNumber: "+1234567890", Password: "dfgdffddfd"}
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

func resetTestDB(t *testing.T) {
	t.Helper()
	_, err := db.Pool.Exec(context.Background(), "DELETE FROM users")
	if err != nil {
		t.Fatalf("Failed to reset test database: %v", err)
	}
}
