use out;
use sal;
use tools;
use archive;
use filesystem;
use algorithm::{apply_to_xml, apply_to_file, apply};

use result::Fb2Result;
use result::Fb2Error;

use std::error::Error;
use std::fs::File;

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

pub fn db_init(db_file_name: &str) -> Fb2Result<()> {
    println!("db_init({})", db_file_name);
    sal::init_tables(db_file_name).map_err(into)
}

pub fn db_drop(db_file_name: &str) -> Fb2Result<()> {
    println!("db_drop({})", db_file_name);
    sal::drop_tables(db_file_name).map_err(into)
}

pub fn db_register(db_file_name: &str, torrent_name: &str) -> Fb2Result<()> {
    println!("db_register({}, {})", db_file_name, torrent_name);
    let metainfo = filesystem::load_torrent(torrent_name)?;
    println!("file name:     {}", &metainfo.get_file_name());
    println!("creation date: {}", &metainfo.get_creation_date());
    println!("info hash:     {}", &metainfo.get_info_hash());
    println!("total length:  {}", &metainfo.get_length());
    println!("piece length:  {}", &metainfo.get_piece_length());
    println!("piece count:   {}", &metainfo.get_piece_count());
    sal::register(db_file_name, metainfo).map_err(into)
}

pub fn db_load(db_file_name: &str, archive_name: &str) -> Fb2Result<()> {
    println!("db_load({}, {})", db_file_name, archive_name);
    Ok(())
}

pub fn db_check(db_file_name: &str, archive_name: &str) -> Fb2Result<()> {
    println!("db_check({}, {})", db_file_name, archive_name);
    filesystem::check_integrity(db_file_name, archive_name)
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
