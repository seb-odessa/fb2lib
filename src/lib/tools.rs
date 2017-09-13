extern crate std;

use result::Fb2Result;
use result::Fb2Error;
use iconv::Converter;


pub fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(
        |window| window == needle,
    )
}

pub fn from(input: &Vec<u8>) -> () {
    const UTF8: &str = "utf-8";
    const WIN_RU: &str = "windows-1251";
    let mut out = Vec::new();
    out.resize(2 * input.len(), 0u8);
    let conv = Converter::new(WIN_RU, UTF8);
    let (a, b, c) = conv.convert(&input, &mut out);
    println!("({}, {}, {})", a,b,c);
    println!("{}", std::str::from_utf8(&out).unwrap());
}

pub fn as_utf8(header: &Vec<u8>) -> Fb2Result<String> {
    match std::str::from_utf8(&header) {
        Ok(utf8) => Ok(String::from(utf8)),
        Err(err) => {
            println!(
                "Non UTF8 content. Was able to read only {} bytes.",
                err.valid_up_to()
            );
            from(header);
            Err(Fb2Error::UnableToMakeUtf8)
        }
    }
}
