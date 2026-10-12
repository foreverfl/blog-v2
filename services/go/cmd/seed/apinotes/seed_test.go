package apinotes

import "testing"

func TestInsertStatementFillsThePattern(t *testing.T) {
	got, err := insertStatement("php", "https://www.php.net/manual/function.{dashed-name}.php", "standard\tarray_filter")
	if err != nil {
		t.Fatal(err)
	}
	want := "INSERT INTO code.api_notes (lang, package, name, doc_url) VALUES ('php', 'standard', 'array_filter', 'https://www.php.net/manual/function.array-filter.php') ON CONFLICT DO NOTHING;"
	if got != want {
		t.Fatalf("got\n%s\nwant\n%s", got, want)
	}
}

func TestInsertStatementTakesTheUrlFromTheThirdColumn(t *testing.T) {
	got, err := insertStatement("mysql", "unused", "String Functions\tGROUP_CONCAT\thttps://dev.mysql.com/doc/refman/8.0/en/aggregate-functions.html")
	if err != nil {
		t.Fatal(err)
	}
	want := "INSERT INTO code.api_notes (lang, package, name, doc_url) VALUES ('mysql', 'String Functions', 'GROUP_CONCAT', 'https://dev.mysql.com/doc/refman/8.0/en/aggregate-functions.html') ON CONFLICT DO NOTHING;"
	if got != want {
		t.Fatalf("got\n%s\nwant\n%s", got, want)
	}
}

func TestQuoteDoublesASingleQuote(t *testing.T) {
	if got := quote("it's"); got != "'it''s'" {
		t.Fatalf("got %s", got)
	}
}
