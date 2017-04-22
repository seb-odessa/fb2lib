extern crate rusqlite;
extern crate time;

use rusqlite::{Connection,Result};
use types::{Registrable};

#[allow(dead_code)]
pub fn register(conn: &Connection, data: &Registrable) -> Result<(i32)>
{
    match data
    {
        &Registrable::Container(ref data) =>
        {
            let sql = "INSERT INTO containers (path, name, md5) VALUES (?1, ?2, ?3)";
            conn.execute(sql, &[&data.path, &data.name, &data.md5])
        },
        &Registrable::File(ref data) =>
        {
            let sql = "INSERT INTO files (container, path, name, md5) VALUES (?1, ?2, ?3)";
            conn.execute(sql, &[&data.container, &data.path, &data.name, &data.md5])
        }
    }
}
