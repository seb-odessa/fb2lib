use sal;
use result::Fb2Result;

pub fn reset(db_file_name: &str) -> Fb2Result<()> {
    println!("reset({})", db_file_name);
    sal::reset_tables(db_file_name)
}
