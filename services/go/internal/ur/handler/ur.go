package handler

import (
	"fmt"
	"net/http"
	"strconv"
	"strings"

	"blog-go-api/internal/common"
	"blog-go-api/internal/config"
	"blog-go-api/internal/ur/client"
	"blog-go-api/internal/ur/model"
	"blog-go-api/internal/ur/service"
)

// collectPrefCodes are the JIS prefecture codes the crawl covers
// (13 = Tokyo, 14 = Kanagawa).
var collectPrefCodes = []string{"13", "14"}

// ListingsHandler collects every vacant UR room and returns the danchis
// whose rooms pass the filter conditions.
//
// Request:  GET /ur/listings?max_rent=120000&min_area=40&min_floor=3&max_walk=10&types=1LDK,2DK
//
//	(each parameter overrides one default condition; 0 disables its check)
//
// Response: 200 []model.Danchi JSON / 400 malformed parameter / 502 crawl failed
func ListingsHandler(cfg *config.Config) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if !common.CheckAuth(r, cfg.ApiSecret) {
			common.WriteJSON(w, http.StatusUnauthorized, map[string]string{"error": "Unauthorized"})
			return
		}
		conditions, err := conditionsFromQuery(r)
		if err != nil {
			common.WriteJSON(w, http.StatusBadRequest, map[string]string{"error": err.Error()})
			return
		}

		var danchis []model.Danchi
		for _, prefCode := range collectPrefCodes {
			collected, err := client.CollectVacant(r.Context(), prefCode)
			if err != nil {
				common.WriteJSON(w, http.StatusBadGateway, map[string]string{"error": "ur collect failed: " + err.Error()})
				return
			}
			danchis = append(danchis, collected...)
		}

		passed := service.Filter(danchis, conditions)
		if passed == nil {
			passed = []model.Danchi{} // JSON [] instead of null
		}
		common.WriteJSON(w, http.StatusOK, passed)
	}
}

// conditionsFromQuery overrides the default conditions with query parameters.
func conditionsFromQuery(r *http.Request) (service.Conditions, error) {
	conditions := service.DefaultConditions
	query := r.URL.Query()

	intTargets := map[string]*int{
		"max_rent":  &conditions.MaxRentYen,
		"min_floor": &conditions.MinFloor,
		"max_walk":  &conditions.MaxStationWalkMin,
	}
	for name, target := range intTargets {
		if raw := query.Get(name); raw != "" {
			value, err := strconv.Atoi(raw)
			if err != nil {
				return conditions, fmt.Errorf("%s must be a whole number", name)
			}
			*target = value
		}
	}
	if raw := query.Get("min_area"); raw != "" {
		value, err := strconv.ParseFloat(raw, 64)
		if err != nil {
			return conditions, fmt.Errorf("min_area must be a number")
		}
		conditions.MinFloorSpace = value
	}
	if raw := query.Get("types"); raw != "" {
		conditions.Layouts = strings.Split(raw, ",")
	}
	return conditions, nil
}
