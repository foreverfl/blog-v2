#!/usr/bin/env bash
set -euo pipefail

container=api-notes-mysql-help

# a throwaway server only for its help tables; removed on exit
docker run --rm -d --name "$container" -e MYSQL_ALLOW_EMPTY_PASSWORD=yes mysql:8.4 >/dev/null
trap 'docker stop "$container" >/dev/null' EXIT

# the entrypoint restarts mysqld once after init, so wait for the real server, not the temporary one
until docker logs "$container" 2>&1 | grep -q 'ready for connections.*port: 3306'; do
  sleep 1
done

docker exec "$container" mysql -uroot -N -B -e "
SELECT c.name, t.name, t.url
FROM mysql.help_topic t
JOIN mysql.help_category c ON t.help_category_id = c.help_category_id
WHERE c.name <> 'Help Metadata'
ORDER BY c.name, t.name
"
