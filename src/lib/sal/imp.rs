use sal;
use tools;
use result::{into, Fb2Result, Fb2Error};
use torrent::Metainfo;
use types::BookDescription;
use types::Archive;
use types::Sizes;
use fb2parser::FictionBook;

use rusqlite;
use rusqlite::DatabaseName;
pub use rusqlite::Connection;
use crypto::sha1::Sha1;
use crypto::digest::Digest;
use bincode::{serialize, deserialize};
use rustc_serialize::hex::ToHex;

use std::hash::Hash;
use std::fmt::Debug;
use std::default::Default;
use std::iter::FromIterator;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::{Read, Write, Seek, SeekFrom};

pub fn reset(db_file_name: &str, system: sal::SUBSYSTEM) -> Fb2Result<()> {
    let conn = Connection::open(db_file_name).map_err(into)?;
    match system {
        sal::SUBSYSTEM::TORRENT => {
            conn.execute_batch(sal::query_create::TORRENTS_SUBSYSTEM).map_err(into)?;
        },
        sal::SUBSYSTEM::PROGRESS => {
            conn.execute_batch(sal::query_create::PROGRESS_SUBSYSTEM).map_err(into)?;
        },
        sal::SUBSYSTEM::FILTER => {
            conn.execute_batch(sal::query_create::FILTER_SUBSYSTEM).map_err(into)?;
        },
        sal::SUBSYSTEM::LANGUAGE => {
            conn.execute_batch(sal::query_create::LANGUAGE_SUBSYSTEM).map_err(into)?;
        },
        sal::SUBSYSTEM::GENRE => {
            conn.execute_batch(sal::query_create::GENRE_SUBSYSTEM).map_err(into)?;
            conn.execute_batch(sal::query_init::INSERT_GENRES).map_err(into)?;
        },

        sal::SUBSYSTEM::VERSION => {
            conn.execute_batch(sal::query_drop::VERSION_SUBSYSTEM).map_err(into)?;
            conn.execute_batch(sal::query_create::VERSION_SUBSYSTEM).map_err(into)?;
        },
        sal::SUBSYSTEM::TITLES => {
            conn.execute_batch(sal::query_drop::TITLES_SUBSYSTEM).map_err(into)?;
            conn.execute_batch(sal::query_create::TITLES_SUBSYSTEM).map_err(into)?;
        },
        sal::SUBSYSTEM::SEQUENCES => {
            conn.execute_batch(sal::query_drop::SEQUENCES_SUBSYSTEM).map_err(into)?;
            conn.execute_batch(sal::query_create::SEQUENCES_SUBSYSTEM).map_err(into)?;
        },
        sal::SUBSYSTEM::PEOPLE => {
            conn.execute_batch(sal::query_drop::PEOPLE_SUBSYSTEM).map_err(into)?;
            conn.execute_batch(sal::query_create::PEOPLE_SUBSYSTEM).map_err(into)?;
        },
        sal::SUBSYSTEM::DESCRIPTIONS => {
            conn.execute_batch(sal::query_drop::DESC_SUBSYSTEM).map_err(into)?;
            conn.execute_batch(sal::query_create::DESC_SUBSYSTEM).map_err(into)?;
        }
    }
    Ok(())
}

