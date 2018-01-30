use result::{into, Fb2Result};
use sal::query_create;
use sal::query_init;
use sal::query_drop;
use sal::query_insert;
use sal::query_select;
use sal::HashesByIdx;
use torrent::Metainfo;

use rusqlite;
use rusqlite::{Connection, Transaction};
use rustc_serialize::hex::ToHex;


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

pub fn get_archive_sizes(conn: &Connection, name: &str) -> Fb2Result<Option<ArchiveSizes>> {
    let mut stmt = conn.prepare(query_select::ARCH_SIZES_BY_NAME).map_err(into)?;
    let rows = stmt.query_map(&[&name], |row| {
        ArchiveSizes::new(row.get(0), row.get(1), row.get(2), row.get(3))
    })?;
    for row in rows {
        let arch = row.map_err(into)?;
        println!("Found {:?}", arch);
        return Ok(Some(arch)); // it is ok due to name column is unique
    }
    Ok(None)
}

pub fn validate(conn: &Connection, id: i64, desc: &HashesByIdx) -> Fb2Result<Option<i64>> {
    let mut stmt = conn.prepare(query_select::INDEX_AND_HASH_BY_ARCH_ID).map_err(into)?;
    let rows = stmt.query_map(&[&id], |row| (row.get(0), row.get(1))).map_err(into)?;
    for row in rows {
        let (index, hash): (i64, String) = row.map_err(into)?;
        if hash != desc[&index] {
            return Ok(Some(index));
        }
    }
    Ok(None)
}

pub fn get_hash(conn: &Connection, id: i64, index: i64) -> Fb2Result<Option<String>> {
    let mut stmt = conn.prepare(query_select::HASH_BY_ARCH_ID_AND_INDEX).map_err(into)?;
    let rows = stmt.query_map(&[&id, &index], |row| (row.get(0))).map_err(into)?;
    for row in rows {
        let hash: String = row.map_err(into)?;
        return Ok(Some(hash));
    }
    Ok(None)
}

fn get_archive_id(conn: &Connection, metainfo: &Metainfo) -> Fb2Result<i64> {
    let mut stmt = conn.prepare(query_select::ID_BY_HASH).map_err(into)?;
    let rows = stmt.query_map(&[&metainfo.get_info_hash()], |row| row.get(0)).map_err(into)?;
     for row in rows {
        let id = row.map_err(into)?;
        return Ok(id);

     }
    conn.execute(query_insert::ARCHIVE, &[
        &metainfo.get_file_name(),
        &metainfo.get_creation_date(),
        &metainfo.get_info_hash(),
        &(metainfo.get_length() as i64),
        &(metainfo.get_piece_length() as i64),
        &(metainfo.get_piece_count() as i64),
    ]).map_err(into)?;
    Ok(conn.last_insert_rowid())

 }

pub fn register(db_file_name: &str, metainfo: Metainfo) -> Fb2Result<()> {
    let mut conn = Connection::open(db_file_name)?;
    let archive_id = get_archive_id(&conn, &metainfo)?;
    let tx = conn.transaction()?;
    {
        let mut stmt = tx.prepare(query_insert::PIECE).map_err(into)?;
        let pieces: &[u8] = metainfo.info.pieces.as_ref();
        let mut index = 0;
        for sha1 in pieces.chunks(20) {
            stmt.execute(&[&archive_id, &index, &sha1.to_hex()]).map_err(into)?;
            index += 1;
        }
    }
    tx.commit().map_err(into)
}

