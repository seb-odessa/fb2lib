extern crate std;
extern crate zip;

use tools;
use helper;
use std::io::Read;
use result::{Fb2Result, Fb2Error};
use fb::FictionBook;
use std::sync::Mutex;


const CHUNCK_LENGTH: usize = 2048;
const FB_CLOSE_TAG: &'static str = "\n</FictionBook>";
const FB_CLOSE_UTF16: &'static str = "\n\0<\0/\0F\0i\0c\0t\0i\0o\0n\0B\0o\0o\0k\0>\0";

pub type ZipArchive = zip::ZipArchive<std::fs::File>;
pub type ZipFile<'a> = Mutex<zip::read::ZipFile<'a>>;

pub fn open(name: &str) -> Fb2Result<ZipArchive> {
    let file = std::fs::File::open(&std::path::Path::new(name))?;
    let archive = ZipArchive::new(file)?;
    Ok(archive)
}

fn load_buffer(file: &mut zip::read::ZipFile, content: &mut Vec<u8>) -> bool {
    //println!("content.len(): {}", content.len());
    content.reserve(CHUNCK_LENGTH);
    let mut buffer = [0u8; CHUNCK_LENGTH];
    if let Some(size) = file.read(&mut buffer).ok() {
        if size > 0 {
            content.extend_from_slice(&buffer);
            return true;
        }
    }
    return false;
}

pub fn load_header(file: &mut zip::read::ZipFile) -> Fb2Result<Vec<u8>> {
    let mut header: Vec<u8> = Vec::new();
    while load_buffer(file, &mut header) {
        const DESC_CLOSE_TAG: &'static str = "</description>";
        if let Some(position) = helper::find(&header, DESC_CLOSE_TAG.as_bytes()) {
            header.resize(position, 0u8);
            header.extend_from_slice(DESC_CLOSE_TAG.as_bytes());
            header.extend_from_slice(FB_CLOSE_TAG.as_bytes());
            return Ok(header);
        }
        // Support of the UTF-16 files
        const DESC_CLOSE_UTF16: &'static str = "<\0/\0d\0e\0s\0c\0r\0i\0p\0t\0i\0o\0n\0>\0";
        if let Some(position) = helper::find(&header, DESC_CLOSE_UTF16.as_bytes()) {
            header.resize(position, 0u8);
            header.extend_from_slice(DESC_CLOSE_UTF16.as_bytes());
            header.extend_from_slice(FB_CLOSE_UTF16.as_bytes());
            return Ok(header);
        }
        // Support of the broken tags
        const DESC_CLOSE_WRONG: &'static str = "&lt;/description&gt";
        if let Some(position) = helper::find(&header, DESC_CLOSE_WRONG.as_bytes()) {
            header.resize(position, 0u8);
            header.extend_from_slice(DESC_CLOSE_TAG.as_bytes());
            header.extend_from_slice(FB_CLOSE_TAG.as_bytes());
            return Ok(header);
        }
    }
    Ok(header)
}

pub fn load_xml(file: &mut ZipFile) -> Fb2Result<String> {
    match file.lock() {
        Ok(ref mut file) => load_header(file).and_then(tools::into_utf8),
        Err(_) => Err(Fb2Error::Custom(String::from("Can't acquire mutex"))),
    }
}

pub fn load_fb2(file: &mut ZipFile) -> Fb2Result<FictionBook> {
    load_xml(file).and_then(tools::into_fb2)
}
