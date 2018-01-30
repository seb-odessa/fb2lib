use sal;
use tools;
use archive;
use result::Fb2Result;
use algorithm::{apply_and_collect, make_regex};

use std::sync::mpsc::channel;
use std::collections::HashSet;


fn extract_langs(db_file_name: &str, archive_name: &str) -> Fb2Result<Vec<String>> {
    println!("extract_langs({}, {})", db_file_name, archive_name);
    let zip = archive::open(archive_name)?;
    let (sender, receiver) = channel();
    apply_and_collect(zip, "*.fb2", sender, tools::into_fb2)?;
    let mut langs = HashSet::new();
    for fb2book in receiver.iter() {
        if let Some(fb2) = fb2book.ok() {
            langs.insert(fb2.get_book_lang());
        }
    }
    Ok(langs.into_iter().collect())
}

pub fn load(db_file_name: &str, archive_name: &str) -> Fb2Result<()> {
    println!("lang_load({}, {})", db_file_name, archive_name);
    let langs = extract_langs(db_file_name, archive_name)?;
    let conn = sal::get_connection(db_file_name)?;
    for lang in &langs {
        sal::insert_language(&conn, lang.to_lowercase().as_str().trim())?;
    }
    Ok(())
}

pub fn ls(db_file_name: &str, archive_name: &str) -> Fb2Result<()> {
    println!("lang::ls({}, {})", db_file_name, archive_name);
    for genre in &extract_langs(db_file_name, archive_name)? {
        println!("'{}'", genre);
    }
    Ok(())
}

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

