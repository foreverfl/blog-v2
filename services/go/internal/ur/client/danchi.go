package client

import (
	"context"
	"encoding/json"
	"fmt"
	"net/url"
	"regexp"
	"strconv"
	"strings"
	"time"

	"blog-go-api/internal/ur/model"
)

const (
	danchiPageBase = "https://www.ur-net.go.jp/chintai/"
	roomDetailURL  = "https://chintai.r6.ur-net.go.jp/chintai/api/bukken/detail/detail_room/"
)

// prefPagePath maps a JIS prefecture code to the danchi page path segment.
var prefPagePath = map[string]string{"13": "kanto/tokyo", "14": "kanto/kanagawa"}

var unitCountPattern = regexp.MustCompile(`<th[^>]*>戸数</th>\s*<td[^>]*>\s*(?:<p[^>]*>)?\s*([0-9,]+)`)

// FetchDanchiDetail: 総戸数 lives in the static danchi page, 階数 and build
// year only in the room-detail API — hence the two requests.
func FetchDanchiDetail(ctx context.Context, prefCode string, danchi model.Danchi) (model.DanchiDetail, error) {
	var detail model.DanchiDetail

	// 1. Unit count (総戸数): GET the danchi's static page, read the 戸数 table.
	pagePath, ok := prefPagePath[prefCode]
	if !ok {
		return detail, fmt.Errorf("unknown prefecture code %q", prefCode)
	}
	pageURL := fmt.Sprintf("%s%s/%s_%s%s.html", danchiPageBase, pagePath, danchi.Shisya, danchi.DanchiID, danchi.Shikibetu)
	page, err := getPage(ctx, pageURL)
	if err != nil {
		return detail, err
	}
	detail.UnitCount = parseUnitCount(page)

	// 2. Floors (階数) + build year: POST the room-detail API for one vacant room.
	roomID := firstRoomID(danchi)
	if roomID == "" {
		return detail, nil // no room to ask; floors and year stay nil
	}
	select {
	case <-ctx.Done():
		return detail, ctx.Err()
	case <-time.After(1500 * time.Millisecond):
	}
	body, err := postForm(ctx, roomDetailURL, url.Values{
		"id":        {roomID},
		"shisya":    {danchi.Shisya},
		"danchi":    {danchi.DanchiID},
		"shikibetu": {danchi.Shikibetu},
	})
	if err != nil {
		return detail, err
	}
	var rooms []struct {
		Year    string `json:"year"`     // building age in years, "52"
		FloorSp string `json:"floor_sp"` // "6階 /8階" — room floor / building floors
	}
	if err := json.Unmarshal(body, &rooms); err != nil || len(rooms) == 0 {
		return detail, nil // detail API answered garbage; page fields still count
	}
	detail.Floors = parseTotalFloors(rooms[0].FloorSp)
	detail.BuiltYear = builtYearFromAge(rooms[0].Year)
	return detail, nil
}

func parseUnitCount(page []byte) *int {
	match := unitCountPattern.FindSubmatch(page)
	if match == nil {
		return nil
	}
	count, err := strconv.Atoi(strings.ReplaceAll(string(match[1]), ",", ""))
	if err != nil {
		return nil
	}
	return &count
}

// parseTotalFloors reads the building floors from "6階 /8階".
func parseTotalFloors(floorSp string) *int {
	_, after, ok := strings.Cut(floorSp, "/")
	if !ok {
		return nil
	}
	floors, err := strconv.Atoi(strings.TrimSuffix(strings.TrimSpace(after), "階"))
	if err != nil {
		return nil
	}
	return &floors
}

func builtYearFromAge(ageText string) *int {
	age, err := strconv.Atoi(ageText)
	if err != nil {
		return nil
	}
	year := time.Now().Year() - age
	return &year
}

func firstRoomID(danchi model.Danchi) string {
	for _, room := range danchi.Rooms {
		if room.ID != "" {
			return room.ID
		}
	}
	return ""
}
