use fb;

pub fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(
        |window| window == needle,
    )
}

fn smart_escape(xml: String) -> Result<String, String> {
    if let Some(pos) = xml.find("&") {
        if Some("&amp;") != xml.get(pos..pos + 5) {
            return Ok(escape(xml));
        }
    }
    Err(xml)
}

fn escape(xml: String) -> String {
    xml.replace("&amp;", "\0").replace("&", "&amp;").replace(
        "\0",
        "&amp;",
    )
}

fn skip_leading(xml: String) -> String {
    // skip leading chars before first '<'
    if xml.chars().next() != Some('<') {
        xml.chars().skip_while(|c| *c != '<').collect()
    } else {
        xml
    }
}

fn get_tag(content: &str, tag: &str) -> Option<String> {
    let beg = format!("<{}>", tag);
    let end = format!("</{}>", tag);
    if let Some(spos) = content.find(&beg) {
        if let Some(epos) = content.find(&end) {
            let needle: &str = &content[spos..epos + end.len()];
            return Some(String::from(needle));
        }
    }
    None
}

fn deduplicate_tags(xml: String, parent: &str, tag: &str) -> String {
    if let Some(content) = get_tag(&xml, parent) {
        if let Some(value) = get_tag(&xml, tag) {
            if let Some(first) = content.find(&value) {
                if let Some(last) = content.rfind(&value) {
                    if first != last {
                        return xml.replacen(&value, "", 1);
                    }
                }
            }
        }
    }
    return xml;
}

fn remove_first_tag(xml: String, parent: &str, tag: &str) -> String {
    if let Some(content) = get_tag(&xml, parent) {
        if let Some(value) = get_tag(&content, tag) {
            return xml.replacen(&value, "", 1);
        }
    }
    return xml;
}

pub fn try_fast(xml: String) -> Result<fb::FictionBook, String> {
    fb::deserialize(xml.as_bytes()).map_err(|_| xml)
}

pub fn try_escaped(xml: String) -> Result<fb::FictionBook, String> {
    match smart_escape(xml) {
        Ok(fixed) => fb::deserialize(fixed.as_bytes()).map_err(|_| fixed),
        Err(unchanged) => Err(unchanged)
    }
}

pub fn try_skip_leading(xml: String) -> Result<fb::FictionBook, String> {
    let fixed = skip_leading(xml);
    fb::deserialize(fixed.as_bytes()).map_err(|_| fixed)
}

pub fn try_fix_double_lang(xml: String) -> Result<fb::FictionBook, String> {
    let fixed = deduplicate_tags(xml, "title-info", "lang");
    fb::deserialize(fixed.as_bytes()).map_err(|_| fixed)
}

pub fn try_fix_double_last_name(xml: String) -> Result<fb::FictionBook, String> {
    let fixed = deduplicate_tags(xml, "title-info", "last-name");
    fb::deserialize(fixed.as_bytes()).map_err(|_| fixed)
}

pub fn try_fix_double_doc_info_nickname(xml: String) -> Result<fb::FictionBook, String> {
    let fixed = remove_first_tag(xml, "document-info", "nickname");
    fb::deserialize(fixed.as_bytes()).map_err(|_| fixed)
}

pub fn try_fix_double_doc_info(xml: String) -> Result<fb::FictionBook, String> {
    let fixed = remove_first_tag(xml, "description", "document-info");
    fb::deserialize(fixed.as_bytes()).map_err(|_| fixed)
}

