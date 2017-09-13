extern crate std;
extern crate zip;

use tools;
use archive;
use result::Fb2Result;

pub fn do_ls(archive_name: &str) -> Fb2Result<()> {
    let mut zip = archive::open(archive_name)?;
    for i in 0..zip.len() {
        let file = zip.by_index(i)?;
        println!(
            "{:16} {:10} {:12}",
            file.name(),
            file.compressed_size(),
            file.size()
        );
    }
    Ok(())
}

pub fn do_info(archive_name: &str, file_name: &str) -> Fb2Result<()> {
    let mut zip = archive::open(archive_name)?;
    let mut file = zip.by_name(file_name)?;
    let header = archive::load_header(&mut file)?;
    let description = tools::as_utf8(&header)?;
    println!("{}", description);
    Ok(())
}
