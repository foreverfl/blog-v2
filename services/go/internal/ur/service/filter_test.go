package service

import (
	"testing"

	"blog-go-api/internal/ur/model"
)

func room(rent, rentNormal, commonFee, layout, floorSpace string) model.Room {
	return model.Room{Rent: rent, RentNormal: rentNormal, CommonFee: commonFee, Layout: layout, FloorSpace: floorSpace}
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
			got := Filter([]model.Room{tt.room}, DefaultConditions)
			if passed := len(got) == 1; passed != tt.want {
				t.Errorf("passed = %v, want %v", passed, tt.want)
			}
		})
	}
}

func TestFilterZeroConditionsPassEverything(t *testing.T) {
	rooms := []model.Room{room("999,999円", "", "9,999円", "1K", "10&#13217;")}
	if got := Filter(rooms, Conditions{}); len(got) != 1 {
		t.Errorf("zero conditions filtered out %d rooms", len(rooms)-len(got))
	}
}
