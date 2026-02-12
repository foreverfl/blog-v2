package main

import (
	"fmt"
	"net/http"
)

func health(w http.ResponseWriter, r *http.Request) {
	fmt.Fprint(w, "ok")
}

func main() {
	http.HandleFunc("/health", health)
	fmt.Println("go-api listening on :8002")
	http.ListenAndServe(":8002", nil)
}