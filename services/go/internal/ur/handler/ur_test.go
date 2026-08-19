package handler

import (
	"net/http/httptest"
	"testing"
)

func TestConditionsFromQuery(t *testing.T) {
	request := httptest.NewRequest("GET", "/ur/listings?max_rent=130000&min_area=30.5&types=1K,1LDK", nil)
	conditions, err := conditionsFromQuery(request)
	if err != nil {
		t.Fatal(err)
	}
	if conditions.MaxRentYen != 130000 || conditions.MinFloorSpace != 30.5 {
		t.Errorf("overrides not applied: %+v", conditions)
	}
	if len(conditions.Layouts) != 2 || conditions.Layouts[0] != "1K" {
		t.Errorf("types not applied: %v", conditions.Layouts)
	}
	if conditions.MinFloor != 3 || conditions.MaxStationWalkMin != 10 {
		t.Errorf("untouched params lost their defaults: %+v", conditions)
	}
}

func TestConditionsFromQueryRejectsBadNumber(t *testing.T) {
	request := httptest.NewRequest("GET", "/ur/listings?max_rent=cheap", nil)
	if _, err := conditionsFromQuery(request); err == nil {
		t.Error("expected an error for max_rent=cheap")
	}
}
