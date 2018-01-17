use out;
use sal;
use tools;
use archive;
use filesystem;
use algorithm::{apply_to_xml, apply_to_file, apply, apply_and_collect};

use result::Fb2Result;
use result::Fb2Error;

use std::error::Error;
use std::fs::File;
use std::sync::mpsc::channel;
use std::collections::HashSet;

fn into<F: Error>(e: F) -> Fb2Error {
    Fb2Error::Custom(e.description().to_string())
}

pub fn do_ls(archive_name: &str) -> Fb2Result<()> {
    let mut zip = archive::open(archive_name)?;
    for i in 0..zip.len() {
        let file = zip.by_index(i)?;
        out::file_info(&file);
    }
    Ok(())
}

pub fn show_xml(archive_name: &str, file_name: &str) -> Fb2Result<()> {
    let zip = archive::open(archive_name)?;
    apply(zip, file_name, out::xml)
}

pub fn show_fb2(archive_name: &str, file_name: &str) -> Fb2Result<()> {
    let zip = archive::open(archive_name)?;
    apply(zip, file_name, out::fb2)
}

pub fn show_inf(archive_name: &str, file_name: &str) -> Fb2Result<()> {
    let zip = archive::open(archive_name)?;
    apply(zip, file_name, out::info)
}

pub fn show_zip(archive_name: &str, file_name: &str) -> Fb2Result<()> {
    let zip = archive::open(archive_name)?;
    apply_to_file(zip, file_name, out::zip_info)
}

pub fn db_cleanup(db_file_name: &str) -> Fb2Result<()> {
    println!("db_cleanup({})", db_file_name);
    sal::cleanup_tables(db_file_name).map_err(into)
}

pub fn torrent_load(db_file_name: &str, torrent_name: &str) -> Fb2Result<()> {
    println!("torrent_load({}, {})", db_file_name, torrent_name);
    let metainfo = filesystem::load_torrent(torrent_name)?;
    println!("file name:     {}", &metainfo.get_file_name());
    println!("creation date: {}", &metainfo.get_creation_date());
    println!("info hash:     {}", &metainfo.get_info_hash());
    println!("total length:  {}", &metainfo.get_length());
    println!("piece length:  {}", &metainfo.get_piece_length());
    println!("piece count:   {}", &metainfo.get_piece_count());
    sal::register(db_file_name, metainfo).map_err(into)
}

pub fn torrent_check(db_file_name: &str, archive_name: &str) -> Fb2Result<()> {
    println!("torrent_check({}, {})", db_file_name, archive_name);
    filesystem::check_integrity(db_file_name, archive_name)
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

pub fn do_parse(file_name: &str) -> Fb2Result<()> {
    let mut file = File::open(file_name).map_err(into)?;
    let xml = archive::load_header(&mut file)?;
    let fb2 = tools::into_utf8(xml).and_then(tools::into_fb2)?;
    println!("{}", fb2);
    Ok(())
}

pub fn do_check(archive_name: &str, quiet: bool) -> Fb2Result<()> {
    use std::io;
    use std::io::Write;
    let zip = archive::open(archive_name)?;
    let count = zip.len();
    let mut succ = 0;
    let mut curr = 0;
    if !quiet {
        print!("Progress:   %");
    }
    apply_to_xml(zip, "*", |file_name, xml| {
        match tools::into_fb2(xml) {
            Ok(_) => succ += 1,
            Err(_) => {
                if !quiet {
                    println!();
                }
                println!(
                    "./fb2lib {} show xml {} > {}",
                    archive_name,
                    &file_name,
                    &file_name
                )
            }
        }
        if !quiet {
            curr += 1;
            print!("\rProgress: {:3}%", 100 * (1 + curr) / count);
            io::stdout().flush().unwrap();
        }
    })?;
    if !quiet {
        println!("\nSucceeded {}/{} ({}%)", succ, count, 100 * succ / count);
    }
    Ok(())
}
