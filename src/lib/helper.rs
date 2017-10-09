use fb;

fn escape(xml: String) -> String {
    if xml.find("&amp;").is_none() {
        if xml.find("&").is_some() {
            return xml.replace("&amp;", "&").replace("&", "&amp;");
        }
    }
    return xml;
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
    let fixed = if xml.chars().next() != Some('<') {
        // skip leading chars until '<'
        xml.chars().skip_while(|c| *c != '<').collect()
    } else {
        xml
    };

    fb::deserialize(fixed.as_bytes()).map_err(|_| { fixed })
}

pub fn try_escaped(xml: String) -> Result<fb::FictionBook, String> {    
    let fixed = escape(xml);
    fb::deserialize(fixed.as_bytes()).map_err(|_| { fixed })
}

pub fn try_fix_lang(xml: String) -> Result<fb::FictionBook, String> {
    let fixed = deduplicate_tags(xml, "title-info", "lang");
    fb::deserialize(fixed.as_bytes()).map_err(|_| { fixed })
}

pub fn try_fix_title_info_double_last_name(xml: String) -> Result<fb::FictionBook, String> {
    let fixed = deduplicate_tags(xml, "title-info", "last-name");
    fb::deserialize(fixed.as_bytes()).map_err(|_| { fixed })
}

pub fn try_fix_doc_info_double_nickname(xml: String) -> Result<fb::FictionBook, String> {
    let fixed = remove_first_tag(xml, "document-info", "nickname");
    fb::deserialize(fixed.as_bytes()).map_err(|_| { fixed })
}

pub fn try_fix_double_doc_info(xml: String) -> Result<fb::FictionBook, String> {
    let fixed = remove_first_tag(xml, "description", "document-info");
    fb::deserialize(fixed.as_bytes()).map_err(|_| { fixed })
}

pub fn done(xml: String) -> Result<fb::FictionBook, fb::Error> {
    fb::deserialize(xml.as_bytes()).map_err(|err| { return err; })
}
