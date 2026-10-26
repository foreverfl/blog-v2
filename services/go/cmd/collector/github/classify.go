package github

import (
	"bufio"
	"encoding/json"
	"errors"
	"fmt"
	"io/fs"
	"os"
	"strconv"
)

const kindsPath = "../../data/github_repo_kinds.jsonl"

// RunClassify prints the full names of up to args[0] repos from the overall ranking, most stars first, that the kinds file does not list yet; it returns an error for a missing or non-numeric count, an unreadable file or a failed search.
func RunClassify(args []string) error {
	if len(args) != 1 {
		return errors.New("usage: collector github-classify <n>")
	}
	limit, err := strconv.Atoi(args[0])
	if err != nil {
		return err
	}

	classifiedNames, err := loadClassifiedNames(kindsPath)
	if err != nil {
		return err
	}
	overallRepos, err := collectUnit(units[0])
	if err != nil {
		return err
	}

	printed := 0
	for _, overallRepo := range overallRepos {
		if printed == limit {
			break
		}
		if classifiedNames[overallRepo.FullName] {
			continue
		}
		fmt.Println(overallRepo.FullName)
		printed++
	}
	return nil
}

// loadClassifiedNames returns the set of full_name values in the jsonl file at path, an empty set when the file does not exist, or an error for a line that is not JSON.
func loadClassifiedNames(path string) (map[string]bool, error) {
	classifiedNames := map[string]bool{}
	file, err := os.Open(path)
	if errors.Is(err, fs.ErrNotExist) {
		return classifiedNames, nil
	}
	if err != nil {
		return nil, err
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		var line struct {
			FullName string `json:"full_name"`
		}
		if err := json.Unmarshal(scanner.Bytes(), &line); err != nil {
			return nil, fmt.Errorf("%s: %w", path, err)
		}
		classifiedNames[line.FullName] = true
	}
	return classifiedNames, scanner.Err()
}
