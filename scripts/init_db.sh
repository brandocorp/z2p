#!/bin/bash
set -x
set -eo pipefail

DB_HOST="localhost"
DB_USER=${POSTGRES_USER:=postgres}
DB_PASS=${POSTGRES_PASSWORD:=password}
DB_NAME=${POSTGRES_DB:=newsletter}
DB_PORT=${POSGRES_PORT:=5432}
DB_CONN="postgres://${DB_USER}:${DB_PASS}@${DB_HOST}:${DB_PORT}/${DB_NAME}"

create_postgres() {
  docker run                        \
    -e POSTGRES_USER=${DB_USER}     \
    -e POSTGRES_PASSWORD=${DB_PASS} \
    -e POSTGRES_DB=${DB_NAME}       \
    -p "${DB_PORT}:5432"            \
    -d postgres                     \
    postgres -N 1000
}

wait_for_postgres() {
  export PGPASSWORD=${DB_PASS}
  
  until psql -h "$DB_HOST" -U "$DB_USER" -p "$DB_PORT" -d "postgres" -c '\q'; do
    >&2 echo "Postgres is still unavailable - sleeping..."
    sleep 1
  done

  unset PGPASSWORD
  
  >&2 echo "Postgres is now ready: ${DB_HOST}:${DB_PORT}"
}

if [[ -z "${SKIP_DOCKER}" ]]; then
  create_postgres
  wait_for_postgres
fi

export DATABASE_URL=${DB_CONN}
sqlx database create
sqlx migrate run
