package model

import (
	"os"
	"strings"
	"testing"
)

// Sample is a real Setagaya (skcs=112) response captured on 2026-08-14,
// with one vacant room in 希望ヶ丘.
func TestParseBukkenResult(t *testing.T) {
	body, err := os.ReadFile("testdata/bukken_result_sample.json")
	if err != nil {
		t.Fatal(err)
	}

	danchis, err := ParseBukkenResult(body)
	if err != nil {
		t.Fatal(err)
	}
	if len(danchis) != 16 {
		t.Fatalf("expected 16 danchi, got %d", len(danchis))
	}

	kibo := danchis[0]
	if kibo.Name != "希望ヶ丘" {
		t.Errorf("danchi name = %q", kibo.Name)
	}
	if kibo.Address != "世田谷区船橋6ほか" {
		t.Errorf("address = %q", kibo.Address)
	}
	if !strings.Contains(kibo.Traffic, "徒歩") {
		t.Errorf("traffic = %q", kibo.Traffic)
	}
	if len(kibo.Rooms) != 1 {
		t.Fatalf("expected 1 room, got %d", len(kibo.Rooms))
	}

	room := kibo.Rooms[0]
	if room.Rent != "118,300円" {
		t.Errorf("rent = %q", room.Rent)
	}
	if room.CommonFee != "3,600円" {
		t.Errorf("commonfee = %q", room.CommonFee)
	}
	if room.Layout != "1K" {
		t.Errorf("layout = %q", room.Layout)
	}
	if room.FloorSpace != "45&#13217;" {
		t.Errorf("floorspace = %q", room.FloorSpace)
	}
	if room.Floor != "13階" {
		t.Errorf("floor = %q", room.Floor)
	}
	if !strings.HasPrefix(room.DetailPath, "/chintai/") {
		t.Errorf("detail path = %q", room.DetailPath)
	}
}
