use zip::read::ZipFile;
use result::Fb2Result;
use archive::{load_header, load_xml, load_fb2};
use tools::{into_utf8, into_fb2};
use std::error::Error;


pub fn file_info(file: &mut ZipFile) -> Fb2Result<()> {
    println!(
        "{:16}{:10}{:10}",
        file.name(),
        file.size(),
        file.compressed_size()
    );
    Ok(())
}

pub fn xml(file: &mut ZipFile) -> Fb2Result<()> {
    let xml = load_xml(file)?;
    println!("{}", xml);
    Ok(())
}

pub fn fb2(file: &mut ZipFile) -> Fb2Result<()> {
    let fb = load_header(file).and_then(into_utf8).and_then(into_fb2)?;
    println!("{:#?}", fb);
    Ok(())
}

pub fn info(file: &mut ZipFile) -> Fb2Result<()> {
    match load_fb2(file) {
        Ok(fb) => println!("{:20}: {}", &file.name(), fb),
        Err(err) => println!("{:20}: {} !!!", &file.name(), err.description()),
    }
    Ok(())
}
