package github

import (
	"context"
	"fmt"
	"os"
	"strings"
	"time"

	"blog-go-api/internal/db"
)

type unit struct {
	key   string
	query string
}

// query is the filter added to the stars range; empty means every repo.
var units = []unit{
	{key: "overall", query: ""},
}

const (
	rounds        = 10
	pagesPerRound = 10
	searchPause   = 2 * time.Second
)

var jst = time.FixedZone("JST", 9*60*60)

// Run collects every unit, or only the one whose key is args[0], and prints rank, full_name and stars per line; it returns an error for an unknown key or a failed request.
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
	pool, err := db.NewPool(ctx, os.Getenv("DATABASE_URL"))
	if err != nil {
		return err
	}
	defer pool.Close()

	snapshotDate := time.Now().In(jst).Format(time.DateOnly)
	for _, selected := range selectedUnits {
		unitRepos, err := collectUnit(selected)
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

// collectUnit returns up to rounds × pagesPerRound × 100 repos for the unit, most stars first and no repo twice, narrowing the stars range each round because one search only exposes its first 1000 results.
func collectUnit(selected unit) ([]repo, error) {
	var unitRepos []repo
	seenFullNames := map[string]bool{}
	upperStars := 10_000_000

	for round := 1; round <= rounds; round++ {
		query := strings.TrimSpace(fmt.Sprintf("%s stars:1000..%d", selected.query, upperStars))
		for page := 1; page <= pagesPerRound; page++ {
			pageRepos, err := searchRepos(query, page)
			if err != nil {
				return nil, err
			}
			for _, pageRepo := range pageRepos {
				if seenFullNames[pageRepo.FullName] {
					continue
				}
				seenFullNames[pageRepo.FullName] = true
				unitRepos = append(unitRepos, pageRepo)
			}
			if len(pageRepos) == 0 {
				return unitRepos, nil
			}
			time.Sleep(searchPause)
		}
		upperStars = unitRepos[len(unitRepos)-1].Stars
		fmt.Fprintf(os.Stderr, "%s round %d done, lowest stars %d, repos %d\n", selected.key, round, upperStars, len(unitRepos))
	}
	return unitRepos, nil
}
