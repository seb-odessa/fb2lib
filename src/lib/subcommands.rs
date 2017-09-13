extern crate std;
extern crate zip;

use result::Fb2Result;
use std::io::Read;

fn do_open_archive(archive_name: &str) -> Fb2Result<zip::ZipArchive<std::fs::File>> {
    let file = std::fs::File::open(&std::path::Path::new(archive_name))?;
    let archive = zip::ZipArchive::new(file)?;
    Ok(archive)
}

pub fn do_ls(archive_name: &str) -> Fb2Result<()> {
    let mut archive = do_open_archive(archive_name)?;
    for i in 0..archive.len() {
        let zip_file = archive.by_index(i)?;
        println!(
            "Filename: {}, {} / {}",
            zip_file.name(),
            zip_file.compressed_size(),
            zip_file.size()
        );
    }
    Ok(())
}

pub fn do_cat(archive_name: &str, file_name: &str) -> Fb2Result<()> {
    let mut archive = do_open_archive(archive_name)?;
    let zip_file = archive.by_name(file_name)?;
    let bytes = zip_file.bytes().take(1024);
    for byte in bytes {
        print!("{}", byte.unwrap() as char);
    }
    Ok(())
}
