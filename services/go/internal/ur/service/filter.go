package service

import (
	"slices"
	"strconv"
	"strings"

	"blog-go-api/internal/ur/model"
)

// Conditions selects rooms; a zero/empty field skips that check.
type Conditions struct {
	MaxRentYen        int      // rent + common fee, inclusive
	MinFloorSpace     float64  // ㎡, inclusive
	Layouts           []string // accepted 間取り values, exact match
	MinFloor          int      // lowest acceptable floor, inclusive
	ExcludeTopFloor   bool     // drop rooms on the building's top floor
	MaxStationWalkMin int      // shortest station walk, minutes, inclusive
}

// DefaultConditions mirrors the values in find-accomodation.md.
var DefaultConditions = Conditions{
	MaxRentYen:        120_000,
	MinFloorSpace:     40,
	Layouts:           []string{"1LDK", "2DK", "2LDK"},
	MinFloor:          3,
	ExcludeTopFloor:   true,
	MaxStationWalkMin: 10,
}

// Filter returns the danchis whose rooms pass every condition, keeping only
// those rooms; danchis left with no room are dropped. A value that fails to
// parse passes its check — over-filtering at collection is worse than noise.
func Filter(danchis []model.Danchi, conditions Conditions) []model.Danchi {
	var passed []model.Danchi
	for _, danchi := range danchis {
		var rooms []model.Room
		for _, room := range danchi.Rooms {
			if roomPasses(danchi, room, conditions) {
				rooms = append(rooms, room)
			}
		}
		if len(rooms) > 0 {
			danchi.Rooms = rooms
			passed = append(passed, danchi)
		}
	}
	return passed
}

func roomPasses(danchi model.Danchi, room model.Room, conditions Conditions) bool {
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
	if conditions.MinFloor > 0 || conditions.ExcludeTopFloor {
		if floor, ok := leadingInt(room.Floor); ok {
			if conditions.MinFloor > 0 && floor < conditions.MinFloor {
				return false
			}
			if conditions.ExcludeTopFloor {
				if topFloor, err := strconv.Atoi(danchi.FloorAll); err == nil && floor >= topFloor {
					return false
				}
			}
		}
	}
	if conditions.MaxStationWalkMin > 0 {
		if walk, ok := parseStationWalk(danchi.Traffic); ok && walk > conditions.MaxStationWalkMin {
			return false
		}
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

// parseStationWalk finds the shortest 徒歩 minutes in a traffic HTML list;
// バス entries are skipped since their 徒歩 counts from the bus stop.
func parseStationWalk(traffic string) (int, bool) {
	shortest, found := 0, false
	for _, entry := range strings.Split(traffic, "<li>") {
		if strings.Contains(entry, "バス") {
			continue
		}
		_, afterWalk, ok := strings.Cut(entry, "徒歩")
		if !ok {
			continue
		}
		minutes, ok := leadingInt(afterWalk)
		if !ok {
			continue
		}
		if !found || minutes < shortest {
			shortest, found = minutes, true
		}
	}
	return shortest, found
}

// leadingInt reads the ASCII digits value starts with ("13階" → 13).
func leadingInt(value string) (int, bool) {
	end := 0
	for end < len(value) && value[end] >= '0' && value[end] <= '9' {
		end++
	}
	if end == 0 {
		return 0, false
	}
	number, err := strconv.Atoi(value[:end])
	if err != nil {
		return 0, false
	}
	return number, true
}
