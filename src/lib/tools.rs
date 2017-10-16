
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

fn find_positions(header: &Vec<u8>, beg: &str, end: &str) -> Option<(usize, usize)> {
    if let Some(pos) = helper::find(&header, beg.as_bytes()) {
        let spos = pos + beg.len();
        if let Some(epos) = helper::find(&header[spos..], end.as_bytes()) {
            return Some((spos, spos + epos));
        }
    }
    None
}

fn extract_xml_prolog(header: &Vec<u8>) -> Vec<u8> {
    if let Some((spos, epos)) = find_positions(header, "<", ">") {
        let mut target = Vec::new();
        target.extend_from_slice(&header[spos..epos]);
        if header.len() > 2 {
            if header[0..1] == [0xFF, 0xFE] || header[0..1] == [0xFE, 0xFF] {
                return target.into_iter().filter(|c| *c != 0).collect();
            }
        }
        return target;
    }
    header.clone()
}

fn get_encoding(header: &Vec<u8>) -> Option<String> {
    let target = extract_xml_prolog(header);
    if let Some((spos, epos)) = find_positions(&target, "encoding=\"", "\"") {
        let encoding = String::from_utf8_lossy(&target[spos..epos]);
        return Some(encoding.into_owned());
    }
    None
}

fn replace_encoding(encoding: &str, xml: &str) -> String {
    let from = format!("encoding=\"{}\"", encoding);
    String::from(xml.replace(&from, "encoding=\"utf-8\""))
}

pub fn as_utf8(header: Vec<u8>) -> Fb2Result<String> {
    if let Some(encoding) = get_encoding(&header) {
        if encoding.to_lowercase() != String::from("utf-8") {
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

#[cfg(test)]
mod bench {
    extern crate test;
    use self::test::Bencher;

    use data::bench::XML;

    #[bench]
    fn bench_as_fb2(bencher: &mut Bencher) {
        let xml = String::from(XML);
        bencher.iter(|| { super::as_fb2(xml.clone()).unwrap(); });
    }

    #[bench]
    fn bench_find_positions(bencher: &mut Bencher) {
        let mut header = Vec::new();
        header.extend_from_slice(XML.as_bytes());
        bencher.iter(|| { super::find_positions(&header, "encoding=\"", "\""); });
    }

    #[bench]
    fn bench_extract_xml_prolog(bencher: &mut Bencher) {
        let mut header = Vec::new();
        header.extend_from_slice(XML.as_bytes());
        bencher.iter(|| { super::extract_xml_prolog(&header); });
    }

    #[bench]
    fn bench_get_encoding(bencher: &mut Bencher) {
        let mut header = Vec::new();
        header.extend_from_slice(XML.as_bytes());
        bencher.iter(|| { super::get_encoding(&header); });
    }

    #[bench]
    fn bench_replace_encoding(bencher: &mut Bencher) {
        bencher.iter(|| { super::replace_encoding("utf-8", XML); });
    }

    #[bench]
    fn bench_as_utf8(bencher: &mut Bencher) {
        let mut header = Vec::new();
        header.extend_from_slice(XML.as_bytes());
        bencher.iter(|| { super::as_utf8(header.clone()).unwrap(); });
    }
}
