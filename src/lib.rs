use std::io;
use std::path::Path;

use io::BufWriter;
use io::Write;

use rusqlite::Connection;

use rusqlite::Rows;
use rusqlite::types::ValueRef;

pub fn sqlite2kvpairs2sink<S>(conn: &Connection, mut sink: S) -> Result<(), io::Error>
where
    S: FnMut(&str, Option<&str>) -> Result<(), io::Error>,
{
    let mut stmt = conn
        .prepare(
            r#"
                SELECT
                    key,
                    val
                FROM kvpairs
                ORDER BY key
            "#,
        )
        .map_err(io::Error::other)?;

    let mut rows: Rows = stmt.query(()).map_err(io::Error::other)?;

    while let Some(row) = rows.next().map_err(io::Error::other)? {
        let vkey: ValueRef = row.get_ref(0).map_err(io::Error::other)?;
        let key: &str = vkey.as_str().map_err(io::Error::other)?;

        let vval: ValueRef = row.get_ref(1).map_err(io::Error::other)?;
        let oval: Option<&str> = vval.as_str_or_null().map_err(io::Error::other)?;

        sink(key, oval)?;
    }
    drop(rows);

    stmt.finalize().map_err(io::Error::other)?;

    Ok(())
}

pub fn wtr2sink4json<W>(mut wtr: W) -> impl FnMut(&str, Option<&str>) -> Result<(), io::Error>
where
    W: Write,
{
    move |key: &str, oval: Option<&str>| {
        serde_json::to_writer(&mut wtr, &(key, oval))?;
        writeln!(&mut wtr)
    }
}

pub fn path2sqlite2pairs2wtr4json<P, W>(path2sqlite: P, mut wtr: W) -> Result<(), io::Error>
where
    P: AsRef<Path>,
    W: Write,
{
    let conn = Connection::open(path2sqlite).map_err(io::Error::other)?;
    let sink = wtr2sink4json(&mut wtr);
    sqlite2kvpairs2sink(&conn, sink)?;
    wtr.flush()
}

pub fn path2sqlite2pairs2stdout4json<P>(path2sqlite: P) -> Result<(), io::Error>
where
    P: AsRef<Path>,
{
    let o = io::stdout();
    let mut ol = o.lock();
    path2sqlite2pairs2wtr4json(path2sqlite, BufWriter::new(&mut ol))?;
    ol.flush()
}
