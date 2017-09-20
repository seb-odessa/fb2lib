extern crate std;
extern crate zip;

use tools;
use std::io::Read;
use result::Fb2Result;
use result::Fb2Error;
use zip::read::ZipFile;
use fb2parser::fb::FictionBook;

const BUFFER_LENGTH: usize = 1024;
const DESC_CLOSE_TAG: &'static str = "</description>";
const FAKE_BODY_TAG: &'static str = "<body> ... WAS SKIPPED ... </body>";
const FB_CLOSE_TAG: &'static str = "\n</FictionBook>";

pub type ZipArchive = zip::ZipArchive<std::fs::File>;

pub fn open(name: &str) -> Fb2Result<ZipArchive> {
    let file = std::fs::File::open(&std::path::Path::new(name))?;
    let archive = ZipArchive::new(file)?;
    Ok(archive)
}

pub fn find_by_name<'a>(zip: &'a mut ZipArchive, file_name: &str) -> Fb2Result<ZipFile<'a>> {
    match zip.by_name(file_name) {
        Ok(file) => Ok(file),
        Err(_) => Err(Fb2Error::FileNotFound)
    }
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
            header.extend_from_slice(FAKE_BODY_TAG.as_bytes());
            header.extend_from_slice(FB_CLOSE_TAG.as_bytes());
            return Ok(header);
        }
    }
    Err(Fb2Error::UnableToLoadFb2Header)
}

pub fn apply_all<F>(mut archive: ZipArchive, mut visitor: F) -> Fb2Result<()>
where
    F: FnMut(ZipFile) -> Fb2Result<()>
{
    for i in 0..archive.len() {
        let file: ZipFile = archive.by_index(i)?;
        visitor(file)?;
    }
    Ok(())
}

pub fn apply_one<F>(mut archive: ZipArchive, file_name: &str, mut visitor: F) -> Fb2Result<()>
where
    F: FnMut(ZipFile) -> Fb2Result<()>
{
    match archive.by_name(file_name) {
        Ok(file) => visitor(file),
        Err(_) => Err(Fb2Error::FileNotFound)
    }
}

pub fn load_xml(file: &mut ZipFile) -> Fb2Result<String> {
    load_header(file).and_then(tools::as_utf8)
}

pub fn load_fb2(file: &mut ZipFile) -> Fb2Result<FictionBook> {
    load_xml(file).and_then(tools::create_fb2)
}