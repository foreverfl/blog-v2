package apinotes

import (
	"bufio"
	"flag"
	"fmt"
	"os"
	"strings"
)

// Seed reads "package<TAB>name[<TAB>url]" lines from stdin and prints one INSERT per line for code.api_notes, as a flyway repeatable migration; returns an error on a bad flag or a line with fewer than two columns.
func Seed(args []string) error {
	flags := flag.NewFlagSet("apinotes", flag.ContinueOnError)
	lang := flags.String("lang", "", "value for the lang column, e.g. php")
	urlPattern := flags.String("url", "", "doc_url pattern; {package}, {name} and {dashed-name} are filled in per row")
	if err := flags.Parse(args); err != nil {
		return err
	}
	if *lang == "" || *urlPattern == "" {
		return fmt.Errorf("usage: go run ./cmd/seed apinotes -lang php -url 'https://www.php.net/manual/function.{dashed-name}.php' < ../../infra/db/seeds/api_notes/php/builtin.tsv > ../../infra/db/migrations/R__seed_api_notes_php_builtin.sql")
	}

	fmt.Printf("-- %s index rows for code.api_notes; regenerate with: go run ./cmd/seed apinotes -lang %s -url '%s' < ../../infra/db/seeds/api_notes/%s/<source>.tsv\n", *lang, *lang, *urlPattern, *lang)
	rows := 0
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		statement, err := insertStatement(*lang, *urlPattern, line)
		if err != nil {
			return err
		}
		fmt.Println(statement)
		rows++
	}
	if err := scanner.Err(); err != nil {
		return err
	}
	fmt.Fprintf(os.Stderr, "%d rows\n", rows)
	return nil
}

// insertStatement turns one "package<TAB>name[<TAB>url]" line into an INSERT for code.api_notes, or an error when the line has fewer than two columns.
func insertStatement(lang, urlPattern, line string) (string, error) {
	fields := strings.Split(line, "\t")
	if len(fields) < 2 {
		return "", fmt.Errorf("bad line: %q", line)
	}
	packageName, name := fields[0], fields[1]
	url := strings.NewReplacer(
		"{package}", packageName,
		"{name}", name,
		"{dashed-name}", strings.ReplaceAll(name, "_", "-"),
	).Replace(urlPattern)
	if len(fields) >= 3 && fields[2] != "" {
		url = fields[2]
	}
	return fmt.Sprintf(
		"INSERT INTO code.api_notes (lang, package, name, doc_url) VALUES (%s, %s, %s, %s) ON CONFLICT DO NOTHING;",
		quote(lang), quote(packageName), quote(name), quote(url),
	), nil
}

// quote wraps value as a SQL string literal, doubling any single quote inside it.
func quote(value string) string {
	return "'" + strings.ReplaceAll(value, "'", "''") + "'"
}
