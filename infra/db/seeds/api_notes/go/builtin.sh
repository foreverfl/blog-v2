#!/usr/bin/env bash
set -euo pipefail

if [ $# -eq 0 ]; then
  echo "usage: builtin.sh <go module dir>... > builtin.tsv" >&2
  exit 2
fi

# standard packages imported directly by the given modules, internal ones left out
std_packages=$(
  for module_dir in "$@"; do
    (cd "$module_dir" && go list -f '{{join .Imports "\n"}}' ./...)
  done | sort -u | grep -F -x -f <(go list std) | grep -v -e internal -e vendor
)

# exported funcs, types and methods: "func Split(", "type Builder ", "func (s *Scanner) Scan("
tab=$'\t'
for package in $std_packages; do
  go doc -all "$package" | sed -n -E \
    -e "s|^func ([A-Z][A-Za-z0-9_]*)[[(].*|$package$tab\1|p" \
    -e "s|^func \([a-z]* \*?([A-Z][A-Za-z0-9_]*)(\[[^]]*\])?\) ([A-Z][A-Za-z0-9_]*)\(.*|$package$tab\1.\3|p" \
    -e "s|^type ([A-Z][A-Za-z0-9_]*)[ [].*|$package$tab\1|p"
done | sort -u
