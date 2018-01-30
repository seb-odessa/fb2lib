mod imp;

mod query_drop;
mod query_create;
mod query_init;
mod query_insert;
mod query_select;
use std::collections::HashMap;

pub type HashesByIdx = HashMap<i64, String>;

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



