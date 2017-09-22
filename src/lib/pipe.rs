use tools;
use fb2parser::fb::FictionBook;
use result::Fb2Result;

pub fn converter(arg: Fb2Result<Vec<u8>>) -> Fb2Result<String> {
    match arg {
        Ok(buffer) => tools::as_utf8(buffer),
        Err(err) => Err(err),
    }
}

pub fn maker(arg: Fb2Result<String>) -> Fb2Result<FictionBook> {
    match arg {
        Ok(xml) => tools::create_fb2(xml),
        Err(err) => Err(err),
    }
}