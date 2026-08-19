package service

import (
	"testing"

	"blog-go-api/internal/ur/model"
)

func room(rent, rentNormal, commonFee, layout, floorSpace string) model.Room {
	return model.Room{Rent: rent, RentNormal: rentNormal, CommonFee: commonFee, Layout: layout, FloorSpace: floorSpace}
}

// passingRoom clears every default condition, so a case fails only by the field it changes.
func passingRoom() model.Room {
	return room("100,000円", "", "3,600円", "1LDK", "45&#13217;")
}

func filterOne(danchi model.Danchi) bool {
	return len(Filter([]model.Danchi{danchi}, DefaultConditions)) == 1
}

func TestFilterDefaultConditions(t *testing.T) {
	tests := []struct {
		name string
		room model.Room
		want bool
	}{
		{"01 sum exactly 120,000 passes", room("116,400円", "", "3,600円", "1LDK", "45&#13217;"), true},
		{"02 sum 120,100 fails", room("116,500円", "", "3,600円", "1LDK", "45&#13217;"), false},
		{"03 rent_normal is used when rent is empty", room("", "116,400円", "3,600円", "2DK", "45&#13217;"), true},
		{"04 rent_normal over budget fails", room("", "130,000円", "3,600円", "2DK", "45&#13217;"), false},
		{"05 empty common fee judges on rent alone", room("118,300円", "", "", "2LDK", "45&#13217;"), true},
		{"06 floor space exactly 40 passes", room("100,000円", "", "3,600円", "1LDK", "40&#13217;"), true},
		{"07 floor space 39.9 fails", room("100,000円", "", "3,600円", "1LDK", "39.9&#13217;"), false},
		{"08 layout 1K fails", room("100,000円", "", "3,600円", "1K", "45&#13217;"), false},
		{"09 layout 2LDK passes", room("100,000円", "", "3,600円", "2LDK", "45&#13217;"), true},
		{"10 unparseable rent passes through", room("要相談", "", "3,600円", "1LDK", "45&#13217;"), true},
		{"11 unparseable floor space passes through", room("100,000円", "", "3,600円", "1LDK", "広め"), true},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if passed := filterOne(model.Danchi{Rooms: []model.Room{tt.room}}); passed != tt.want {
				t.Errorf("passed = %v, want %v", passed, tt.want)
			}
		})
	}
}

func TestFilterFloorAndStationWalk(t *testing.T) {
	walkable := "<li>京王線「芦花公園」駅 徒歩3分</li>"
	tests := []struct {
		name     string
		floor    string
		floorAll string
		traffic  string
		want     bool
	}{
		{"01 floor exactly 3 passes", "3階", "10", walkable, true},
		{"02 floor 2 fails", "2階", "10", walkable, false},
		{"03 top floor 10 of 10 fails", "10階", "10", walkable, false},
		{"04 floor 9 of 10 passes", "9階", "10", walkable, true},
		{"05 unparseable floor passes through", "地下", "10", walkable, true},
		{"06 walk exactly 10 passes", "5階", "10", "<li>京王線「仙川」駅 徒歩10分</li>", true},
		{"07 walk 11 fails", "5階", "10", "<li>京王線「仙川」駅 徒歩11分</li>", false},
		{"08 range reads its low end", "5階", "10", "<li>京王線「仙川」駅 徒歩8～12分</li>", true},
		{"09 shortest station wins", "5階", "10", "<li>A駅 徒歩15分</li><li>B駅 徒歩9分</li>", true},
		{"10 bus walk is not a station walk", "5階", "10", "<li>八幡山」駅バス5分 徒歩2～4分</li><li>仙川」駅 徒歩16分</li>", false},
		{"11 bus-only traffic passes as missing", "5階", "10", "<li>千歳船橋」駅バス4分 徒歩2分</li>", true},
		{"12 empty traffic passes as missing", "5階", "10", "", true},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			testRoom := passingRoom()
			testRoom.Floor = tt.floor
			danchi := model.Danchi{FloorAll: tt.floorAll, Traffic: tt.traffic, Rooms: []model.Room{testRoom}}
			if passed := filterOne(danchi); passed != tt.want {
				t.Errorf("passed = %v, want %v", passed, tt.want)
			}
		})
	}
}

func TestFilterZeroConditionsPassEverything(t *testing.T) {
	worstRoom := room("999,999円", "", "9,999円", "1K", "10&#13217;")
	worstRoom.Floor = "10階" // top floor of a 10-floor building
	danchi := model.Danchi{FloorAll: "10", Traffic: "<li>仙川」駅 徒歩30分</li>", Rooms: []model.Room{worstRoom}}
	if got := Filter([]model.Danchi{danchi}, Conditions{}); len(got) != 1 {
		t.Error("zero conditions filtered out the danchi")
	}
}

func TestFilterDropsEmptiedDanchi(t *testing.T) {
	failing := model.Danchi{Rooms: []model.Room{room("130,000円", "", "3,600円", "1LDK", "45&#13217;")}}
	passing := model.Danchi{Rooms: []model.Room{passingRoom()}}
	got := Filter([]model.Danchi{failing, passing}, DefaultConditions)
	if len(got) != 1 || len(got[0].Rooms) != 1 {
		t.Fatalf("expected 1 danchi with 1 room, got %d danchi", len(got))
	}
}
