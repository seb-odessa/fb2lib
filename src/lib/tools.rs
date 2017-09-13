extern crate std;

use result::Fb2Result;
use result::Fb2Error;

pub fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(
        |window| window == needle,
    )
}

pub fn as_utf8(header: &Vec<u8>) -> Fb2Result<String> {
    match std::str::from_utf8(&header) {
        Ok(utf8) => Ok(String::from(utf8)),
        Err(err) => {
            println!(
                "Non UTF8 content. Was able to read only {} bytes.",
                err.valid_up_to()
            );
            Err(Fb2Error::UnableToMakeUtf8)
        }
    }
}