pub fn reset_tables(db_file_name: &str) -> Fb2Result<()> {
    let conn = Connection::open(db_file_name).map_err(into)?;
    // conn.execute(query_drop::ARCHIVES, &[]).map_err(into)?;
    // conn.execute(query_drop::PIECES, &[]).map_err(into)?;
    // conn.execute(query_drop::LANGUAGES, &[]).map_err(into)?;
     conn.execute(query_drop::FILTERS, &[]).map_err(into)?;
    // conn.execute(query_drop::FILTERS_DEF, &[]).map_err(into)?;
    // conn.execute(query_drop::LANGUAGES_DISABLED, &[]).map_err(into)?;
    // conn.execute(query_drop::LANGUAGES_ENABLED, &[]).map_err(into)?;
    conn.execute_batch(query_drop::GENRE_SUBSYSTEM).map_err(into)?;

    // conn.execute(query_create::ARCHIVES, &[]).map_err(into)?;
    // conn.execute(query_create::PIECES, &[]).map_err(into)?;
    // conn.execute(query_create::LANGUAGES, &[]).map_err(into)?;
    // conn.execute(query_create::LANGUAGES_AUTO, &[]).map_err(into)?;
    conn.execute(query_create::FILTERS, &[]).map_err(into)?;
    // conn.execute(query_create::FILTERS_DEF, &[]).map_err(into)?;
    // conn.execute(query_create::LANGUAGES_DISABLED, &[]).map_err(into)?;
    // conn.execute(query_create::LANGUAGES_ENABLED, &[]).map_err(into)?;
    conn.execute_batch(query_create::GENRE_SUBSYSTEM).map_err(into)?;
    conn.execute_batch(query_init::INSERT_GENRES).map_err(into)?;
    conn.execute_batch(query_init::INSERT_FILTER_TYPES).map_err(into)?;

    Ok(())
}

pub fn insert_language(conn: &Connection, lang: &str) -> Fb2Result<i32> {
    conn.execute(query_insert::LANGUAGES, &[&lang]).map_err(into)
}

pub fn get_languages_disabled(conn: &Connection) -> Fb2Result<Vec<String>> {
    let mut result = Vec::new();
    let mut stmt = conn.prepare(query_select::LANGUAGES_DISABLED).map_err(into)?;
    for row in stmt.query_map(&[], |row| row.get(0)).map_err(into)? {
        let lang: String = row.map_err(into)?;
        result.push(lang);
    }
    Ok(result)
}

pub fn get_languages_enabled(conn: &Connection) -> Fb2Result<Vec<String>> {
    let mut result = Vec::new();
    let mut stmt = conn.prepare(query_select::LANGUAGES_ENABLED).map_err(into)?;
    for row in stmt.query_map(&[], |row| row.get(0)).map_err(into)? {
        let lang: String = row.map_err(into)? ;
        result.push(lang);
    }
    Ok(result)
}

pub fn disable_language(conn: &Connection, lang: &str) -> Fb2Result<i32> {
    conn.execute(query_insert::DISABLE_LANGUAGE, &[&lang]).map_err(into)
}

pub fn enable_language(conn: &Connection, lang: &str) -> Fb2Result<(i32)> {
    conn.execute(query_insert::ENABLE_LANGUAGE, &[&lang]).map_err(into)
}

pub fn get_genre_name(conn: &Connection, genre: &str) -> Fb2Result<Option<(i32, String)>> {
    let mut stmt = conn.prepare(query_select::GENRE_NAME).map_err(into)?;
    for row in stmt.query_map(&[&genre], |row| (row.get(0), row.get(1))).map_err(into)? {
        return Ok(Some(row.map_err(into)? as (i32, String)));
    }
    Ok(None)
}

pub fn get_genres_disabled(conn: &Connection) -> Fb2Result<Vec<(String, String)>> {
    let mut result = Vec::new();
    let mut stmt = conn.prepare(query_select::GENRES_DISABLED).map_err(into)?;
    for row in stmt.query_map(&[], |row| (row.get(0), row.get(1))).map_err(into)? {
        result.push(row.map_err(into)? as (String, String));
    }
    Ok(result)
}

pub fn get_genres_enabled(conn: &Connection) -> Fb2Result<Vec<(String, String)>> {
    let mut result = Vec::new();
    let mut stmt = conn.prepare(query_select::GENRES_ENABLED).map_err(into)?;
    for row in stmt.query_map(&[], |row| (row.get(0), row.get(1))).map_err(into)? {
        result.push(row.map_err(into)? as (String, String));
    }
    Ok(result)
}