#!/bin/bash

wsm="./sqlite2kvpairs.wasm"

hdir="$PWD"
gdir="/guest.d"
bname="example.sqlite.db3"
gpath="${gdir}/${bname}"
hpath="${hdir}/${bname}"

geninput(){
  echo trying to create the example sqlite db file...

  sqlite3 \
    "${hpath}" \
    "
      CREATE TABLE IF NOT EXISTS kvpairs(
        key TEXT PRIMARY KEY,
        val TEXT
      )
      WITHOUT ROWID
    "

  sqlite3 "${hpath}" "INSERT INTO kvpairs VALUES ('helo',  'wrld')"
  sqlite3 "${hpath}" "INSERT INTO kvpairs VALUES ('hello', 'world')"
}

run_wasi(){
    wasmtime \
      run \
      --env ENV_SQLITE_DB_FILE_PATH="${gpath}" \
      --dir "${hdir}::${gdir}" \
      "${wsm}"
}

test -f "${hpath}" || geninput

run_wasi |
  jq -c
