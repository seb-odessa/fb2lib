use result::Fb2Result;
use result::Fb2Error;
use iconv::Converter;

pub fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(
        |window| window == needle,
    )
}

pub fn get_encoding(header: &Vec<u8>) -> Option<String> {
    // Looking for <?xml version="1.0" encoding="Windows-1251"?> subsequence
    const BEGIN: &str = "encoding=\"";
    const END: &str = "\"?>";
    if let Some(pos) = find(&header, BEGIN.as_bytes()) {
        if let Some(end) = find(&header, END.as_bytes()) {
            let start = pos + BEGIN.len();
            let encoding = String::from_utf8_lossy(&header[start..end]).to_lowercase();
            return Some(encoding);
        }
    }
    None
}

pub fn as_utf8(header: &Vec<u8>) -> Fb2Result<String> {
    let mut result = header.clone();
    if let Some(encoding) = get_encoding(&header) {
        if encoding != String::from("utf-8") {
            result.resize(3 * header.len(), 0);
            let (_, output_length, ret) = Converter::new(&encoding, "utf-8").convert(&header, &mut result);
            if 0 != ret && output_length == result.len() {
                return Err(Fb2Error::UnableToMakeUtf8);
            }
        }
    }
    match String::from_utf8(result) {
        Ok(utf8) => Ok(utf8),
        Err(_) => Err(Fb2Error::UnableToMakeUtf8)
    }
}
