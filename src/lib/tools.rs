use result::Fb2Result;
use result::Fb2Error;
use iconv::Converter;
use fb;
use helper;
use fb::{FictionBook, Description, Author};
use std::error::Error;

fn create(xml: String) -> Result<FictionBook, fb::SerdeError> {
    return helper::try_fast(xml).
        or_else(helper::try_escaped).
        or_else(helper::try_fix_lang).
        or_else(helper::try_fix_title_info_double_last_name).
        or_else(helper::try_fix_doc_info_double_nickname).
        or_else(helper::try_fix_double_doc_info).
        or_else(helper::done);
}

pub fn create_fb2(xml: String) -> Fb2Result<FictionBook> {
    match create(xml) {
        Ok(fb) => Ok(fb),
        Err(e) => Err(Fb2Error::Custom(String::from("Unable to deserializeXML") + e.description()))
    }
}


pub fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(
        |window| window == needle,
    )
}

fn get_encoding(header: &Vec<u8>) -> Option<String> {
    const BEGIN: &str = "encoding=\"";
    const END: &str = "\"?>";
    if let Some(pos) = find(&header, BEGIN.as_bytes()) {
        if let Some(end) = find(&header, END.as_bytes()) {
            let start = pos + BEGIN.len();
            let encoding = String::from_utf8_lossy(&header[start..end]);
            return Some(encoding.into_owned());
        }
    }
    None
}

fn convert_to_utf8(src: Vec<u8>) -> Fb2Result<String> {
    match String::from_utf8(src) {
        Ok(utf8) => Ok(utf8),
        Err(e) => Err(Fb2Error::Custom(String::from("Unable to apply 'std::string::String::from_utf8' to ") + e.description()))
    }
}

fn replace_encoding(encoding: &str, xml: &str) -> String {
    let from = String::from("encoding=\"") + encoding + "\"";
    String::from(xml.replace(&from, "encoding=\"utf-8\""))
}


fn try_create_converter(src: &str, dst: &str) -> Fb2Result<Converter> {
    match Converter::new(src, dst) {
        Ok(converter) => Ok(converter),
        Err(msg) => Err(Fb2Error::Custom(msg))
    }
}

fn try_convert(converter: &Converter, src: &Vec<u8>) -> Fb2Result<Vec<u8>> {
    let expected_len = 4 * src.len();
    let mut dst = Vec::with_capacity(expected_len);
    dst.resize(expected_len, 0u8);
    let (_, length, ret) = converter.convert(&src, &mut dst);
    if 0 != ret {
        return Err(Fb2Error::Custom(String::from("Unable to convert input buffer")))
    }
    if length == dst.len() {
        return Err(Fb2Error::Custom(String::from("Unable to convert input buffer. Not enough destination buffer length")))
    }
    dst.resize(length, 0u8);
    return Ok(dst);
}

pub fn as_utf8(header: Vec<u8>) -> Fb2Result<String> {
    if let Some(encoding) = get_encoding(&header) {
        if encoding != String::from("utf-8") {
            let converter = try_create_converter(&encoding.to_lowercase(), "utf-8")?;
            let buffer = try_convert(&converter, &header)?;
            let header = convert_to_utf8(buffer)?;
            return Ok(replace_encoding(&encoding, &header));
        }
    }
    convert_to_utf8(header)
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

pub fn fmt_book(fb: &FictionBook) -> String {
    fmt_info(&fb.description)
}

#[cfg(test)]
mod tests {

    const FB2_HEADER: &str =
        "<?xml version=\"1.0\" encoding=\"Utf-8\"?>
            <FictionBook xmlns=\"http://www.gribuser.ru/xml/fictionbook/2.0\" xmlns:l=\"http://www.w3.org/1999/xlink\">
            <description>
                <title-info>
                    <genre>жанр</genre>
                    <author>
                        <first-name>Имя</first-name>
                        <middle-name>Отчество</middle-name>
                        <last-name>Фамилия</last-name>
                    </author>
                    <book-title>Название с невалидным XML символом & (амперсанд)</book-title>
                    <lang>ru</lang>
                </title-info>
                <document-info>
                </document-info>
            </description>
        </FictionBook>";

    #[test]
    fn find_in_buffer() {
        let buffer = FB2_HEADER.as_bytes();
        assert_eq!(Some(20), super::find(&buffer, "encoding=".as_bytes()));
        assert_eq!(None, super::find(&buffer, "&&&&".as_bytes()));
    }

    #[test]
    fn get_encoding() {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(FB2_HEADER.as_bytes());
        assert_eq!(Some(String::from("Utf-8")), super::get_encoding(&buffer));
    }

    #[test]
    fn replace_encoding() {
        let result = super::replace_encoding("Utf-8", FB2_HEADER);
        assert_eq!(Some(20), result.find("encoding=\"utf-8\"") );
        assert_eq!(None, result.find("encoding=\"Utf-8\"") );
    }


/*
    #[test]
    fn parse_double_last_name_tag() {
        let xml = load_xml("test_data/double_last_name.xml");
        assert!(xml.is_ok());
        let obj = self::create(xml.unwrap());
        assert!(obj.is_ok());
    }

    #[test]
    fn parse_description_xml() {
        let xml = load_xml("test_data/description.xml");
        assert!(xml.is_ok());
        let obj = self::create(xml.unwrap());
        assert!(obj.is_ok());
        let fb: FictionBook = obj.unwrap();
        assert_eq!(
            fb,
            FictionBook {
                description: Description {
                    title_info: TitleInfo {
                        genre: vec![
                            "sf".to_owned(),
                            "sf_history".to_owned()
                            ],
                        author: vec![
                            Author {
                                first_name: "Константин".to_owned(),
                                middle_name: "Георгиевич".to_owned(),
                                last_name: "Калбанов".to_owned(),
                                nick_name: "".to_owned(),
                                home_page: "http://samlib.ru/k/kalbazow_k_g/".to_owned(),
                                email: "mahoni928@yandex.ru".to_owned(),
                            },
                        ],
                        book_title: "Робинзоны".to_owned(),
                        date: "".to_owned(),
                        lang: "ru".to_owned(),
                        src_lang: "".to_owned(),
                        translator: vec![],
                        sequence: vec![
                            Sequence {
                                name: "Робинзоны".to_owned(),
                                number: "1".to_owned(),
                                lang: "".to_owned(),
                            },
                        ],
                    },
                },
            }
        );
    }
*/
}