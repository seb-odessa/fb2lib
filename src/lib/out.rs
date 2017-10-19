use zip::read::ZipFile;
use result::Fb2Result;
use tools::into_fb2;

pub fn file_info(file: &ZipFile) {
    println!(
        "{:16}{:10}{:10}",
        file.name(),
        file.size(),
        file.compressed_size()
    );
}

pub fn xml(_: &str, xml: String) -> Fb2Result<()> {
    println!("{}", xml);
    Ok(())
}

pub fn fb2(file_name: &str, xml: String) -> Fb2Result<()> {
    match into_fb2(xml) {
        Ok(fb2) => println!("{:#?}", fb2),
        Err(err) => println!("{}: {}", file_name, err),
    }
    Ok(())
}

pub fn info(file_name: &str, xml: String) -> Fb2Result<()> {
    match into_fb2(xml) {
        Ok(fb2) => println!("{:20}: {}", file_name, fb2),
        Err(err) => println!("{:20}: {}", file_name, err),
    }
    Ok(())
}