pub fn done(xml: String) -> Result<fb::FictionBook, fb::Error> {
    fb::deserialize(xml.as_bytes()).map_err(|err| { return err; })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_find() {
        let buffer = "<?xml version=\"1.0\" encoding=\"Utf-8\"?>".as_bytes();
        assert_eq!(Some(20), find(&buffer, "encoding=".as_bytes()));
        assert_eq!(None, find(&buffer, "&&&&".as_bytes()));
    }

    #[test]
    fn test_escape() {
        assert_eq!(
            "Smith and Wesson",
            &escape(String::from("Smith and Wesson"))
        );
        assert_eq!(
            "Smith &amp; Wesson",
            &escape(String::from("Smith & Wesson"))
        );
        assert_eq!(
            "Smith &amp; Wesson",
            &escape(String::from("Smith &amp; Wesson"))
        );
        assert_eq!(
            "Smith &amp; Wesson &amp;&amp; Johnson &amp; Johnson",
            &escape(String::from("Smith & Wesson && Johnson & Johnson"))
        );
    }

    #[test]
    fn test_smart_escape() {
        assert_eq!(
            Ok(String::from("Smith &amp; Wesson")),
            smart_escape(String::from("Smith & Wesson"))
        );
        assert_eq!(
            Err(String::from("Smith &amp; Wesson")),
            smart_escape(String::from("Smith &amp; Wesson"))
        );

        assert_eq!(
            Err(String::from("Smith and Wesson")),
            smart_escape(String::from("Smith and Wesson"))
        );
    }

    const XML: &str = "<?xml version=\"1.0\" encoding=\"Utf-8\"?>
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

    const EXPECTED_XML: &str = "<?xml version=\"1.0\" encoding=\"Utf-8\"?>
        <FictionBook xmlns=\"http://www.gribuser.ru/xml/fictionbook/2.0\" xmlns:l=\"http://www.w3.org/1999/xlink\">
            <description>
                <title-info>
                    <genre>жанр</genre>
                    <author>
                        <first-name>Имя</first-name>
                        <middle-name>Отчество</middle-name>
                        <last-name>Фамилия</last-name>
                    </author>
                    <book-title>Название с невалидным XML символом &amp; (амперсанд)</book-title>
                    <lang>ru</lang>
                </title-info>
                <document-info>
                </document-info>
            </description>
        </FictionBook>";

    #[bench]
    fn bench_escape(bencher: &mut Bencher) {
        let xml = String::from(XML);
        let expected = String::from(EXPECTED_XML);
        bencher.iter(|| {
            assert_eq!(expected.clone(), escape(expected.clone()));
            assert_eq!(expected.clone(), escape(xml.clone()));
            assert_eq!(expected.clone(), escape(expected.clone()));
        });
    }

    #[bench]
    fn bench_smart_escape(bencher: &mut Bencher) {
        let xml = String::from(XML);
        let expected = String::from(EXPECTED_XML);
        bencher.iter(|| {
            assert_eq!(Err(expected.clone()), smart_escape(expected.clone()));
            assert_eq!(Ok(expected.clone()), smart_escape(xml.clone()));
            assert_eq!(Err(expected.clone()), smart_escape(expected.clone()));
        });

    }

    #[test]
    fn test_skip_leading() {
        assert_eq!(
            "<FictionBook />",
            &skip_leading(String::from("...<FictionBook />"))
        );
        assert_eq!(
            "<FictionBook />",
            &skip_leading(String::from("\0<FictionBook />"))
        );
        assert_eq!(
            "<FictionBook />",
            &skip_leading(String::from("\r\n<FictionBook />"))
        );
    }

    #[test]
    fn test_get_tag() {
        let xml = "<author><first>Имя</first><middle>Отчество</middle><last>Фамилия</last></author>";
        assert_eq!(
            Some(String::from("<first>Имя</first>")),
            get_tag(xml, "first")
        );
        assert_eq!(
            Some(String::from("<middle>Отчество</middle>")),
            get_tag(xml, "middle")
        );
        assert_eq!(
            Some(String::from("<last>Фамилия</last>")),
            get_tag(xml, "last")
        );
        assert_eq!(None, get_tag(xml, "not_exist"));
    }

    #[test]
    fn test_parse_double_lang_tag() {
        let xml = "<FictionBook>
            <description>
                <title-info>
                    <genre>жанр</genre>
                    <author>
                        <first-name>Имя</first-name>
                        <middle-name>Отчество</middle-name>
                        <last-name>Фамилия</last-name>
                    </author>
                    <lang>ru</lang>
                    <book-title>Название</book-title>
                    <lang>ru</lang>
                </title-info>
                <document-info>
                </document-info>
            </description>
        </FictionBook>"
            .to_owned();
        assert!(try_fix_double_lang(xml).is_ok());
    }

}
