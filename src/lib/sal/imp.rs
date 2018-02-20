use sal;
use result::{into, Fb2Result, Fb2Error};
use torrent::Metainfo;

use rusqlite;
use rustc_serialize::hex::ToHex;

use std::iter::FromIterator;
use std::collections::HashMap;
use std::collections::HashSet;

pub type Connection = rusqlite::Connection;

pub fn reset_tables(db_file_name: &str) -> Fb2Result<()> {
    let conn = Connection::open(db_file_name).map_err(into)?;
    // conn.execute(sal::query_drop::ARCHIVES, &[]).map_err(into)?;
    // conn.execute(sal::query_drop::PIECES, &[]).map_err(into)?;
    // conn.execute(sal::query_drop::LANGUAGES, &[]).map_err(into)?;
    // conn.execute(sal::query_drop::LANGUAGES_DISABLED, &[]).map_err(into)?;
    // conn.execute(sal::query_drop::LANGUAGES_ENABLED, &[]).map_err(into)?;
    // conn.execute_batch(sal::query_drop::FILTER_SUBSYSTEM).map_err(into)?;
    // conn.execute_batch(sal::query_drop::GENRE_SUBSYSTEM).map_err(into)?;
    // conn.execute_batch(sal::query_drop::PEOPLE_SUBSYSTEM).map_err(into)?;
    // conn.execute_batch(sal::query_drop::PROGRESS_SUBSYSTEM).map_err(into)?;
    conn.execute_batch(sal::query_drop::TITLES_SUBSYSTEM).map_err(into)?;
    conn.execute_batch(sal::query_drop::SEQUENCES_SUBSYSTEM).map_err(into)?;

    // conn.execute(sal::query_create::ARCHIVES, &[]).map_err(into)?;
    // conn.execute(sal::query_create::PIECES, &[]).map_err(into)?;
    // conn.execute(sal::query_create::LANGUAGES, &[]).map_err(into)?;
    // conn.execute(sal::query_create::LANGUAGES_AUTO, &[]).map_err(into)?;
    // conn.execute(sal::query_create::LANGUAGES_DISABLED, &[]).map_err(into)?;
    // conn.execute(sal::query_create::LANGUAGES_ENABLED, &[]).map_err(into)?;
    // conn.execute_batch(sal::query_create::FILTER_SUBSYSTEM).map_err(into)?;
    // conn.execute_batch(sal::query_init::FILTER_SUBSYSTEM).map_err(into)?;
    // conn.execute_batch(sal::query_create::GENRE_SUBSYSTEM).map_err(into)?;
    // conn.execute_batch(sal::query_init::INSERT_GENRES).map_err(into)?;
    // conn.execute_batch(sal::query_create::PEOPLE_SUBSYSTEM).map_err(into)?;
    // conn.execute_batch(sal::query_create::PROGRESS_SUBSYSTEM).map_err(into)?;
    conn.execute_batch(sal::query_create::TITLES_SUBSYSTEM).map_err(into)?;
    conn.execute_batch(sal::query_create::SEQUENCES_SUBSYSTEM).map_err(into)?;

    Ok(())
}

fn get_task_id(oper: sal::TASK) -> i64 {
    match oper {
        sal::TASK::LANGUAGE => 1,
        sal::TASK::GENRE => 2,
        sal::TASK::AUTHOR => 3,
        sal::TASK::TITLE => 4,
        sal::TASK::SEQUENCE => 5,
    }
}

fn get_status_type(code: i64) -> sal::STATUS {
    match code {
        1 => sal::STATUS::STARTED,
        2 => sal::STATUS::VISITED,
        3 => sal::STATUS::COMPLETE,
        5 => sal::STATUS::FAILURE,
        _ => sal::STATUS::UNKNOWN,
    }
}

fn get_status_id(code: sal::STATUS) -> i64 {
    match code {
        sal::STATUS::STARTED => 1,
        sal::STATUS::VISITED => 2,
        sal::STATUS::COMPLETE => 3,
        sal::STATUS::FAILURE => 4,
        sal::STATUS::UNKNOWN => 0,
    }
}

