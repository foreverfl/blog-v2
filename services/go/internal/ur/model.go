package ur

import "encoding/json"

// Danchi is one housing complex entry in the listing API response.
// Values are kept as the API sends them (strings); numeric conversion
// happens where a filter actually needs a number.
type Danchi struct {
	Name    string `json:"danchiNm"`
	Address string `json:"place"`
	Traffic string `json:"traffic"` // station walk, HTML <li> list
	Rooms   []Room `json:"room"`
}

// Room is one vacant room inside a Danchi.
type Room struct {
	Building   string `json:"roomNmMain"` // "7-1-1号棟"
	RoomNo     string `json:"roomNmSub"`  // "1305号室"
	Rent       string `json:"rent"`       // "118,300円"
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
