package common

import (
	"encoding/json"
	"net/http"
)

func WriteJSON(writer http.ResponseWriter, status int, value any) {
	writer.Header().Set("Content-Type", "application/json")
	writer.WriteHeader(status)
	json.NewEncoder(writer).Encode(value)
}

// CheckAuth returns true if the bearer token matches the API key.
func CheckAuth(request *http.Request, apiKey string) bool {
	return request.Header.Get("Authorization") == "Bearer "+apiKey
}
