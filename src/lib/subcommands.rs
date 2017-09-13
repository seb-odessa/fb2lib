extern crate std;
extern crate zip;

use result::Fb2Result;
use result::Fb2Error;
use std::io::Read;
use zip::read::ZipFile;

const BUFFER_LENGTH: usize = 1024;
const FB_CLOSE_TAG: &'static str = "\n</FictionBook>";
const DESCRIPTION_TAG: &'static str = "</description>";

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

fn load(file: &mut ZipFile, result: &mut Vec<u8>) -> Fb2Result<usize> {
    let mut buffer: [u8; BUFFER_LENGTH] = [0; BUFFER_LENGTH];
    match file.read(&mut buffer) {
        Ok(size) => {
            result.extend_from_slice(&buffer);
            Ok(size)
        },
        Err(err) => {
            Err(Fb2Error::Io(err))
        },
    }
}

fn find_subsequence_start(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

fn find_subsequence_end(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    match find_subsequence_start(haystack, needle) {
        Some(size) => Some(size + needle.len()),
        None => None,
    }
}

fn load_header(file: &mut ZipFile, header: &mut Vec<u8>) -> Fb2Result<()> {    
    while let Some(size) = load(file, header).ok() {
        println!("Loaded {} bytes", size);
        if let Some(size) = find_subsequence_end(&header, DESCRIPTION_TAG.as_bytes()) {            
            header.resize(size, 0);            
            header.extend_from_slice(FB_CLOSE_TAG.as_bytes());
            return Ok(());
        }
    }    
    Err(Fb2Error::UnableToLoadFb2Header)
}

fn as_utf8(header: &Vec<u8>) -> Fb2Result<String> {
    match std::str::from_utf8(&header) {
        Ok(utf8) => {
            Ok(String::from(utf8))
        },
        Err(err) => {
            println!("Non UTF8 content. Was able to read only {} bytes.", err.valid_up_to());
            Err(Fb2Error::UnableToMakeUtf8)
        }
    }
}

pub fn do_cat(archive_name: &str, file_name: &str) -> Fb2Result<()> {
    let mut archive = do_open_archive(archive_name)?;
    let mut file = archive.by_name(file_name)?;
    let mut header: Vec<u8> = Vec::new();
    load_header(&mut file, &mut header)?;
    let description = as_utf8(&header)?;
    println!("Header length: {}", header.len());
    println!("{}", description);
    
    Ok(())
}
