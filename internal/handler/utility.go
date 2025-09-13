package handler

import (
	"encoding/json"
	"net/http"
)

func ExtractFromRequest(r *http.Request, reqItem interface{}) error {
	err := json.NewDecoder(r.Body).Decode(&reqItem)
	if err != nil {
		return err
	} else {
		return nil
	}
}

func RespondWithJson(w http.ResponseWriter, statusCode int, data interface{}) {
	w.Header().Set("Content-type", "application/json")
	w.WriteHeader(statusCode)
	json.NewEncoder(w).Encode(data)
}
