#!/usr/bin/env bash
set -euo pipefail

# pg_catalog functions a query would call; drops type I/O (int4in), operator bodies (int82lt) and trigger internals (RI_FKey_cascade_del)
query="
SELECT DISTINCT 'function', p.proname
FROM pg_proc p
WHERE p.pronamespace = 'pg_catalog'::regnamespace
  AND p.proname NOT LIKE 'pg\_%'
  AND p.proname NOT LIKE '\_%'
  AND p.prorettype NOT IN ('internal'::regtype, 'cstring'::regtype, 'trigger'::regtype, 'event_trigger'::regtype)
  AND NOT ('internal'::regtype = ANY (p.proargtypes::oid[]) OR 'cstring'::regtype = ANY (p.proargtypes::oid[]))
  AND coalesce(obj_description(p.oid, 'pg_proc'), '') NOT LIKE 'implementation of %'
ORDER BY 2
"

docker exec blog-postgres sh -c 'psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" -At -F "$(printf "\t")" -c "$1"' sh "$query"