pub fn get_task_id(oper: sal::TASK) -> i64 {
    match oper {
        sal::TASK::UNDEFINED => 0,
        sal::TASK::LANGUAGE => 1,
        sal::TASK::GENRE => 2,
        sal::TASK::NAME => 3,
        sal::TASK::TITLE => 4,
        sal::TASK::SEQUENCE => 5,
        sal::TASK::DESC => 6,
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

pub fn get_status_id(code: sal::STATUS) -> i64 {
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

pub fn get_archive_id_by_name(conn: &Connection, archive: &str) -> Fb2Result<i64> {
    let mut stmt = conn.prepare(sal::query_select::ARCHIVE_ID_BY_NAME).map_err(into)?;
    let rows = stmt.query_map(&[&archive], |row| { row.get(0) })?;
    for row in rows {
        let id: i64 = row.map_err(into)?;
        return Ok(id)
    }
    Err(Fb2Error::Custom(format!("Archive {} not found in database", archive)))
}

pub fn set_archive_status(conn: &Connection, archive: &str, task: i64, status: i64) -> Fb2Result<()> {
    let archive_id = get_archive_id_by_name(conn, archive)?;
    conn.execute(sal::query_insert::PROGRESS, &[&archive_id, &task, &status]).map_err(into)?;
    Ok(())
}

pub fn get_connection(db_file_name: &str) -> Fb2Result<Connection> {
    Connection::open(db_file_name).map_err(into)
}

pub fn get_archive_sizes(conn: &Connection, name: &str) -> Fb2Result<Option<Sizes>> {
    let mut stmt = conn.prepare(sal::query_select::ARCH_SIZES_BY_NAME).map_err(into)?;
    let rows = stmt.query_map(&[&name], |row| {
        Sizes::new(row.get(0), row.get(1), row.get(2), row.get(3))
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

pub fn get_piece_hash(conn: &Connection, id: i64, index: i64) -> Fb2Result<Option<String>> {
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

pub fn load_people(conn: &Connection) -> Fb2Result<HashMap<(String, String, String, String), i64>> {
    let mut map = HashMap::new();
    let mut stmt = conn.prepare(sal::query_select::LOAD_ID_BY_NAME).map_err(into)?;
    let rows = stmt.query_map(&[], |row| (row.get(0), row.get(1), row.get(2), row.get(3), row.get(4))).map_err(into)?;
    for row in rows {
        let author = row.map_err(into)? as (i64, String,String,String,String);
        map.insert((author.1, author.2, author.3, author.4), author.0);
    }
    Ok(map)
}

pub fn load_id_by_name(conn: &Connection, sql: &str) -> Fb2Result<HashMap<String, i64>> {
    let mut map = HashMap::new();
    let mut stmt = conn.prepare(sql).map_err(into)?;
    let rows = stmt.query_map(&[], |row| (row.get(0), row.get(1))).map_err(into)?;
    for row in rows {
        let tuple = row.map_err(into)? as (i64, String);
        map.insert(tuple.1, tuple.0);
    }
    Ok(map)
}

pub fn load_hash_to_id(conn: &Connection, sql: &str) -> Fb2Result<HashMap<u64, i64>> {
    let mut map = HashMap::new();
    let mut stmt = conn.prepare(sql).map_err(into)?;
    let rows = stmt.query_map(&[], |row| (row.get(0), row.get(1))).map_err(into)?;
    for row in rows {
        let (id, value) = row.map_err(into)? as (i64, String);
        map.insert(tools::get_hash(&value), id);
    }
    Ok(map)
}

fn insert_from_set(conn: &Connection, sql: &str, items: &HashSet<String>) -> Fb2Result<()> {
    let mut stmt = conn.prepare(sql).map_err(into)?;
    for item in items {
        match stmt.execute(&[item]).map_err(into) {
            Ok(_) => {},
            Err(e) => {println!("\n'{}' -> {}", item, e); return Err(e); }
        }
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

pub fn save_names(conn: &Connection, names: &HashSet<String>) -> Fb2Result<()> {
    insert_from_set(conn, sal::query_insert::NAMES, names)
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

pub fn load_titles(conn: &Connection) -> Fb2Result<HashSet<String>> {
    let vector = select_column(conn, sal::query_select::TITLES, 0)?;
    Ok(HashSet::from_iter(vector))
}

pub fn select_languages(conn: &Connection) -> Fb2Result<HashSet<String>> {
    let vector = select_column(conn, sal::query_select::LANGUAGES, 0)?;
    Ok(HashSet::from_iter(vector))
}

pub fn load_sequences(conn: &Connection) -> Fb2Result<HashSet<String>> {
    let vector = select_column(conn, sal::query_select::SEQUENCES, 0)?;
    Ok(HashSet::from_iter(vector))
}

pub fn select_authors_joined(conn: &Connection) -> Fb2Result<Vec<(i64, String, String)>> {
    let mut result = Vec::new();
    let mut stmt = conn.prepare(sal::query_select::AUTHORS_JOINED).map_err(into)?;
    let rows = stmt.query_map(&[], |row| (row.get(0), row.get(1), row.get(2))).map_err(into)?;
    for row in rows {
        let record = row.map_err(into)?;
        result.push(record);
    }
    Ok(result)
}

pub fn select_titles_joined(conn: &Connection) -> Fb2Result<Vec<(i64, String, String)>> {
    let mut result = Vec::new();
    let mut stmt = conn.prepare(sal::query_select::TITLES_JOINED).map_err(into)?;
    let rows = stmt.query_map(&[], |row| (row.get(0), row.get(1), row.get(2))).map_err(into)?;
    for row in rows {
        let record = row.map_err(into)?;
        result.push(record);
    }
    Ok(result)
}

pub fn select_sequences_joined(conn: &Connection) -> Fb2Result<Vec<(i64, String, String)>> {
    let mut result = Vec::new();
    let mut stmt = conn.prepare(sal::query_select::SEQUENCES_JOINED).map_err(into)?;
    let rows = stmt.query_map(&[], |row| (row.get(0), row.get(1), row.get(2))).map_err(into)?;
    for row in rows {
        let record = row.map_err(into)?;
        result.push(record);
    }
    Ok(result)
}

pub fn link_authors(conn: &Connection, src: i64, dst: i64) -> Fb2Result<i32> {
    conn.execute(sal::query_insert::AUTHOR_LINK, &[&src, &dst]).map_err(into)
}

pub fn link_titles(conn: &Connection, src: i64, dst: i64) -> Fb2Result<i32> {
    conn.execute(sal::query_insert::TITLE_LINK, &[&src, &dst]).map_err(into)
}

pub fn link_sequences(conn: &Connection, src: i64, dst: i64) -> Fb2Result<i32> {
    conn.execute(sal::query_insert::SEQUENCES_LINK, &[&src, &dst]).map_err(into)
}

pub fn unlink_authors(conn: &Connection, src: i64, dst: i64) -> Fb2Result<i32> {
    conn.execute(sal::query_delete::AUTHOR_LINK, &[&src, &dst]).map_err(into)
}

pub fn unlink_titles(conn: &Connection, src: i64, dst: i64) -> Fb2Result<i32> {
    conn.execute(sal::query_delete::TITLE_LINK, &[&src, &dst]).map_err(into)
}

pub fn unlink_sequences(conn: &Connection, src: i64, dst: i64) -> Fb2Result<i32> {
    conn.execute(sal::query_delete::SEQUENCES_LINK, &[&src, &dst]).map_err(into)
}
//================== Books ==================
pub fn save_books(conn: &Connection, descriptions: &Vec<BookDescription>) -> Fb2Result<()> {
    let mut stmt = conn.prepare(sal::query_insert::BOOK).map_err(into)?;
    for desc in descriptions {
        stmt.execute_named(&[
            (":archive_id", &desc.arch),
            (":file_name", &desc.file.name),
            (":compression_method", &desc.file.compression_method),
            (":compressed_size", &desc.file.compressed_size),
            (":original_size", &desc.file.original_size),
            (":src32", &desc.file.src32),
            (":offset", &desc.file.offset),
            (":size", &desc.blob.size),
            (":description", &Some(desc.blob.data.clone())),
            (":sha1", &desc.blob.sha1),
        ])?;
    }
    Ok(())
}

pub fn load_known_books(conn: &Connection) -> Fb2Result<HashSet<String>> {
    let mut result = HashSet::new();
    let mut stmt = conn.prepare(sal::query_select::BOOKS_SHA1).map_err(into)?;
    let mut rows = stmt.query(&[]).map_err(into)?;
    while let Some(row) = rows.next() {
        if let Some(row) = row.ok() {
            result.insert(row.get(0));
        }
    }
    Ok(result)
}

pub fn load_books(conn: &Connection, archive_id: i64) -> Fb2Result<VecDeque<FictionBook>> {
    let mut result = VecDeque::new();
    let mut stmt = conn.prepare(sal::query_select::BOOKS_IN_ARCHIVE).map_err(into)?;
    let mut rows = stmt.query(&[&archive_id]).map_err(into)?;
    while let Some(row) = rows.next() {
        if let Some(row) = row.ok() {
            let bytes: Vec<u8> = row.get(0);
            if let Some(book) = FictionBook::load(&bytes) {
                result.push_back(book);
            }
        }
    }
    Ok(result)
}
//================== Archives ==================
pub fn load_archives(conn: &Connection) -> Fb2Result<VecDeque<Archive>> {
    let mut result = VecDeque::new();
    let mut stmt = conn.prepare(sal::query_select::ARCHIVES).map_err(into)?;
    let mut rows = stmt.query(&[]).map_err(into)?;
    while let Some(row) = rows.next() {
        if let Some(row) = row.ok() {
            result.push_back(Archive::new(row.get(0), row.get(1)))
        }
    }
    Ok(result)
}



fn make_index_by_name(stmt: &rusqlite::Statement)->Fb2Result<HashMap<String, i32>> {
    let mut result = HashMap::new();
    let columns = stmt.column_names();
    for column_name in &columns {
        let column_index = stmt.column_index(column_name).map_err(into)?;
        result.insert(String::from(*column_name), column_index);
    }
    Ok(result)
}

pub fn load_names(conn: &Connection) -> Fb2Result<HashSet<String>> {
    let mut names = HashSet::new();
    let mut stmt = conn.prepare(sal::query_select::NAMES).map_err(into)?;
    let mut rows = stmt.query(&[]).map_err(into)?;
    while let Some(result) = rows.next() {
        let row = result?;
        names.insert(row.get(0));
    }
    Ok(names)
}

pub fn load_id_by_names(conn: &Connection) -> Fb2Result<HashMap<String, i64>> {
    let mut names = HashMap::new();
    let mut stmt = conn.prepare(sal::query_select::ID_BY_NAMES).map_err(into)?;
    let mut rows = stmt.query(&[]).map_err(into)?;
    while let Some(result) = rows.next() {
        let row = result?;
        names.insert(row.get(0), row.get(1));
    }
    Ok(names)
}