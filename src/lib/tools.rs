
use iconv;
use helper;
use fb::FictionBook;
use result::Fb2Result;
use result::Fb2Error;

pub fn as_fb2(xml: String) -> Fb2Result<FictionBook> {
    helper::try_create(xml)
        .or_else(helper::try_create_with_first_error_fixing) // Fix first error
        .or_else(helper::try_create_with_first_error_fixing) // Fix second error
        // .or_else(helper::try_create_with_first_error_fixing) // Fix third error
        // .or_else(helper::try_create_with_first_error_fixing) // Fix fourth error
        .map_err(|e| {
            Fb2Error::Custom(format!("Unable to deserialize XML: {}", e))
        })
}

fn get_encoding(header: &Vec<u8>) -> Option<String> {
    const BEGIN: &str = "encoding=\"";
    const END: &str = "\"";
    let target: Vec<u8> = header.clone().into_iter().filter(|c| *c != 0).collect();
    if let Some(pos) = helper::find(&target, BEGIN.as_bytes()) {
        let spos = pos + BEGIN.len();
        if let Some(end) = helper::find(&target[spos..], END.as_bytes()) {
            let epos = spos + end;
            let encoding = String::from_utf8_lossy(&target[spos..epos]);
            return Some(encoding.into_owned());
        }
    }
    None
}

fn replace_encoding(encoding: &str, xml: &str) -> String {
    let from = format!("encoding=\"{}\"", encoding);
    String::from(xml.replace(&from, "encoding=\"utf-8\""))
}

pub fn as_utf8(header: Vec<u8>) -> Fb2Result<String> {
    if let Some(encoding) = get_encoding(&header) {
        if encoding != String::from("utf-8") {
            let utf8 = iconv::to_utf8(&encoding, &header)?;
            return Ok(replace_encoding(&encoding, &utf8));
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
        let result =
            super::replace_encoding("koi8-r", "<?xml version=\"1.0\" encoding=\"koi8-r\"?>");
        assert_eq!(Some(20), result.find("encoding=\"utf-8\""));
        assert_eq!(None, result.find("encoding=\"koi8-r\""));
    }
}