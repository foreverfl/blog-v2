package service

import (
	"slices"
	"strconv"
	"strings"

	"blog-go-api/internal/ur/model"
)

// Conditions selects rooms; a zero/empty field skips that check.
type Conditions struct {
	MaxRentYen    int      // rent + common fee, inclusive
	MinFloorSpace float64  // ㎡, inclusive
	Layouts       []string // accepted 間取り values, exact match
}

// DefaultConditions mirrors the values in find-accomodation.md.
var DefaultConditions = Conditions{
	MaxRentYen:    120_000,
	MinFloorSpace: 40,
	Layouts:       []string{"1LDK", "2DK", "2LDK"},
}

// Filter returns the rooms passing every condition. A value that fails to
// parse passes its check — over-filtering at collection is worse than noise.
func Filter(rooms []model.Room, conditions Conditions) []model.Room {
	var passed []model.Room
	for _, room := range rooms {
		if roomPasses(room, conditions) {
			passed = append(passed, room)
		}
	}
	return passed
}

func roomPasses(room model.Room, conditions Conditions) bool {
	if conditions.MaxRentYen > 0 {
		// The API fills exactly one of rent (discounted) / rent_normal (regular).
		rent, err := parseYen(firstNonEmpty(room.Rent, room.RentNormal))
		if err == nil {
			commonFee, feeErr := parseYen(room.CommonFee)
			if feeErr != nil {
				commonFee = 0 // missing fee: judge on rent alone
			}
			if rent+commonFee > conditions.MaxRentYen {
				return false
			}
		}
	}
	if conditions.MinFloorSpace > 0 {
		space, err := parseFloorSpace(room.FloorSpace)
		if err == nil && space < conditions.MinFloorSpace {
			return false
		}
	}
	if len(conditions.Layouts) > 0 && !slices.Contains(conditions.Layouts, room.Layout) {
		return false
	}
	return true
}

func firstNonEmpty(a, b string) string {
	if a != "" {
		return a
	}
	return b
}

// parseYen turns "118,300円" into 118300.
func parseYen(value string) (int, error) {
	value = strings.TrimSuffix(value, "円")
	value = strings.ReplaceAll(value, ",", "")
	return strconv.Atoi(value)
}

// parseFloorSpace turns "45&#13217;" (㎡ as an HTML entity) into 45.
func parseFloorSpace(value string) (float64, error) {
	return strconv.ParseFloat(strings.TrimSuffix(value, "&#13217;"), 64)
}
