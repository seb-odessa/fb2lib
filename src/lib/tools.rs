use iconv;
use fb2parser::FictionBook;
use result::{Fb2Result, Fb2Error};

use std::error::Error;
use std::hash::Hash;
use std::hash::Hasher;
//use std::collections::hash_map::DefaultHasher;


pub fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let sz = needle.len();
    haystack.windows(sz).position(|window| window == needle)
}

#[allow(dead_code)]
pub fn capitalize<S: Into<String>>(text: S) -> String {
    let mut buffer: Vec<char> = text.into().chars().collect();
    if !buffer.is_empty() {
        buffer[0] = buffer[0].to_uppercase().nth(0).unwrap();
    }
    buffer.into_iter().collect()
}

fn drop_leading_bytes(xml: String) -> Result<String, String> {
    if xml.chars().next() != Some('<') {
        return Ok(xml.chars().skip_while(|c| *c != '<').collect());
    }
    Ok(xml)
}

fn escape_amp(xml: String) -> Result<String, String> {
    if let Some(pos) = xml.find("&") {
        if Some("&amp;") != xml.get(pos..pos + 5) {
            return Ok(xml.replace("&amp;", "\0").replace("&", "&amp;").replace(
                "\0",
                "&amp;",
            ));
        }
    }
    Ok(xml)
}

fn find_tag_content(xml: &String, tag: &str) -> Option<(usize, usize)> {
    let s_tag = format!("<{}>", tag);
    let e_tag = format!("</{}>", tag);
    if let Some(spos) = xml.find(&s_tag) {
        if let Some(pos) = xml.find(&e_tag) {
            return Some((spos, pos + e_tag.len()));
        }
    }
    None
}

fn drop_annotation(xml: String) -> Result<String, String> {
    if let Some((spos, epos)) = find_tag_content(&xml, "annotation") {
        let xml = format!("{}{}", &xml[..spos], &xml[epos..]);
        return Ok(xml);
    }
    Ok(xml)
}

fn drop_coverpage(xml: String) -> Result<String, String> {
    if let Some((spos, epos)) = find_tag_content(&xml, "coverpage") {
        let xml = format!("{}{}", &xml[..spos], &xml[epos..]);
        return Ok(xml);
    }
    Ok(xml)
}


pub fn into_fb2(xml: String) -> Fb2Result<FictionBook> {
    if let Some(fb2) = FictionBook::new(xml.as_bytes()).ok() {
        return Ok(fb2);
    }
    let xml = drop_leading_bytes(xml)
        .and_then(escape_amp)
        .and_then(drop_annotation)
        .and_then(drop_coverpage)
        .unwrap();
    FictionBook::new(xml.as_bytes()).map_err(|e| Fb2Error::Custom(String::from(e.description())))
}

fn find_positions(header: &[u8], beg: &str, end: &str) -> Option<(usize, usize)> {
    if let Some(pos) = find(header, beg.as_bytes()) {
        let spos = pos + beg.len();
        if let Some(epos) = find(&header[spos..], end.as_bytes()) {
            return Some((spos, spos + epos));
        }
    }
    None
}

fn extract_xml_prolog(header: &[u8]) -> Vec<u8> {
    if let Some((spos, epos)) = find_positions(header, "<", ">") {
        let prolog: Vec<u8> = header[spos..epos].to_vec();
        if header[0] == 0xFE || header[0] == 0xFF {
            return prolog.into_iter().filter(|c| *c != 0).collect();
        }
        return prolog;
    }
    header.to_vec()
}

fn get_encoding(header: &[u8]) -> Option<String> {
    let target = extract_xml_prolog(header);
    if let Some((spos, epos)) = find_positions(&target, "encoding=\"", "\"") {
        return String::from_utf8(target[spos..epos].to_vec()).ok();
    }
    None
}

fn utf8(header: &[u8]) -> Fb2Result<String> {
    if let Some(encoding) = get_encoding(&header) {
        let enc = encoding.to_lowercase();
        if &enc != "utf-8" {
            let xml = iconv::to_utf8(&enc, header)?;
            return Ok(xml.replace(&encoding, "utf-8"));
        }
    }
    Ok(String::from_utf8_lossy(header).to_string())
}

pub fn into_utf8(header: Vec<u8>) -> Fb2Result<String> {
    // consuming version used in operation chains
    return utf8(&header);
}

//pub fn get_hash<T: Hash>(value: &T) -> u64 {
//    let mut hasher = DefaultHasher::new();
//    value.hash(&mut hasher);
//    return hasher.finish();
//}

#[cfg(test)]
mod tests {

    #[test]
    fn get_encoding() {
        let xml = "<?xml version=\"1.0\" encoding=\"utf-8\"?>"
            .as_bytes()
            .to_vec();
        assert_eq!(Some(String::from("utf-8")), super::get_encoding(&xml));
    }

//    #[test]
//    fn get_hash() {
//        let value = String::from("String value");
//        assert_eq!(14472655620516614020u64, super::get_hash(&value));
//        assert_eq!(14472655620516614020u64, super::get_hash(&value.clone()));
//
//        let tuple = (value, 42u64);
//        assert_eq!(9839853712099573565u64, super::get_hash(&tuple));
//        assert_eq!(9839853712099573565u64, super::get_hash(&tuple.clone()));
//    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn find_positions(bencher: &mut Bencher) {
        let xml = XML.as_bytes();
        bencher.iter(|| { super::find_positions(&xml, "<", ">"); });
    }

    #[bench]
    fn extract_xml_prolog(bencher: &mut Bencher) {
        let xml = XML.as_bytes();
        bencher.iter(|| { super::extract_xml_prolog(&xml); });
    }

    #[bench]
    fn get_encoding(bencher: &mut Bencher) {
        let xml = XML.as_bytes();
        bencher.iter(|| { super::get_encoding(&xml); });
    }

    #[bench]
    fn into_utf8(bencher: &mut Bencher) {
        let xml = XML.as_bytes();
        bencher.iter(|| { super::utf8(xml).unwrap(); });
    }

    pub const XML: &str = "
    <?xml version=\"1.0\" encoding=\"utf-8\"?>
    <FictionBook xmlns=\"http://www.gribuser.ru/xml/fictionbook/2.0\" xmlns:l=\"http://www.w3.org/1999/xlink\">
    <description>
        <title-info>
            <genre>sf_space</genre>
            <author>
                <first-name>Дж. Майкл</first-name>
                <last-name>Стражинский</last-name>
            </author>
            <book-title>Тень его мыслей</book-title>
            <keywords>Вавилон 5</keywords>
            <date>1999</date>
            <coverpage>
                <image l:href=\"Any2FbImgLoader0\"/>
            </coverpage>
            <lang>ru</lang>
        </title-info>
        <document-info>
            <author>
                <first-name></first-name>
                <last-name></last-name>
            </author>
            <program-used></program-used>
            <date value=\"2008-03-06\">2008-03-06</date>
            <id></id>
            <version></version>
        </document-info>
    </description>
    </FictionBook>";

}
