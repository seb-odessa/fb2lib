use result::{into, Fb2Result};
use sal::query_create;
use sal::query_drop;
use sal::query_insert;
use sal::query_select;

use sal::HashesByIdx;


use torrent::Metainfo;

use rusqlite;
use rusqlite::{Connection, Transaction};
use rustc_serialize::hex::ToHex;

pub type SalResultOption<T> = Result<Option<T>, rusqlite::Error>;
pub type SalResult<T> = Result<T, rusqlite::Error>;


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

pub fn get_connection(db_file_name: &str) -> Fb2Result<Connection> {
    Connection::open(db_file_name).map_err(into)
}

pub fn get_archive_sizes(conn: &Connection, name: &str) -> SalResultOption<ArchiveSizes> {
    let mut stmt = conn.prepare(query_select::ARCH_SIZES_BY_NAME)?;
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
    let mut stmt = conn.prepare(query_select::INDEX_AND_HASH_BY_ARCH_ID)?;
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
    let mut stmt = conn.prepare(query_select::HASH_BY_ARCH_ID_AND_INDEX)?;
    let rows = stmt.query_map(&[&id, &index], |row| (row.get(0)))?;
    for row in rows {
        let hash: String = row?;
        return Ok(Some(hash));
    }
    Ok(None)
}

fn get_archive_id(conn: &Connection, metainfo: &Metainfo) -> SalResult<i64> {
    let mut stmt = conn.prepare(query_select::ID_BY_HASH)?;
    let rows = stmt.query_map(&[&metainfo.get_info_hash()], |row| row.get(0))?;
     for row in rows {
        let id = row?;
        return Ok(id);

     }
    conn.execute(query_insert::ARCHIVE, &[
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
        let mut stmt = tx.prepare(query_insert::PIECE)?;
        let pieces: &[u8] = metainfo.info.pieces.as_ref();
        let mut index = 0;
        for sha1 in pieces.chunks(20) {
            stmt.execute(&[&archive_id, &index, &sha1.to_hex()])?;
            index += 1;
        }
    }
    tx.commit()
}

pub fn reset_tables(db_file_name: &str) -> SalResult<()> {
    let conn = Connection::open(db_file_name)?;
    // conn.execute(query_drop::ARCHIVES, &[])?;
    // conn.execute(query_drop::PIECES, &[])?;
    // conn.execute(query_drop::LANGUAGES, &[])?;
    // conn.execute(query_drop::FILTERS, &[])?;
    // conn.execute(query_drop::FILTERS_DEF, &[])?;
    // conn.execute(query_drop::LANGUAGES_DISABLED, &[])?;
    // conn.execute(query_drop::LANGUAGES_ENABLED, &[])?;
    conn.execute_batch(query_drop::GENRE_SUBSYSTEM)?;

    // conn.execute(query_create::ARCHIVES, &[])?;
    // conn.execute(query_create::PIECES, &[])?;
    // conn.execute(query_create::LANGUAGES, &[])?;
    // conn.execute(query_create::LANGUAGES_AUTO, &[])?;
    // conn.execute(query_create::FILTERS, &[])?;
    // conn.execute(query_create::FILTERS_DEF, &[])?;
    // conn.execute(query_create::FILL_FILTER, &[&1, &"lang"])?;
    // conn.execute(query_create::FILL_FILTER, &[&2, &"genre"])?;
    // conn.execute(query_create::LANGUAGES_DISABLED, &[])?;
    // conn.execute(query_create::LANGUAGES_ENABLED, &[])?;
    conn.execute_batch(query_create::GENRE_SUBSYSTEM)?;
    conn.execute_batch(query_insert::INSERT_GENRES)?;

    Ok(())
}

pub fn insert_language(tx: &Transaction, lang: &str) -> SalResult<i32> {
    tx.execute(query_insert::LANGUAGES, &[&lang])
}

pub fn get_languages_disabled(conn: &Connection) -> SalResult<Vec<String>> {
    let mut result = Vec::new();
    let mut stmt = conn.prepare(query_select::LANGUAGES_DISABLED)?;
    for row in stmt.query_map(&[], |row| row.get(0))? {
        let lang: String = row?;
        result.push(lang);
    }
    Ok(result)
}

pub fn get_languages_enabled(conn: &Connection) -> SalResult<Vec<String>> {
    let mut result = Vec::new();
    let mut stmt = conn.prepare(query_select::LANGUAGES_ENABLED)?;
    for row in stmt.query_map(&[], |row| row.get(0))? {
        let lang: String = row?;
        result.push(lang);
    }
    Ok(result)
}

pub fn disable_language(conn: &Connection, lang: &str) -> SalResult<i32> {
    conn.execute(query_insert::DISABLE_LANGUAGE, &[&lang])
}

pub fn enable_language(conn: &Connection, lang: &str) -> SalResult<(i32)> {
    conn.execute(query_insert::ENABLE_LANGUAGE, &[&lang])
}

pub fn get_genre_name(conn: &Connection, genre: &str) -> SalResultOption<String> {
    let mut stmt = conn.prepare(query_select::GENRE_NAME)?;
    for row in stmt.query_map(&[&genre], |row| (row.get(0)))? {
        let genre: String = row?;
        return Ok(Some(genre));
    }
    Ok(None)
}
