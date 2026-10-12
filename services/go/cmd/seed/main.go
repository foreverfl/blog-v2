package main

import (
	"fmt"
	"os"

	"blog-go-api/cmd/seed/apinotes"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Fprintln(os.Stderr, "usage: go run ./cmd/seed <table> [flags]\n  apinotes   turn a package<TAB>name list on stdin into INSERTs for code.api_notes")
		os.Exit(2)
	}
	var err error
	switch os.Args[1] {
	case "apinotes":
		err = apinotes.Seed(os.Args[2:])
	default:
		err = fmt.Errorf("unknown table %q", os.Args[1])
	}
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
