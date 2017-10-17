
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

fn find_positions(header: &[u8], beg: &str, end: &str) -> Option<(usize, usize)> {
    if let Some(pos) = helper::find(header, beg.as_bytes()) {
        let spos = pos + beg.len();
        if let Some(epos) = helper::find(&header[spos..], end.as_bytes()) {
            return Some((spos, spos + epos));
        }
    }
    None
}

fn extract_xml_prolog(header: &[u8]) -> Vec<u8> {
    if let Some((spos, epos)) = find_positions(header, "<", ">") {
        let prolog: Vec<u8> = header[spos..epos].to_vec();
        if header[0..1] == [0xFF, 0xFE] || header[0..1] == [0xFE, 0xFF] {
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

fn replace_encoding(encoding: &str, xml: &str) -> String {
    String::from(xml.replace(encoding, "utf-8"))
}

pub fn as_utf8(header: Vec<u8>) -> Fb2Result<String> {
    if let Some(encoding) = get_encoding(&header) {
        let encoding = encoding.to_lowercase(); 
        if &encoding != "utf-8" {
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
        let xml = "<?xml version=\"1.0\" encoding=\"utf-8\"?>"
            .as_bytes()
            .to_vec();
        assert_eq!(Some(String::from("utf-8")), super::get_encoding(&xml));
    }

    #[test]
    fn replace_encoding() {
        let xml = super::replace_encoding("koi8-r", "<?xml version=\"1.0\" encoding=\"koi8-r\"?>");
        assert_eq!("<?xml version=\"1.0\" encoding=\"utf-8\"?>", xml);
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use self::test::Bencher;
    use data::bench::XML;

    #[bench]
    fn as_fb2(bencher: &mut Bencher) {
        let xml = String::from(XML);
        bencher.iter(|| { super::as_fb2(xml.clone()).unwrap(); });
    }

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
    fn replace_encoding(bencher: &mut Bencher) {
        bencher.iter(|| { super::replace_encoding("utf-8", XML); });
    }

    #[bench]
    fn as_utf8(bencher: &mut Bencher) {
        let xml = XML.as_bytes().to_vec();
        bencher.iter(|| { super::as_utf8(xml.clone()).unwrap(); });
    }
}
