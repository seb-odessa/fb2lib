mod imp;

mod query_drop;
mod query_create;
mod query_init;
mod query_insert;
mod query_select;
use std::collections::HashMap;

use rusqlite;
pub type HashesByIdx = HashMap<i64, String>;
pub type Connection = rusqlite::Connection;

#[derive(Debug)]
pub enum STATUS {
    COMPLETE,
    INCOMPLETE,
    IGNORE,
    FAILURE,
    UNKNOWN // Record not found in table
}

#[derive(Debug)]
pub enum TASK {
    LANGUAGE,
    GENRE,
    AUTHOR
}


pub use sal::imp::reset_tables;
pub use sal::imp::get_connection;

pub use sal::imp::get_archive_sizes;
pub use sal::imp::register;
pub use sal::imp::validate;
pub use sal::imp::get_hash;

pub use sal::imp::insert_language;
pub use sal::imp::get_languages_disabled;
pub use sal::imp::get_languages_enabled;
pub use sal::imp::disable_language;
pub use sal::imp::enable_language;

pub use sal::imp::get_genre_name;
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

pub use sal::imp::insert_people;

pub use sal::imp::get_archive_status;
pub use sal::imp::set_archive_complete;
pub use sal::imp::set_archive_incomplete;
pub use sal::imp::set_archive_ignore;
pub use sal::imp::set_archive_failure;

