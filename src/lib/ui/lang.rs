use sal;
use tools;
use archive;
use result::into;
use result::Fb2Result;
use algorithm::{apply_and_collect, make_regex};

//use clap::{App, Arg, SubCommand, ArgMatches};
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
    let mut conn = sal::get_connection(db_file_name).map_err(into)?;
    let tx = conn.transaction().map_err(into)?;
    for lang in &langs {
        sal::insert_language(&tx, lang.to_lowercase().as_str().trim()).map_err(into)?;
    }
    tx.commit().map_err(into)
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
    print!("disabled languages: ");
    for lang in &sal::get_languages_disabled(&conn).map_err(into)? {
        print!("'{}' ", lang);
    }
    println!("");
    print!("enabled languages: ");
    for lang in &sal::get_languages_enabled(&conn).map_err(into)? {
        print!("'{}' ", lang);
    }
    println!("");
    Ok(())
}

pub fn enable(db_file_name: &str, lang: &str) -> Fb2Result<()> {
    println!("lang_enable({}, {})", db_file_name, lang);
    let conn = sal::get_connection(db_file_name).map_err(into)?;
    let re = make_regex(lang)?;
    for lang in &sal::get_languages_disabled(&conn).map_err(into)? {
        if re.is_match(lang) {
            sal::enable_language(&conn, lang).map_err(into)?;
            println!("{} enabled", lang);
        }
    }
    Ok(())
}

pub fn disable(db_file_name: &str, lang: &str) -> Fb2Result<()> {
    println!("lang_disable({}, {})", db_file_name, lang);
    let conn = sal::get_connection(db_file_name).map_err(into)?;
    let re = make_regex(lang)?;
    for lang in &sal::get_languages_enabled(&conn).map_err(into)? {
        if re.is_match(lang) {
            sal::disable_language(&conn, lang).map_err(into)?;
            println!("{} disabled", lang);
        }
    }
    Ok(())
}

