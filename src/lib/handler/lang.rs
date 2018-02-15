use sal;
use result::Fb2Result;
use algorithm::make_regex;

pub fn display(db_file_name: &str) -> Fb2Result<()> {
    println!("lang_display({})", db_file_name);
    let conn = sal::get_connection(db_file_name)?;
    println!("======== DISABLED ========");
    for lang in &sal::get_languages_disabled(&conn)? {
        print!("'{}' ", lang);
    }
    println!("");
    println!("======== ENABLED ========");
    for lang in &sal::get_languages_enabled(&conn)? {
        print!("'{}' ", lang);
    }
    println!("");
    Ok(())
}

pub fn enable(db_file_name: &str, lang: &str) -> Fb2Result<()> {
    println!("enable({}, {})", db_file_name, lang);
    let conn = sal::get_connection(db_file_name)?;
    let re = make_regex(lang)?;
    for lang in &sal::get_languages_disabled(&conn)? {
        if re.is_match(lang) {
            sal::enable_language(&conn, lang)?;
            println!("{} enabled", lang);
        }
    }
    Ok(())
}

pub fn disable(db_file_name: &str, lang: &str) -> Fb2Result<()> {
    println!("disable({}, {})", db_file_name, lang);
    let conn = sal::get_connection(db_file_name)?;
    let re = make_regex(lang)?;
    for lang in &sal::get_languages_enabled(&conn)? {
        if re.is_match(lang) {
            sal::disable_language(&conn, lang)?;
            println!("{} disabled", lang);
        }
    }
    Ok(())
}

