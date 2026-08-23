package model

import "encoding/json"

// Danchi is one housing complex entry in the listing API response.
// Values are kept as the API sends them (strings); numeric conversion
// happens where a filter actually needs a number.
type Danchi struct {
	Name      string `json:"danchiNm"`
	Address   string `json:"place"`
	Traffic   string `json:"traffic"`  // station walk, HTML <li> list
	FloorAll  string `json:"floorAll"` // building floors, "14"
	AllCount  string `json:"allCount"` // total vacant rooms in the prefecture
	Shisya    string `json:"shisya"`
	DanchiID  string `json:"danchi"`
	Shikibetu string `json:"shikibetu"`
	RoomCount string `json:"roomCount"` // vacant rooms here; Rooms is truncated to 5
	Rooms     []Room `json:"room"`
}

// Key identifies a danchi across crawls — the same "20_2520" slug the
// danchi page URL uses.
func (d Danchi) Key() string {
	return d.Shisya + "_" + d.DanchiID + d.Shikibetu
}

// DanchiDetail holds crawled danchi attributes; nil = unreadable.
// 棟数 is published nowhere, so it is deliberately absent.
type DanchiDetail struct {
	UnitCount *int `json:"unitCount"` // 総戸数
	Floors    *int `json:"floors"`    // building floors of the sampled room
	BuiltYear *int `json:"builtYear"` // completion year, current year - age
}

// Room is one vacant room inside a Danchi. Exactly one of Rent/RentNormal
// is non-empty (rent carries discounted listings, rent_normal the rest).
type Room struct {
	ID         string `json:"id"`         // "000050602", keys the room-detail API
	Building   string `json:"roomNmMain"` // "7-1-1号棟"
	RoomNo     string `json:"roomNmSub"`  // "1305号室"
	Name       string `json:"name"`       // room-list API only: "5-7号棟402号室"
	Rent       string `json:"rent"`       // "118,300円"
	RentNormal string `json:"rent_normal"`
	CommonFee  string `json:"commonfee"`  // "3,600円"
	Layout     string `json:"type"`       // "1K"
	FloorSpace string `json:"floorspace"` // "45&#13217;" (㎡)
	Floor      string `json:"floor"`      // "13階"
	DetailPath string `json:"roomLinkPc"` // "/chintai/kanto/tokyo/…_room.html?JKSS=…"
}

// ParseBukkenResult unmarshals one listing API response body.
func ParseBukkenResult(body []byte) ([]Danchi, error) {
	var danchis []Danchi
	if err := json.Unmarshal(body, &danchis); err != nil {
		return nil, err
	}
	return danchis, nil
}

// ParseDetailRooms unmarshals one room-list API response body (a flat array
// of rooms, no danchi wrapper).
func ParseDetailRooms(body []byte) ([]Room, error) {
	var rooms []Room
	if err := json.Unmarshal(body, &rooms); err != nil {
		return nil, err
	}
	return rooms, nil
}
