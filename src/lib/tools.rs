
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
    if let Some(encoding) = get_encoding(&header) {
        if encoding != String::from("utf-8") {
            let mut result = Vec::new();    
            result.resize(4 * header.len(), 0u8);
            let (_, _, _) = Converter::new(&encoding, "utf-8").convert(&header, &mut result);
            Ok(String::from(String::from_utf8_lossy(&result)))
        } else {
            Ok(String::from(String::from_utf8_lossy(&header)))
        }   
    } else {
        Err(Fb2Error::UnableToMakeUtf8)
    }
}
