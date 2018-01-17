mod imp;

mod query_create;
mod query_drop;
mod query_insert;
mod query_select;

use std::collections::HashMap;

pub type HashesByIdx = HashMap<i64, String>;

pub use sal::imp::cleanup_tables;
pub use sal::imp::insert_language;
pub use sal::imp::insert_ignored_language;

pub use sal::imp::get_connection;
pub use sal::imp::get_archive_sizes;
pub use sal::imp::register;
pub use sal::imp::validate;
pub use sal::imp::get_hash;



