package snapshot

import (
	"context"
	"fmt"
	"os"
	"time"

	"github.com/jackc/pgx/v5/pgxpool"

	"blog-go-cli/internal/client/github"
)

type unit struct {
	key    string
	filter string
}

// filter is added to the stars range of the search; empty means every repo.
var units = []unit{
	{key: "overall", filter: ""},
}

var jst = time.FixedZone("JST", 9*60*60)

// Run collects every unit, or only the one whose key is args[0], and saves each as today's snapshot; it returns an error for an unknown key, an unreachable DB or a failed request.
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

	ctx := context.Background()
	pool, err := pgxpool.New(ctx, os.Getenv("DATABASE_URL"))
	if err != nil {
		return err
	}
	defer pool.Close()
	if err := pool.Ping(ctx); err != nil {
		return err
	}

	snapshotDate := time.Now().In(jst).Format(time.DateOnly)
	for _, selected := range selectedUnits {
		unitRepos, err := github.CollectTopRepos(selected.filter)
		if err != nil {
			return err
		}
		if err := saveSnapshot(ctx, pool, snapshotDate, selected.key, unitRepos); err != nil {
			return err
		}
		fmt.Printf("%s %s: saved %d repos\n", snapshotDate, selected.key, len(unitRepos))
	}
	return nil
}
