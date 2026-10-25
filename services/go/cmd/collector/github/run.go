package github

import "fmt"

type unit struct {
	key   string
	query string
}

var units = []unit{
	{key: "overall", query: "stars:>1000"},
	{key: "claude-code", query: "topic:claude-code"},
	{key: "mcp-server", query: "topic:mcp-server"},
	{key: "observability", query: "topic:observability"},
}

// Run prints every search unit, or only the one whose key is args[0]; it returns an error for an unknown key.
func Run(args []string) error {
	selectedUnits := units
	if len(args) > 0 {
		selectedUnits = nil
		for _, candidate := range units {
			if candidate.key == args[0] {
				selectedUnits = append(selectedUnits, candidate)
			}
		}
		if len(selectedUnits) == 0 {
			return fmt.Errorf("unknown unit %q", args[0])
		}
	}

	for _, selected := range selectedUnits {
		fmt.Printf("%s\t%s\n", selected.key, selected.query)
	}
	return nil
}
