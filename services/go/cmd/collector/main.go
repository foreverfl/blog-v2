package main

import (
	"fmt"
	"os"

	"blog-go-api/cmd/collector/github"
)

// main dispatches to the subcommand named in os.Args[1] and exits 1 on any error.
func main() {
	if len(os.Args) < 2 {
		fmt.Fprintln(os.Stderr, "usage: collector <github> [args...]")
		os.Exit(1)
	}

	var err error
	switch os.Args[1] {
	case "github":
		err = github.Run(os.Args[2:])
	default:
		err = fmt.Errorf("unknown subcommand %q", os.Args[1])
	}

	if err != nil {
		fmt.Fprintln(os.Stderr, "collector:", err)
		os.Exit(1)
	}
}
