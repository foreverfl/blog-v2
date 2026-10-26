package main

import (
	"fmt"
	"os"

	"blog-go-cli/internal/cmd/classify"
	"blog-go-cli/internal/cmd/snapshot"
)

// main dispatches to the subcommand named in os.Args[1] and exits 1 on any error.
func main() {
	if len(os.Args) < 2 {
		fmt.Fprintln(os.Stderr, "usage: go-cli <github-snapshot|github-classify> [args...]")
		os.Exit(1)
	}

	var err error
	switch os.Args[1] {
	case "github-snapshot":
		err = snapshot.Run(os.Args[2:])
	case "github-classify":
		err = classify.Run(os.Args[2:])
	default:
		err = fmt.Errorf("unknown subcommand %q", os.Args[1])
	}

	if err != nil {
		fmt.Fprintln(os.Stderr, "go-cli:", err)
		os.Exit(1)
	}
}
