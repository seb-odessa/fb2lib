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
pub trait Save<T> {
    fn save(&mut self, conn: &Connection) -> result::Fb2Result<()>;
    fn task(&self) -> TASK;
    fn get_new_count(&self) -> usize;
    fn get_stored_count(&self) -> usize;
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
    AUTHOR,
    TITLE,
    SEQUENCE,
}


pub use sal::imp::reset_tables;
pub use sal::imp::get_connection;

pub use sal::imp::get_archive_sizes;
pub use sal::imp::register;
pub use sal::imp::validate;
pub use sal::imp::get_hash;

pub use sal::imp::insert_languages;
pub use sal::imp::select_languages;
pub use sal::imp::get_languages_disabled;
pub use sal::imp::get_languages_enabled;
pub use sal::imp::disable_language;
pub use sal::imp::enable_language;

pub use sal::imp::get_genre_codes;
pub use sal::imp::get_genres_disabled;
pub use sal::imp::get_genres_enabled;
pub use sal::imp::get_genre_groups_disabled;
pub use sal::imp::get_genre_groups_enabled;
pub use sal::imp::disable_genre;
pub use sal::imp::enable_genre;
pub use sal::imp::disable_genre_group;
pub use sal::imp::enable_genre_group;
pub use sal::imp::get_genre_codes_disabled;
pub use sal::imp::get_genre_codes_and_groups;

pub use sal::imp::get_archive_status;
pub use sal::imp::set_archive_started;
pub use sal::imp::set_archive_visited;
pub use sal::imp::set_archive_complete;
pub use sal::imp::set_archive_failure;

pub use sal::imp::insert_people;
pub use sal::imp::select_people;
pub use sal::imp::select_authors_joined;
pub use sal::imp::link_authors;
pub use sal::imp::unlink_authors;

pub use sal::imp::insert_titles;
pub use sal::imp::select_titles;
pub use sal::imp::select_titles_joined;
pub use sal::imp::link_titles;
pub use sal::imp::unlink_titles;

pub use sal::imp::insert_sequences;
pub use sal::imp::select_sequences;
pub use sal::imp::select_sequences_joined;
pub use sal::imp::link_sequences;
pub use sal::imp::unlink_sequences;

