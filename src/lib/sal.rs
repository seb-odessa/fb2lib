
//use time::Timespec;

use rusqlite::Error;
use rusqlite::Connection;
use torrent::Metainfo;
use rustc_serialize::hex::ToHex;
use std::collections::HashMap;

#[allow(dead_code)]
const CREATE_TABLES: &'static str = "
    BEGIN;
    CREATE TABLE archives (
	    id         	    INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    name   	        TEXT NOT NULL,
	    created    	    TEXT NOT NULL,
	    hash       	    TEXT NOT NULL UNIQUE,
	    total_length	INTEGER NOT NULL,
	    piece_length	INTEGER NOT NULL,
	    pieces_count	INTEGER NOT NULL
    );

    CREATE TABLE pieces (
	    id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    archive_id  	INTEGER NOT NULL, /* FK to archives.id */
	    piece_idx       INTEGER NOT NULL,
	    hash      	    TEXT NOT NULL UNIQUE
    );

    CREATE TABLE books (
        id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
        archive_id      INTEGER NOT NULL,       /* FK to archives.id */
        file_name       TEXT NOT NULL UNIQUE,   /* e.g.: book.fb2 */
        method          INTEGER,
        packed_size     INTEGER,
        unpacked_size   INTEGER,
        file_offset     INTEGER
    );
    COMMIT;
";

const DROP_TABLES: &'static str = "
    BEGIN;
    DROP TABLE archives;
    DROP TABLE pieces;
    COMMIT;
";

const QUERY_INDEX_AND_HASH: &'static str = "SELECT piece_idx, hash FROM pieces WHERE archive_id = ?1";

const QUERY_ARCHIVE_SIZES: &'static str = "SELECT id, total_length, piece_length, pieces_count FROM archives WHERE name = ?1";

const QUERY_HASH_BY_INDEX: &'static str = "SELECT hash FROM pieces WHERE archive_id = ?1 AND piece_idx = ?2";

const INSERT_ARCHIVE: &'static str = "INSERT INTO archives (name, created, hash, total_length, piece_length, pieces_count) VALUES (?, ?, ?, ?, ?, ?)";

const INSERT_PIECE: &'static str = "INSERT INTO pieces (archive_id, offset, hash) VALUES (?, ?, ?)";


pub type SalResult<T> = Result<Option<T>, Error>;
pub type HashesByIdx = HashMap<i64, String>;

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

pub fn get_archive_sizes(conn: &Connection, archive: &str) -> SalResult<ArchiveSizes> {
    let mut stmt = conn.prepare(QUERY_ARCHIVE_SIZES)?;
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

pub fn validate(conn: &Connection, id: i64, desc: &HashesByIdx) -> SalResult<i64> {
    let mut stmt = conn.prepare(QUERY_INDEX_AND_HASH)?;
    let rows = stmt.query_map(&[&id], |row| (row.get(0), row.get(1)))?;
    for row in rows {
        let (index, hash): (i64, String) = row?;
        if hash != desc[&index] {
            return Ok(Some(index));
        }
    }
    Ok(None)
}

pub fn get_hash(conn: &Connection, id: i64, index: i64) -> SalResult<String> {
    let mut stmt = conn.prepare(QUERY_HASH_BY_INDEX)?;
    let rows = stmt.query_map(&[&id, &index], |row| (row.get(0)))?;
    for row in rows {
        let hash: String = row?;
        return Ok(Some(hash));
    }
    Ok(None)
}

pub fn register(db_file_name: &str, metainfo: Metainfo) -> Result<(), Error> {
    let mut conn = Connection::open(db_file_name)?;
    conn.execute(INSERT_ARCHIVE, &[
        &metainfo.get_file_name(),
        &metainfo.get_creation_date(),
        &metainfo.get_info_hash(),
        &(metainfo.get_length() as i64),
        &(metainfo.get_piece_length() as i64),
        &(metainfo.get_piece_count() as i64),
    ])?;
    let archive_id = conn.last_insert_rowid();
    let tx = conn.transaction()?;
    {
       let mut stmt = tx.prepare(INSERT_PIECE)?;
        let pieces: &[u8] = metainfo.info.pieces.as_ref();
        let mut index = 0;
        for sha1 in pieces.chunks(20) {
            stmt.execute(&[&archive_id, &index, &sha1.to_hex().to_uppercase()])?;
            index += 1;
        }
    }
    tx.commit()?;
    Ok(())
}

pub fn init_tables(db_file_name: &str) -> Result<(), Error> {
    let conn = Connection::open(db_file_name)?;
    conn.execute_batch(CREATE_TABLES)
}

pub fn drop_tables(db_file_name: &str) -> Result<(), Error> {
    let conn = Connection::open(db_file_name)?;
    conn.execute_batch(DROP_TABLES)
}