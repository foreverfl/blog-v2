package middleware

import (
	"log"
	"net"
	"net/http"
	"net/netip"
	"strings"
	"time"
)

type responseRecorder struct {
	http.ResponseWriter
	status int
	bytes  int
}

func (r *responseRecorder) WriteHeader(code int) {
	r.status = code
	r.ResponseWriter.WriteHeader(code)
}

func (r *responseRecorder) Write(b []byte) (int, error) {
	if r.status == 0 {
		r.status = http.StatusOK
	}
	n, err := r.ResponseWriter.Write(b)
	r.bytes += n
	return n, err
}

func (r *responseRecorder) Flush() {
	if f, ok := r.ResponseWriter.(http.Flusher); ok {
		f.Flush()
	}
}

func parseIP(s string) (string, bool) {
	addr, err := netip.ParseAddr(strings.TrimSpace(s))
	if err != nil {
		return "", false
	}
	return addr.String(), true
}

func remoteIP(r *http.Request) string {
	if host, _, err := net.SplitHostPort(r.RemoteAddr); err == nil {
		if ip, ok := parseIP(host); ok {
			return ip
		}
	}
	if ip, ok := parseIP(r.RemoteAddr); ok {
		return ip
	}
	return "unknown"
}

func forwardedIP(r *http.Request) string {
	if xff := r.Header.Get("X-Forwarded-For"); xff != "" {
		first := xff
		if idx := strings.IndexByte(first, ','); idx >= 0 {
			first = first[:idx]
		}
		if ip, ok := parseIP(first); ok {
			return ip
		}
	}
	if ip, ok := parseIP(r.Header.Get("X-Real-IP")); ok {
		return ip
	}
	return ""
}

func Logging(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		start := time.Now()
		rec := &responseRecorder{ResponseWriter: w}
		next.ServeHTTP(rec, r)
		if rec.status == 0 {
			rec.status = http.StatusOK
		}
		fwd := forwardedIP(r)
		if fwd == "" {
			fwd = "-"
		}
		log.Printf("%s %q %d %dB %s ip=%s fwd=%s ua=%q",
			r.Method,
			r.URL.Path,
			rec.status,
			rec.bytes,
			time.Since(start),
			remoteIP(r),
			fwd,
			r.UserAgent(),
		)
	})
}