pub fn get_archive_status(conn: &Connection, archive: &str, oper: sal::TASK) -> Fb2Result<sal::STATUS> {
    let mut stmt = conn.prepare(sal::query_select::PROGRESS_STATUS).map_err(into)?;
    let rows = stmt.query_map(&[&archive, &get_task_id(oper)], |row| { row.get(0) })?;
    for row in rows {
        let status: i64 = row.map_err(into)?;
        return Ok(get_status_type(status))
    }
    Ok(sal::STATUS::UNKNOWN)
}

fn get_archive_id_by_name(conn: &Connection, archive: &str) -> Fb2Result<i64> {
    let mut stmt = conn.prepare(sal::query_select::ARCHIVE_ID_BY_NAME).map_err(into)?;
    let rows = stmt.query_map(&[&archive], |row| { row.get(0) })?;
    for row in rows {
        let id: i64 = row.map_err(into)?;
        return Ok(id)
    }
    Err(Fb2Error::Custom(format!("Archive {} not found in database", archive)))
}

fn set_archive_status(conn: &Connection, archive: &str, task: i64, status: i64) -> Fb2Result<()> {
let archive_id = get_archive_id_by_name(conn, archive)?;
    conn.execute(sal::query_insert::PROGRESS, &[&archive_id, &task, &status]).map_err(into)?;
    Ok(())
}

pub fn set_archive_started(conn: &Connection, archive: &str, oper: sal::TASK) -> Fb2Result<()> {
    set_archive_status(conn, archive, get_task_id(oper), get_status_id(sal::STATUS::STARTED))
}

pub fn set_archive_visited(conn: &Connection, archive: &str, oper: sal::TASK) -> Fb2Result<()> {
    set_archive_status(conn, archive, get_task_id(oper), get_status_id(sal::STATUS::VISITED))
}

pub fn set_archive_complete(conn: &Connection, archive: &str, oper: sal::TASK) -> Fb2Result<()> {
    set_archive_status(conn, archive, get_task_id(oper), get_status_id(sal::STATUS::COMPLETE))
}

pub fn set_archive_failure(conn: &Connection, archive: &str, oper: sal::TASK) -> Fb2Result<()> {
    set_archive_status(conn, archive, get_task_id(oper), get_status_id(sal::STATUS::FAILURE))
}

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
    let mut stmt = conn.prepare(sal::query_select::ARCH_SIZES_BY_NAME).map_err(into)?;
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

pub fn validate(conn: &Connection, id: i64, desc: &sal::HashesByIdx) -> Fb2Result<Option<i64>> {
    let mut stmt = conn.prepare(sal::query_select::INDEX_AND_HASH_BY_ARCH_ID).map_err(into)?;
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
    let mut stmt = conn.prepare(sal::query_select::HASH_BY_ARCH_ID_AND_INDEX).map_err(into)?;
    let rows = stmt.query_map(&[&id, &index], |row| (row.get(0))).map_err(into)?;
    for row in rows {
        let hash: String = row.map_err(into)?;
        return Ok(Some(hash));
    }
    Ok(None)
}

