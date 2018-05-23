mod imp;

mod query_drop;
mod query_create;
mod query_init;
mod query_insert;
mod query_select;
mod query_delete;
mod query_update;

use result;
use rusqlite;
use std::collections::HashMap;

pub type HashesByIdx = HashMap<i64, String>;
pub type Connection = rusqlite::Connection;

pub trait Save {
    fn save(&mut self, conn: &Connection) -> result::Fb2Result<()>;
    fn task(&self) -> TASK;
    fn set_status(&self, conn: &Connection, archive: &str, status: STATUS) -> result::Fb2Result<()> {
        set_archive_status(conn, archive, get_task_id(self.task()), get_status_id(status))
    }
}

pub enum SUBSYSTEM {
    TORRENT,
    LANGUAGE,
    TITLES,
    SEQUENCES,
    DESCRIPTIONS,
    FILTER,
    GENRE,
    NAMES,
    AUTHORS,
    PROGRESS,
}

#[derive(Debug, Clone, Copy)]
pub enum STATUS {
    STARTED,
    VISITED,
    COMPLETE,
    FAILURE,
    UNKNOWN // Record not found in table
}

#[derive(Debug, Clone, Copy)]
pub enum TASK {
    LANGUAGE,
    GENRE,
    NAME,
    TITLE,
    SEQUENCE,
    DESC,
    AUTHOR,
}

pub use sal::imp::reset;
pub use sal::imp::get_connection;

pub use sal::imp::load_archives;
pub use sal::imp::get_archive_sizes;
pub use sal::imp::get_archive_id_by_name;
pub use sal::imp::register;
pub use sal::imp::validate;
pub use sal::imp::get_piece_hash;

pub use sal::imp::insert_languages;
pub use sal::imp::load_languages;
pub use sal::imp::get_languages_disabled;
pub use sal::imp::get_languages_enabled;
pub use sal::imp::disable_language;
pub use sal::imp::enable_language;

pub use sal::imp::get_genres_disabled;
pub use sal::imp::get_genres_enabled;
pub use sal::imp::get_genre_groups_disabled;
pub use sal::imp::get_genre_groups_enabled;
pub use sal::imp::disable_genre;
pub use sal::imp::enable_genre;
pub use sal::imp::disable_genre_group;
pub use sal::imp::enable_genre_group;
pub use sal::imp::get_genre_codes_disabled;

pub use sal::imp::get_archive_status;
pub use sal::imp::set_archive_status;
pub use sal::imp::get_task_id;
pub use sal::imp::get_status_id;

//pub use sal::imp::insert_people;
//pub use sal::imp::select_people;
pub use sal::imp::select_authors_joined;
pub use sal::imp::link_authors;
pub use sal::imp::unlink_authors;

pub use sal::imp::insert_titles;
pub use sal::imp::load_titles;
pub use sal::imp::select_titles_joined;
pub use sal::imp::link_titles;
pub use sal::imp::unlink_titles;

pub use sal::imp::insert_sequences;
pub use sal::imp::load_sequences;
pub use sal::imp::select_sequences_joined;
pub use sal::imp::link_sequences;
pub use sal::imp::unlink_sequences;

pub use sal::imp::load_people;
pub use sal::imp::save_people;

//pub use sal::query_select::LOAD_ID_BY_GENRE;
//pub use sal::query_select::LOAD_ID_BY_LANG;
//pub use sal::query_select::LOAD_ID_BY_TITLE;
//pub use sal::query_select::LOAD_ID_BY_SEQUENCE;
//pub use sal::imp::load_id_by_name;
//pub use sal::imp::load_hash_to_id;

pub use sal::imp::save_books;
pub use sal::imp::load_known_books;
pub use sal::imp::load_books;

pub use sal::imp::load_names;
pub use sal::imp::save_names;
pub use sal::imp::load_id_by_names;



//pub use sal::imp::load_genres;
