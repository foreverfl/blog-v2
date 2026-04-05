package dateutil

import "time"

var kst = time.FixedZone("KST", 9*60*60)

// TodayKST returns today's date in YYMMDD format (KST).
func TodayKST() string {
	return time.Now().In(kst).Format("060102")
}

// ResolveDate returns the given date string or today's KST date if empty.
func ResolveDate(date string) string {
	if date == "" {
		return TodayKST()
	}
	return date
}
