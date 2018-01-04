
//use time::Timespec;
use rusqlite::Error;
use rusqlite::Connection;

use std::collections::HashMap;
/*
    TABLE archives( - shall be created by import tool
        id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
        name            TEXT NOT NULL UNIQUE,
        created         TEXT NOT NULL,
        hash            TEXT NOT NULL UNIQUE,
        total_length    INTEGER NOT NULL,
        piece_length    INTEGER NOT NULL,
        pieces_count    INTEGER NOT NULL
    )

    TABLE pieces ( - shall be created by import tooll
        id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
        archive_id      INTEGER NOT NULL,       /* FK to archives.id */
        offset          INTEGER NOT NULL,       /* index of the piece in the file */
        hash            TEXT NOT NULL UNIQUE,
    )


    CREATE TABLE books (
        id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
        archive_id      INTEGER NOT NULL,       /* FK to archives.id */
        file_name       TEXT NOT NULL UNIQUE,   /* e.g.: book.fb2 */
        method          INTEGER,
        packed_size     INTEGER,
        unpacked_size   INTEGER,
        file_offset     INTEGER,
    )
*/

#[derive(Debug)]
pub struct ArchiveSizes {
    pub id: i64,
    pub total_length: usize,
    pub piece_length: usize,
    pub pieces_count: usize,
}
impl ArchiveSizes {
    pub fn new(id: i64, total_length: i64, piece_length: i64, pieces_count: i64) -> Self {
        ArchiveSizes {
            id: id,
            total_length: total_length as usize,
            piece_length: piece_length as usize,
            pieces_count: pieces_count as usize,
        }
    }
}

pub fn get_connection(db_file_name: &str) -> Result<Connection, Error> {
    Connection::open(db_file_name)
}

pub fn get_archive_sizes(conn: &Connection, archive: &str) -> Result<Option<ArchiveSizes>, Error> {
    let mut stmt = conn.prepare(
        "SELECT id, total_length, piece_length, pieces_count FROM archives WHERE name = ?1",
    )?;
    let rows = stmt.query_map(&[&archive], |row| {
        ArchiveSizes::new(row.get(0), row.get(1), row.get(2), row.get(3))
    })?;
    for row in rows {
        let arch = row?;
        println!("Found {:?}", arch);
        return Ok(Some(arch)); // it is ok due to `name` is TEXT NOT NULL UNIQUE
    }
    Ok(None)
}


pub fn validate_pieces(
    conn: &Connection,
    id: i64,
    desc: &HashMap<i64, String>,
) -> Result<i64, Error> {
    let mut stmt = conn.prepare_cached(
        "SELECT offset, hash FROM pieces WHERE archive_id = ?1",
    )?;
    let rows = stmt.query_map(&[&id], |row| (row.get(0), row.get(1)))?;
    for row in rows {
        let (index, hash): (i64, String) = row?;
        if hash != desc[&index] {
            return Ok(index);
        }
    }
    Ok(0)
}




pub fn init_tables(db_file_name: &str) -> Result<(), Error> {
    let mut conn = Connection::open(db_file_name)?;
    let tran = conn.transaction()?;
    tran.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  time_created    TEXT NOT NULL,
                  data            BLOB
                  )",
        &[],
    )?;
    tran.commit()
}
