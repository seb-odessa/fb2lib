use sal;
use tools;
use archive;
use algorithm::{apply_and_collect};

use result::Fb2Result;
use result::Fb2Error;

use std::error::Error;
use std::sync::mpsc::channel;
use std::collections::HashSet;

fn into<F: Error>(e: F) -> Fb2Error {
    Fb2Error::Custom(e.description().to_string())
}

pub fn extract_langs(db_file_name: &str, archive_name: &str) -> Fb2Result<Vec<String>> {
    println!("extract_langs({}, {})", db_file_name, archive_name);
    let zip = archive::open(archive_name)?;
    let (sender, receiver) = channel();
    apply_and_collect(zip, "*.fb2", sender, tools::into_fb2)?;
    let mut langs = HashSet::new();
    for fb2book in receiver.iter() {
        langs.insert(fb2book?.get_book_lang());
    }
    Ok(langs.into_iter().collect())
}

pub fn lang_load(db_file_name: &str, archive_name: &str) -> Fb2Result<()> {
    println!("lang_load({}, {})", db_file_name, archive_name);
    let langs = extract_langs(db_file_name, archive_name)?;
    let mut conn = sal::get_connection(db_file_name).map_err(into)?;
    let tx = conn.transaction().map_err(into)?;
    for lang in &langs {
        sal::insert_language(&tx, lang).map_err(into)?;
    }
    tx.commit().map_err(into)
}

pub fn lang_show(db_file_name: &str, archive_name: &str) -> Fb2Result<()> {
    println!("lang_show({}, {})", db_file_name, archive_name);
    let langs = extract_langs(db_file_name, archive_name)?;
    for lang in &langs {
        println!("'{}'", lang);
    }
    Ok(())
}

pub fn lang_ignore(db_file_name: &str, language: &str) -> Fb2Result<()> {
    println!("lang_ignore({}, {})", db_file_name, language);
    let conn = sal::get_connection(db_file_name).map_err(into)?;
    sal::insert_ignored_language(&conn, &String::from(language)).map_err(into)?;
    Ok(())
}

