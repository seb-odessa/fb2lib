use zip::ZipFile;
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

pub fn xml(_: String, xml: String) {
    println!("{}", xml);
}

pub fn fb2(file_name: String, xml: String) {
    match into_fb2(xml) {
        Ok(fb2) => println!("{:#?}", fb2),
        Err(err) => println!("{}: {}", file_name, err),
    }
}

fn make_authors(authors: Vec<(String,String,String,String)>) -> String {
    let mut result = Vec::new();
    for author in authors.into_iter() {
        result.push(format!("{} {} {} {}", author.0, author.1, author.2, author.3));        
    }
    result.join(", ")
}

pub fn info(file_name: String, xml: String) {
    match into_fb2(xml) {
        Ok(fb2) => println!("{:12} : {} : {} : {} : {}", 
            file_name, 
            fb2.get_book_title(), 
            make_authors(fb2.get_book_authors()),
            fb2.get_book_lang(),
            fb2.get_book_genres().join(", ")
        ),
        Err(err) => println!("{:20}: {}", file_name, err),
    }
}

pub fn zip_info(file: &ZipFile) -> Fb2Result<()> {
    println!(
        "{:10} ({:10}) : {:8}/{:8} crc32: {:12}, offset: {}",
        &file.name(),
        &file.compression(),
        &file.compressed_size(),
        &file.size(),

        &file.crc32(),
        &file.offset()
    );
    Ok(())
}
