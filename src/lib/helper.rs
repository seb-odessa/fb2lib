use fb;

fn escape(xml: &String) -> String {
    if xml.find("&amp;").is_none() {
        if xml.find("&").is_some() {
            return xml.replace("&amp;", "&").replace("&", "&amp;");
        }
    }
    return xml.clone();
}

fn get_tag(content: &str, tag: &str) -> Option<String> {
    let beg = String::from("<") + tag + ">";
    let end = String::from("</") + tag + ">";
    if let Some(spos) = content.find(&beg) {
        if let Some(epos) = content.find(&end) {
            let needle: &str = &content[spos..epos + end.len()];
            return Some(String::from(needle));
        }
    }
    None
}

fn deduplicate_tags(xml: &String, parent: &str, tag: &str) -> String {
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
    return xml.clone();
}

fn remove_first_tag(xml: &String, parent: &str, tag: &str) -> String {
    if let Some(content) = get_tag(&xml, parent) {
        if let Some(value) = get_tag(&content, tag) {
            return xml.replacen(&value, "", 1);
        }
    }
    return xml.clone();
}

pub fn try_fast(xml: String) -> Result<fb::FictionBook, String> {
    // Skip heading bytes until '<' will found
    let clean:String = xml.chars().skip_while(|c| *c != '<').collect();
    match fb::deserialize(clean.as_bytes()) {
        Ok(result) => Ok(result),
        Err(_) => Err(clean),
    }
}

pub fn try_escaped(xml: String) -> Result<fb::FictionBook, String> {
    let fixed_xml = escape(&xml);
    match fb::deserialize(fixed_xml.as_bytes()) {
        Ok(result) => Ok(result),
        Err(_) => Err(fixed_xml),
    }
}

pub fn try_fix_lang(xml: String) -> Result<fb::FictionBook, String> {
    let fixed_xml = deduplicate_tags(&xml, "title-info", "lang");
    match fb::deserialize(fixed_xml.as_bytes()) {
        Ok(result) => Ok(result),
        Err(_) => Err(fixed_xml),
    }
}

pub fn try_fix_title_info_double_last_name(xml: String) -> Result<fb::FictionBook, String> {
    let fixed_xml = deduplicate_tags(&xml, "title-info", "last-name");
    match fb::deserialize(fixed_xml.as_bytes()) {
        Ok(result) => Ok(result),
        Err(_) => Err(fixed_xml),
    }
}

pub fn try_fix_doc_info_double_nickname(xml: String) -> Result<fb::FictionBook, String> {
    let fixed_xml = remove_first_tag(&xml, "document-info", "nickname");
    match fb::deserialize(fixed_xml.as_bytes()) {
        Ok(result) => Ok(result),
        Err(_) => Err(fixed_xml),
    }
}

pub fn try_fix_double_doc_info(xml: String) -> Result<fb::FictionBook, String> {
    let fixed_xml = remove_first_tag(&xml, "description", "document-info");
    match fb::deserialize(fixed_xml.as_bytes()) {
        Ok(result) => Ok(result),
        Err(_) => Err(fixed_xml),
    }
}

pub fn done(xml: String) -> Result<fb::FictionBook, fb::Error> {
    match fb::deserialize(xml.as_bytes()) {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

