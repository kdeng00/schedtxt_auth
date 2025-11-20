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
