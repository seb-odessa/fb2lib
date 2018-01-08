
//use time::Timespec;
use queries;
use torrent::Metainfo;
use rusqlite;
use rusqlite::Connection;
use rustc_serialize::hex::ToHex;
use std::collections::HashMap;

const DROP_TABLES: &'static str = "
    BEGIN;
    DROP TABLE archives;
    DROP TABLE pieces;
    COMMIT;
";


pub type SalResultOption<T> = Result<Option<T>, rusqlite::Error>;
pub type SalResult<T> = Result<T, rusqlite::Error>;
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

pub fn get_connection(db_file_name: &str) -> SalResult<Connection> {
    Connection::open(db_file_name)
}

pub fn get_archive_sizes(conn: &Connection, name: &str) -> SalResultOption<ArchiveSizes> {
    let mut stmt = conn.prepare(queries::GET_ARCH_SIZES_BY_NAME)?;
    let rows = stmt.query_map(&[&name], |row| {
        ArchiveSizes::new(row.get(0), row.get(1), row.get(2), row.get(3))
    })?;
    for row in rows {
        let arch = row?;
        println!("Found {:?}", arch);
        return Ok(Some(arch)); // it is ok due to name column is unique
    }
    Ok(None)
}

pub fn validate(conn: &Connection, id: i64, desc: &HashesByIdx) -> SalResultOption<i64> {
    let mut stmt = conn.prepare(queries::GET_INDEX_AND_HASH_BY_ARCH_ID)?;
    let rows = stmt.query_map(&[&id], |row| (row.get(0), row.get(1)))?;
    for row in rows {
        let (index, hash): (i64, String) = row?;
        if hash != desc[&index] {
            return Ok(Some(index));
        }
    }
    Ok(None)
}

pub fn get_hash(conn: &Connection, id: i64, index: i64) -> SalResultOption<String> {
    let mut stmt = conn.prepare(queries::GET_HASH_BY_ARCH_ID_AND_INDEX)?;
    let rows = stmt.query_map(&[&id, &index], |row| (row.get(0)))?;
    for row in rows {
        let hash: String = row?;
        return Ok(Some(hash));
    }
    Ok(None)
}

fn get_archive_id(conn: &Connection, metainfo: &Metainfo) -> SalResult<i64> {
    let mut stmt = conn.prepare(queries::GET_ID_BY_HASH)?;
    let rows = stmt.query_map(&[&metainfo.get_info_hash()], |row| row.get(0))?;
    for row in rows {
        let id = row?;
        return Ok(id);
    }
    conn.execute(queries::INSERT_ARCHIVE, &[
        &metainfo.get_file_name(),
        &metainfo.get_creation_date(),
        &metainfo.get_info_hash(),
        &(metainfo.get_length() as i64),
        &(metainfo.get_piece_length() as i64),
        &(metainfo.get_piece_count() as i64),
    ])?;
    Ok(conn.last_insert_rowid())
}

pub fn register(db_file_name: &str, metainfo: Metainfo) -> SalResult<()> {
    let mut conn = Connection::open(db_file_name)?;
    let archive_id = get_archive_id(&conn, &metainfo)?;
    let tx = conn.transaction()?;
    {
        let mut stmt = tx.prepare(queries::INSERT_PIECE)?;
        let pieces: &[u8] = metainfo.info.pieces.as_ref();
        let mut index = 0;
        for sha1 in pieces.chunks(20) {
            stmt.execute(&[&archive_id, &index, &sha1.to_hex()])?;
            index += 1;
        }
    }
    tx.commit()
}

pub fn init_tables(db_file_name: &str) -> SalResult<()> {
    let mut conn = Connection::open(db_file_name)?;
    let tx = conn.transaction()?;
    tx.execute(queries::CREATE_ARCHIVES, &[])?;
    tx.execute(queries::CREATE_PIECES, &[])?;
    tx.commit()
}

pub fn drop_tables(db_file_name: &str) -> SalResult<()> {
    let conn = Connection::open(db_file_name)?;
    conn.execute_batch(DROP_TABLES)
}