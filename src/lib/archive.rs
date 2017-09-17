extern crate std;
extern crate zip;

use tools;
use regex::Regex;
use std::io::Read;
use result::Fb2Result;
use result::Fb2Error;
use zip::read::ZipFile;

const BUFFER_LENGTH: usize = 1024;
const FB_CLOSE_TAG: &'static str = "\n</FictionBook>";
const DESC_CLOSE_TAG: &'static str = "</description>";

type Archive = zip::ZipArchive<std::fs::File>;

pub fn find(zip: &mut Archive, file: &str) -> Fb2Result<Vec<usize>> {
    let mut result = Vec::new();
    let re = Regex::new(file)?;
    for i in 0..zip.len() {
        let file: ZipFile = zip.by_index(i)?;
        if re.is_match(&file.name()) {
            result.push(i);
        }
    }
    Ok(result)
}

pub fn open(name: &str) -> Fb2Result<Archive> {
    let file = std::fs::File::open(&std::path::Path::new(name))?;
    let archive = zip::ZipArchive::new(file)?;
    Ok(archive)
}

fn load_buffer(file: &mut ZipFile, result: &mut Vec<u8>) -> Fb2Result<usize> {
    let mut buffer: [u8; BUFFER_LENGTH] = [0; BUFFER_LENGTH];
    match file.read(&mut buffer) {
        Ok(size) => {
            result.extend_from_slice(&buffer);
            Ok(size)
        }
        Err(err) => Err(Fb2Error::Io(err)),
    }
}

pub fn load_header(file: &mut ZipFile) -> Fb2Result<Vec<u8>> {
    let mut header: Vec<u8> = Vec::new();
    while let Some(_) = load_buffer(file, &mut header).ok() {
        if let Some(position) = tools::find(&header, DESC_CLOSE_TAG.as_bytes()) {
            header.resize(position, 0u8);
            header.extend_from_slice(DESC_CLOSE_TAG.as_bytes());
            header.extend_from_slice(FB_CLOSE_TAG.as_bytes());
            return Ok(header);
        }
    }
    Err(Fb2Error::UnableToLoadFb2Header)
}

