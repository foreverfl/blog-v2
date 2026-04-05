package common

// EnsureSummaryMap ensures the "summary" key exists as a map.
func EnsureSummaryMap(item map[string]any) {
	if item["summary"] == nil {
		item["summary"] = map[string]any{}
	}
}

// EnsureTitleMap ensures the "title" key exists as a map.
func EnsureTitleMap(item map[string]any) {
	if item["title"] == nil {
		item["title"] = map[string]any{}
	}
}

// IsTitleEmpty checks if a title for the given language is empty or missing.
func IsTitleEmpty(item map[string]any, lang string) bool {
	t, ok := item["title"].(map[string]any)
	if !ok || t == nil {
		return true
	}
	v, ok := t[lang]
	if !ok || v == nil {
		return true
	}
	s, ok := v.(string)
	return ok && s == ""
}

// IsEmpty checks if a top-level string field is empty or missing.
func IsEmpty(item map[string]any, field string) bool {
	v, ok := item[field]
	if !ok || v == nil {
		return true
	}
	s, ok := v.(string)
	return ok && s == ""
}

// IsSummaryEmpty checks if a summary for the given language is empty or missing.
func IsSummaryEmpty(item map[string]any, lang string) bool {
	s, ok := item["summary"].(map[string]any)
	if !ok || s == nil {
		return true
	}
	v, ok := s[lang]
	if !ok || v == nil {
		return true
	}
	str, ok := v.(string)
	return ok && str == ""
}
