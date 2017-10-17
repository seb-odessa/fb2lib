use fb;

pub fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let sz = needle.len();
    haystack.windows(sz).position(|window| window == needle)
}

pub fn try_create(xml: String) -> Result<fb::FictionBook, String> {
    // println!("pub fn try_create(xml: String) -> Result<fb::FictionBook, String>");
    fb::deserialize(xml.as_bytes()).map_err(|_| xml)
}

pub fn try_create_with_first_error_fixing(xml: String) -> Result<fb::FictionBook, String> {
    try_fix_escape(xml)
        .or_else(try_fix_leading)
        .or_else(try_fix_double_lang)
        .or_else(try_fix_double_last_name)
        .or_else(try_fix_double_doc_info_nickname)
        .or_else(try_fix_double_doc_info)
        .and_then(try_create)
}

pub fn try_fix_escape(xml: String) -> Result<String, String> {
    // println!("pub fn try_fix_escape(xml: String) -> Result<String, String>");
    if let Some(pos) = xml.find("&") {
        if Some("&amp;") != xml.get(pos..pos + 5) {
            return Ok(xml.replace("&amp;", "\0").replace("&", "&amp;").replace(
                "\0",
                "&amp;",
            ));
        }
    }
    Err(xml)
}

pub fn try_fix_leading(xml: String) -> Result<String, String> {
    // println!("pub fn try_fix_leading(xml: String) -> Result<String, String>");
    if xml.chars().next() != Some('<') {
        Ok(xml.chars().skip_while(|c| *c != '<').collect())
    } else {
        Err(xml)
    }
}

pub fn try_fix_double_lang(xml: String) -> Result<String, String> {
    // println!("pub fn try_fix_double_lang(xml: String) -> Result<String, String>");
    remove_first_tag(xml, "title-info", "lang")
}

pub fn try_fix_double_last_name(xml: String) -> Result<String, String> {
    // println!("pub fn try_fix_double_last_name(xml: String) -> Result<String, String>");
    remove_first_tag(xml, "title-info", "last-name")
}

pub fn try_fix_double_doc_info_nickname(xml: String) -> Result<String, String> {
    // println!("pub fn try_fix_double_doc_info_nickname(xml: String) -> Result<String, String>");
    remove_first_tag(xml, "document-info", "nickname")
}

pub fn try_fix_double_doc_info(xml: String) -> Result<String, String> {
    // println!("pub fn try_fix_double_doc_info(xml: String) -> Result<String, String>");
    remove_first_tag(xml, "description", "document-info")
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

fn is_doubled_tags(content: &str, tag: &str) -> bool {
    let begin = format!("<{}>", tag);
    if let Some(first) = content.find(&begin) {
        if let Some(last) = content.rfind(&begin) {
            return first != last;
        }
    }
    return false;
}

fn remove_first_tag(xml: String, parent: &str, tag: &str) -> Result<String, String> {
    if let Some(content) = get_tag(&xml, parent) {
        if is_doubled_tags(&content, tag) {
            if let Some(tag_content) = get_tag(&content, tag) {
                return Ok(xml.replacen(&tag_content, "", 1));
            }
        }
    }
    Err(xml)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        let buffer = "<?xml version=\"1.0\" encoding=\"utf-8\"?>".as_bytes();
        assert_eq!(Some(20), find(&buffer, "encoding=".as_bytes()));
        assert_eq!(None, find(&buffer, "&&&&".as_bytes()));
    }

    #[test]
    fn test_try_fix_escape() {
        {
            let src = "Smith and Wesson".to_owned();
            let expected = "Smith and Wesson".to_owned();
            assert_eq!(Err(expected), try_fix_escape(src));
        }
        {
            let src = "Smith &amp; Wesson".to_owned();
            let expected = "Smith &amp; Wesson".to_owned();
            assert_eq!(Err(expected), try_fix_escape(src));
        }
        {
            let src = "Smith & Wesson".to_owned();
            let expected = "Smith &amp; Wesson".to_owned();
            assert_eq!(Ok(expected), try_fix_escape(src));
        }
        {
            let src = "Smith & Wesson && Johnson & Johnson".to_owned();
            let expected = "Smith &amp; Wesson &amp;&amp; Johnson &amp; Johnson".to_owned();
            assert_eq!(Ok(expected), try_fix_escape(src));
        }
    }

    #[test]
    fn test_try_fix_leading() {
        {
            let src = "\r\n\0<FictionBook />".to_owned();
            let expected = "<FictionBook />".to_owned();
            assert_eq!(Ok(expected), try_fix_leading(src));
        }
        {
            let src = "<FictionBook />".to_owned();
            let expected = "<FictionBook />".to_owned();
            assert_eq!(Err(expected), try_fix_leading(src));
        }
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
    fn test_is_doubled_tags() {
        let xml = "<title-info><lang>ru</lang><book-title>title</book-title><lang>ru</lang></title-info>";
        assert!(is_doubled_tags(xml, "lang"));
        assert!(!is_doubled_tags(xml, "title"));
        assert!(!is_doubled_tags(xml, "xml"));
    }

    #[test]
    fn try_remove_first_tag() {
        let xml = "<title-info><lang>ru</lang><book-title>title</book-title><lang>ru</lang></title-info>";
        let exp = "<title-info><book-title>title</book-title><lang>ru</lang></title-info>";
        assert_eq!(
            Ok(String::from(exp)),
            remove_first_tag(String::from(xml), "title-info", "lang")
        );
        assert_eq!(
            Err(String::from(exp)),
            remove_first_tag(String::from(exp), "title-info", "lang")
        );
    }


}