fn get_archive_id(conn: &Connection, metainfo: &Metainfo) -> Fb2Result<i64> {
    let mut stmt = conn.prepare(sal::query_select::ID_BY_HASH).map_err(into)?;
    let rows = stmt.query_map(&[&metainfo.get_info_hash()], |row| row.get(0)).map_err(into)?;
     for row in rows {
        let id = row.map_err(into)?;
        return Ok(id);

     }
    conn.execute(sal::query_insert::ARCHIVE, &[
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
        let mut stmt = tx.prepare(sal::query_insert::PIECE).map_err(into)?;
        let pieces: &[u8] = metainfo.info.pieces.as_ref();
        let mut index = 0;
        for sha1 in pieces.chunks(20) {
            stmt.execute(&[&archive_id, &index, &sha1.to_hex()]).map_err(into)?;
            index += 1;
        }
    }
    tx.commit().map_err(into)
}

pub fn get_languages_disabled(conn: &Connection) -> Fb2Result<Vec<String>> {
    let mut result = Vec::new();
    let mut stmt = conn.prepare(sal::query_select::LANGUAGES_DISABLED).map_err(into)?;
    for row in stmt.query_map(&[], |row| row.get(0)).map_err(into)? {
        let lang: String = row.map_err(into)?;
        result.push(lang);
    }
    Ok(result)
}

pub fn get_languages_enabled(conn: &Connection) -> Fb2Result<Vec<String>> {
    let mut result = Vec::new();
    let mut stmt = conn.prepare(sal::query_select::LANGUAGES_ENABLED).map_err(into)?;
    for row in stmt.query_map(&[], |row| row.get(0)).map_err(into)? {
        let lang: String = row.map_err(into)? ;
        result.push(lang);
    }
    Ok(result)
}

pub fn disable_language(conn: &Connection, lang: &str) -> Fb2Result<i32> {
    conn.execute(sal::query_insert::DISABLE_LANGUAGE, &[&lang]).map_err(into)
}

pub fn enable_language(conn: &Connection, lang: &str) -> Fb2Result<(i32)> {
    conn.execute(sal::query_insert::ENABLE_LANGUAGE, &[&lang]).map_err(into)
}

pub fn get_genre_codes(conn: &Connection) -> Fb2Result<Vec<String>> {
    let mut result = Vec::new();
    let mut stmt = conn.prepare(sal::query_select::GENRE_CODES).map_err(into)?;
    for row in stmt.query_map(&[], |row| row.get(0)).map_err(into)? {
        let code: String = row.map_err(into)? ;
        result.push(code);
    }
    Ok(result)
}

pub fn get_genres_disabled(conn: &Connection) -> Fb2Result<Vec<(String, String)>> {
    let mut result = Vec::new();
    let mut stmt = conn.prepare(sal::query_select::GENRES_DISABLED).map_err(into)?;
    for row in stmt.query_map(&[], |row| (row.get(0), row.get(1))).map_err(into)? {
        result.push(row.map_err(into)? as (String, String));
    }
    Ok(result)
}

pub fn get_genres_enabled(conn: &Connection) -> Fb2Result<Vec<(String, String)>> {
    let mut result = Vec::new();
    let mut stmt = conn.prepare(sal::query_select::GENRES_ENABLED).map_err(into)?;
    for row in stmt.query_map(&[], |row| (row.get(0), row.get(1))).map_err(into)? {
        result.push(row.map_err(into)? as (String, String));
    }
    Ok(result)
}

pub fn get_genre_groups_disabled(conn: &Connection) -> Fb2Result<Vec<String>> {
    let mut result = Vec::new();
    let mut stmt = conn.prepare(sal::query_select::GENRES_GROUPS_DISABLED).map_err(into)?;
    for row in stmt.query_map(&[], |row| row.get(0)).map_err(into)? {
        let group: String = row.map_err(into)? ;
        result.push(group);
    }
    Ok(result)
}

pub fn get_genre_groups_enabled(conn: &Connection) -> Fb2Result<Vec<String>> {
    let mut result = Vec::new();
    let mut stmt = conn.prepare(sal::query_select::GENRES_GROUPS_ENABLED).map_err(into)?;
    for row in stmt.query_map(&[], |row| row.get(0)).map_err(into)? {
        let group: String = row.map_err(into)? ;
        result.push(group);
    }
    Ok(result)
}

pub fn disable_genre(conn: &Connection, name: &str) -> Fb2Result<i32> {
    conn.execute(sal::query_insert::DISABLE_GENRE, &[&name]).map_err(into)
}

pub fn enable_genre(conn: &Connection, name: &str) -> Fb2Result<(i32)> {
    conn.execute(sal::query_insert::ENABLE_GENRE, &[&name]).map_err(into)
}

pub fn disable_genre_group(conn: &Connection, name: &str) -> Fb2Result<i32> {
    conn.execute(sal::query_insert::DISABLE_GENRE_GROUP, &[&name]).map_err(into)
}

pub fn enable_genre_group(conn: &Connection, name: &str) -> Fb2Result<(i32)> {
    conn.execute(sal::query_insert::ENABLE_GENRE_GROUP, &[&name]).map_err(into)
}

pub fn get_genre_codes_disabled(conn: &Connection) -> Fb2Result<Vec<String>> {
    let mut result = Vec::new();
    let mut stmt = conn.prepare(sal::query_select::GENRE_CODES_DISABLED).map_err(into)?;
    for row in stmt.query_map(&[], |row| row.get(0)).map_err(into)? {
        let group: String = row.map_err(into)? ;
        result.push(group);
    }
    Ok(result)
}

pub fn get_genre_codes_and_groups(conn: &Connection) -> Fb2Result<HashMap<String, String>> {
    let mut result = HashMap::new();
    let mut stmt = conn.prepare(sal::query_select::GENRES_CODES_AND_GROUPS).map_err(into)?;
    for row in stmt.query_map(&[], |row| (row.get(0), row.get(1))).map_err(into)? {
        let (code, group) = row.map_err(into)? as (String, String);
        result.insert(code, group);
    }
    Ok(result)
}

pub fn insert_people(conn: &Connection, authors: &HashSet<(String, String, String, String)>) -> Fb2Result<()> {
    let mut stmt = conn.prepare(sal::query_insert::PEOPLE).map_err(into)?;
    for author in authors {
        let &(ref first_name, ref middle_name, ref last_name, ref nick_name) = author;
        stmt.execute(&[first_name, middle_name, last_name, nick_name]).map_err(into)?;
    }
    Ok(())
}

pub fn select_people(conn: &Connection) -> Fb2Result<HashSet<(String, String, String, String)>> {
    let mut authors = HashSet::new();
    let mut stmt = conn.prepare(sal::query_select::PEOPLE).map_err(into)?;
    let rows = stmt.query_map(&[], |row| (row.get(0), row.get(1), row.get(2), row.get(3))).map_err(into)?;
    for row in rows {
        let author = row.map_err(into)? as (String,String,String,String);
        authors.insert(author);
    }
    Ok(authors)
}

fn insert_from_set(conn: &Connection, sql: &str, items: &HashSet<String>) -> Fb2Result<()> {
    for item in items {
        conn.execute(sql, &[item]).map_err(into)?;
    }
    Ok(())
}

pub fn insert_languages(conn: &Connection, langs: &HashSet<String>) -> Fb2Result<()> {    
    insert_from_set(conn, sal::query_insert::LANGUAGE, langs)
}

pub fn insert_titles(conn: &Connection, titles: &HashSet<String>) -> Fb2Result<()> {
    insert_from_set(conn, sal::query_insert::TITLES, titles)
}

pub fn insert_sequences(conn: &Connection, sequences: &HashSet<String>) -> Fb2Result<()> {
    insert_from_set(conn, sal::query_insert::SEQUENCES, sequences)
}

fn select_column(conn: &Connection, sql: &str, col_num: i32) -> Fb2Result<Vec<String>> {
    let mut result = Vec::new();
    let mut stmt = conn.prepare(sql).map_err(into)?;
    let rows = stmt.query_map(&[], |row| row.get(col_num)).map_err(into)?;
    for row in rows {
        let data: String = row.map_err(into)?;
        result.push(data);
    }
    Ok(result)
}

pub fn select_titles(conn: &Connection) -> Fb2Result<HashSet<String>> {
    let vector = select_column(conn, sal::query_select::TITLES, 0)?;
    Ok(HashSet::from_iter(vector))
}

pub fn select_languages(conn: &Connection) -> Fb2Result<HashSet<String>> {
    let vector = select_column(conn, sal::query_select::LANGUAGES, 0)?;
    Ok(HashSet::from_iter(vector))
}

pub fn select_sequences(conn: &Connection) -> Fb2Result<HashSet<String>> {
    let vector = select_column(conn, sal::query_select::SEQUENCES, 0)?;
    Ok(HashSet::from_iter(vector))
}