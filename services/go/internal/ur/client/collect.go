package client

import (
	"context"
	"fmt"
	"log/slog"
	"strconv"
	"time"

	"blog-go-api/internal/ur/model"
)

// pageCap stops a runaway loop if the API ever stops returning empty pages.
const pageCap = 100

// CollectVacant pages through the prefecture's vacant rooms until an empty
// page, waiting politely between requests.
func CollectVacant(ctx context.Context, prefCode string) ([]model.Danchi, error) {
	var all []model.Danchi
	for page := 0; ; page++ {
		if page >= pageCap {
			return nil, fmt.Errorf("page cap %d reached for pref %s", pageCap, prefCode)
		}
		if page > 0 {
			select {
			case <-ctx.Done():
				return nil, ctx.Err()
			case <-time.After(1500 * time.Millisecond):
			}
		}

		body, err := FetchVacantPage(ctx, prefCode, page)
		if err != nil {
			return nil, err
		}
		danchis, err := model.ParseBukkenResult(body)
		if err != nil {
			return nil, err
		}
		if len(danchis) == 0 {
			break
		}
		all = append(all, danchis...)
	}

	rooms := 0
	for i := range all {
		if err := refetchTruncatedRooms(ctx, &all[i]); err != nil {
			return nil, err
		}
		rooms += len(all[i].Rooms)
	}
	slog.Info("ur vacant rooms collected", "pref", prefCode, "danchi", len(all), "rooms", rooms)
	return all, nil
}

// refetchTruncatedRooms refetches the danchi's full room list (replacing
// the listing's max-5 slice) when roomCount says some rooms are missing.
func refetchTruncatedRooms(ctx context.Context, danchi *model.Danchi) error {
	declaredCount, err := strconv.Atoi(danchi.RoomCount)
	if err != nil || len(danchi.Rooms) >= declaredCount {
		return nil
	}

	var rooms []model.Room
	for page := 0; len(rooms) < declaredCount && page < pageCap; page++ {
		select {
		case <-ctx.Done():
			return ctx.Err()
		case <-time.After(1500 * time.Millisecond):
		}

		body, err := FetchDanchiRoomPage(ctx, *danchi, page)
		if err != nil {
			return err
		}
		pageRooms, err := model.ParseDetailRooms(body)
		if err != nil {
			return err
		}
		if len(pageRooms) == 0 {
			break
		}
		rooms = append(rooms, pageRooms...)
	}
	danchi.Rooms = rooms
	return nil
}
