use result::Fb2Result;
use result::Fb2Error;
use iconv::Converter;
use fb::XmlError;
use helper;
use fb::FictionBook;

fn create(xml: String) -> Result<FictionBook, XmlError> {
    return helper::try_fast(xml).
        or_else(helper::try_escaped).
        or_else(helper::try_skip_leading).
        or_else(helper::try_fix_double_lang).
        or_else(helper::try_fix_double_last_name).
        or_else(helper::try_fix_double_doc_info_nickname).
        or_else(helper::try_fix_double_doc_info).
        or_else(helper::done);
}

pub fn create_fb2(xml: String) -> Fb2Result<FictionBook> {
    create(xml).map_err(|e| { Fb2Error::Custom(format!("Unable to deserialize XML: {}", e)) })
}

fn get_encoding(header: &Vec<u8>) -> Option<String> {
    const BEGIN: &str = "encoding=\"";
    const END: &str = "\"?>";
    if let Some(pos) = helper::find(&header, BEGIN.as_bytes()) {
        if let Some(end) = helper::find(&header, END.as_bytes()) {
            let start = pos + BEGIN.len();
            let encoding = String::from_utf8_lossy(&header[start..end]);
            return Some(encoding.into_owned());
        }
    }
    None
}

fn replace_encoding(encoding: &str, xml: &str) -> String {
    let from = format!("encoding=\"{}\"",encoding);
    String::from(xml.replace(&from, "encoding=\"utf-8\""))
}

pub fn as_utf8(header: Vec<u8>) -> Fb2Result<String> {
    if let Some(encoding) = get_encoding(&header) {
        if encoding != String::from("utf-8") {
            let converter = Converter::new(&encoding.to_lowercase(), "utf-8")?;
            let buffer = converter.utf8(&header)?;
            let header = String::from_utf8_lossy(&buffer);
            return Ok(replace_encoding(&encoding, &header));
        }
    }
    Ok(String::from_utf8_lossy(&header).to_string())
}

#[cfg(test)]
mod tests {

    #[test]
    fn get_encoding() {
        let mut buffer = Vec::new();
        buffer.extend_from_slice("<?xml version=\"1.0\" encoding=\"Utf-8\"?>".as_bytes());
        assert_eq!(Some(String::from("Utf-8")), super::get_encoding(&buffer));
    }

    #[test]
    fn replace_encoding() {
        let result = super::replace_encoding("Utf-8", "<?xml version=\"1.0\" encoding=\"Utf-8\"?>");
        assert_eq!(Some(20), result.find("encoding=\"utf-8\"") );
        assert_eq!(None, result.find("encoding=\"Utf-8\"") );
    }


/*
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