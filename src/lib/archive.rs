extern crate std;
extern crate zip;

use tools;
use helper;
use regex::Regex;
use std::io::Read;
use result::Fb2Result;
use result::Fb2Error;
use zip::read::ZipFile;
use std::error::Error;
use fb::FictionBook;

const CHUNCK_LENGTH: usize = 2048;
const FB_CLOSE_TAG: &'static str = "\n</FictionBook>";
const FB_CLOSE_UTF16: &'static str = "\n\0<\0/\0F\0i\0c\0t\0i\0o\0n\0B\0o\0o\0k\0>\0";

pub type ZipArchive = zip::ZipArchive<std::fs::File>;

pub fn open(name: &str) -> Fb2Result<ZipArchive> {
    let file = std::fs::File::open(&std::path::Path::new(name))?;
    let archive = ZipArchive::new(file)?;
    Ok(archive)
}

fn load_buffer(file: &mut ZipFile, content: &mut Vec<u8>) -> bool {
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

pub fn load_header(file: &mut ZipFile) -> Fb2Result<Vec<u8>> {
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

pub fn apply<F>(mut archive: ZipArchive, file_name: &str, mut visitor: F) -> Fb2Result<()>
where
    F: FnMut(ZipFile) -> Fb2Result<()>,
{
    match Regex::new(&wildcards_to_regex(file_name)) {
        Ok(re) => {
            for i in 0..archive.len() {
                let file: ZipFile = archive.by_index(i)?;
                if re.is_match(file.name()) {
                    visitor(file)?;
                }
            }
            Ok(())
        }
        Err(e) => Err(Fb2Error::Custom(String::from(e.description()))),
    }
}


pub fn load_xml(file: &mut ZipFile) -> Fb2Result<String> {
    load_header(file).and_then(tools::into_utf8)
}

pub fn load_fb2(file: &mut ZipFile) -> Fb2Result<FictionBook> {
    load_xml(file).and_then(tools::into_fb2)
}

#[allow(dead_code)]
fn wildcards_to_regex(arg: &str) -> String {
    let reg = String::from("^") + arg + "$";
    reg.replace(".", "\\.")
        .replace("\\*", "\0")
        .replace("*", "(.*)")
        .replace("\0", "\\*")
        .replace("\\?", "\0")
        .replace("?", "(.{1})")
        .replace("\0", "\\?")
}

#[cfg(test)]
mod tests {
    extern crate regex;

    #[test]
    fn expand_asterix_to_regexp() {
        assert_eq!("^file\\.txt$", &super::wildcards_to_regex("file.txt"));
        assert_eq!("^file(.*)\\.txt$", &super::wildcards_to_regex("file*.txt"));
        assert_eq!(
            "^file\\*(.*)\\.txt$",
            &super::wildcards_to_regex("file\\**.txt")
        );
    }

    #[test]
    fn expand_question_to_regexp() {
        assert_eq!("^file\\.txt$", &super::wildcards_to_regex("file.txt"));
        assert_eq!(
            "^file(.{1})\\.txt$",
            &super::wildcards_to_regex("file?.txt")
        );
        assert_eq!(
            "^file\\?(.{1})\\.txt$",
            &super::wildcards_to_regex("file\\??.txt")
        );
    }

    #[test]
    fn regex_asterix() {
        let re = regex::Regex::new("^file(.*).txt$").unwrap();
        assert!(re.is_match("file.txt"));
        assert!(re.is_match("file_long_name.txt"));
        assert!(re.is_match("file*.txt"));
        assert!(re.is_match("file..txt"));
    }

    #[test]
    fn regex_question() {
        let re = regex::Regex::new("^file(.{1})txt$").unwrap();
        assert!(re.is_match("file.txt"));
        assert!(!re.is_match("filetxt"));
        assert!(re.is_match("file_txt"));
        assert!(re.is_match("file*txt"));
    }

    #[test]
    fn regex_user_input_asterix() {
        let re = regex::Regex::new(&super::wildcards_to_regex("fil*.txt")).unwrap();
        assert!(re.is_match("file.txt"));
        assert!(re.is_match("file1.txt"));
        assert!(re.is_match("file_with_long_name.txt"));
        assert!(re.is_match("filefile.txt"));
        assert!(re.is_match("file.txt.file.txt"));
    }

    #[test]
    fn regex_user_input_question() {
        let re = regex::Regex::new(&super::wildcards_to_regex("fil??txt")).unwrap();
        assert!(re.is_match("file.txt"));
        assert!(re.is_match("fil__txt"));
        assert!(!re.is_match("file_with_long_name.txt"));
        assert!(!re.is_match("filefile.txt"));
        assert!(!re.is_match("file.txt.file.txt"));
    }

    #[test]
    fn regex_user_input_wo_wildcards() {
        let re = regex::Regex::new(&super::wildcards_to_regex("file.txt")).unwrap();
        assert!(re.is_match("file.txt"));
        assert!(!re.is_match(".file.txt"));
        assert!(!re.is_match("file.txt."));
        assert!(!re.is_match("fil__txt"));
        assert!(!re.is_match("file_with_long_name.txt"));
        assert!(!re.is_match("filefile.txt"));
        assert!(!re.is_match("file.txt.file.txt"));
    }
}
