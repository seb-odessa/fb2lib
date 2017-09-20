extern crate fb2parser;

use result::Fb2Result;
use result::Fb2Error;
use iconv::Converter;
use fb2parser::fb::{FictionBook, Description, Author};

pub fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(
        |window| window == needle,
    )
}

pub fn get_encoding(header: &Vec<u8>) -> Option<String> {
    // Looking for <?xml version="1.0" encoding="Windows-1251"?> subsequence
    const BEGIN: &str = "encoding=\"";
    const END: &str = "\"?>";
    if let Some(pos) = find(&header, BEGIN.as_bytes()) {
        if let Some(end) = find(&header, END.as_bytes()) {
            let start = pos + BEGIN.len();
            let encoding = String::from_utf8_lossy(&header[start..end]).to_lowercase();
            return Some(encoding);
        }
    }
    None
}

pub fn as_utf8(header: Vec<u8>) -> Fb2Result<String> {
    let mut result = header.clone();
    if let Some(encoding) = get_encoding(&header) {
        if encoding != String::from("utf-8") {
            result.resize(3 * header.len(), 0u8);
            let (_, length, ret) = Converter::new(&encoding, "utf-8").convert(&header, &mut result);
            if 0 != ret && length == result.len() {
                return Err(Fb2Error::UnableToMakeUtf8);
            }
            result.resize(length, 0u8);
        }
    }
    match String::from_utf8(result) {
        Ok(utf8) => Ok(utf8),
        Err(_) => Err(Fb2Error::UnableToMakeUtf8)
    }
}

pub fn create_fb2(xml: String) -> Fb2Result<FictionBook> {
    match fb2parser::create(xml) {
        Ok(fb) => Ok(fb),
        Err(_) => Err(Fb2Error::UnableDeserializeXML)
    }
}

fn fmt_author(authors: &Vec<Author>) -> String{
    let mut result = String::new();
    for author in authors {
        if !result.is_empty() {
            result += ", ";
        }
        result += &format!("{} {} {}", &author.first_name, &author.middle_name, &author.last_name);
    }
    return result;
}

pub fn fmt_info(description: &Description) -> String {
    format!("'{}' - {}",
        &description.title_info.book_title, 
        fmt_author(&description.title_info.author)
        )
}