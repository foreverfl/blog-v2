package classify

import (
	"os"
	"path/filepath"
	"testing"
)

// oneLine returns one jsonl line of the kinds file for the repo fullName.
func oneLine(fullName string) string {
	return `{"full_name":"` + fullName + `"}` + "\n"
}

func TestLoadClassifiedNamesReadsEveryLine(t *testing.T) {
	kindsPath := filepath.Join(t.TempDir(), "kinds.jsonl")
	if err := os.WriteFile(kindsPath, []byte(oneLine("golang/go")+oneLine("rust-lang/rust")), 0o644); err != nil {
		t.Fatal(err)
	}

	classifiedNames, err := loadClassifiedNames(kindsPath)
	if err != nil {
		t.Fatal(err)
	}
	if len(classifiedNames) != 2 || !classifiedNames["golang/go"] || !classifiedNames["rust-lang/rust"] {
		t.Errorf("classifiedNames = %v", classifiedNames)
	}
}

func TestLoadClassifiedNamesWithoutFileIsEmpty(t *testing.T) {
	missingPath := filepath.Join(t.TempDir(), "kinds.jsonl")

	classifiedNames, err := loadClassifiedNames(missingPath)
	if err != nil {
		t.Fatal(err)
	}
	if len(classifiedNames) != 0 {
		t.Errorf("classifiedNames = %v", classifiedNames)
	}
}
