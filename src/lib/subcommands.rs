
use tools;
use archive;
use result::Fb2Result;
use fb2parser::fb::FictionBook;
use zip::read::ZipFile;

pub fn do_ls(archive_name: &str) -> Fb2Result<()> {
    apply(archive_name, |file| {
        println!(
            "{:16}{:10}{:10}",
            file.name(),
            file.size(),
            file.compressed_size()
        );
    })
}

pub fn apply<F>(archive_name: &str, mut visitor: F) -> Fb2Result<()>
where
    F: FnMut(&mut ZipFile) -> (),
{
    let mut zip = archive::open(archive_name)?;
    for i in 0..zip.len() {
        let mut file: ZipFile = zip.by_index(i)?;
        visitor(&mut file);
    }
    Ok(())
}


pub fn do_desc(archive_name: &str, file_name: &str) -> Fb2Result<()> {
    let mut zip = archive::open(archive_name)?;
    let mut file = zip.by_name(file_name)?;
    let header = archive::load_header(&mut file)?;
    let xml = tools::as_utf8(&header)?;
    println!("{}", xml);
    Ok(())
}

pub fn do_fb(archive_name: &str, file_name: &str) -> Fb2Result<()> {
    let mut zip = archive::open(archive_name)?;
    let mut file = zip.by_name(file_name)?;
    let header = archive::load_header(&mut file)?;
    let xml = tools::as_utf8(&header)?;
    let fb = FictionBook::new(&xml)?;
    println!("{:#?}", fb);
    Ok(())
}

pub fn do_info(archive_name: &str, file_name: &str) -> Fb2Result<()> {
    let mut zip = archive::open(archive_name)?;
    let files = archive::find(&mut zip, file_name)?;
    for i in 0..files.len() {
        let mut file: ZipFile = zip.by_index(i)?;
        let header = archive::load_header(&mut file)?;
        let xml = tools::as_utf8(&header)?;
        match FictionBook::new(&xml) {
            Ok(fb) => println!("{}", tools::make_info(&fb.description)),
            Err(_) => println!("!!!!Can't parse {}", &file.name())
        }
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    const ARCHIVE_NAME: &str = "data/arch.zip";

    #[bench]
    fn bench_each_book_count(bencher: &mut Bencher) {
        bencher.iter(|| {
            let mut cnt: u32 = 0;
            apply(ARCHIVE_NAME, |_| { cnt += 1; }).unwrap();
            assert_eq!(5, cnt);;
        });
    }

    #[bench]
    fn bench_each_book_load_header(bencher: &mut Bencher) {
        let mut result = Ok(());
        bencher.iter(|| {
            result = apply(ARCHIVE_NAME, |mut book| {
                assert!(archive::load_header(&mut book).is_ok());
            });
        });
        assert!(result.is_ok());
    }

    #[bench]
    fn bench_each_book_load_as_utf8(bencher: &mut Bencher) {
        let mut result = Ok(());
        bencher.iter(|| {
            result = apply(ARCHIVE_NAME, |mut book| {
                let header = archive::load_header(&mut book);
                assert!(header.is_ok());
                assert!(tools::as_utf8(&header.unwrap()).is_ok());
            });
        });
        assert!(result.is_ok());
    }

    #[bench]
    fn bench_each_book_load_as_fb2(bencher: &mut Bencher) {
        let mut result = Ok(());
        bencher.iter(|| {
            result = apply(ARCHIVE_NAME, |mut book| {
                let header = archive::load_header(&mut book);
                assert!(header.is_ok());
                let xml = tools::as_utf8(&header.unwrap());
                assert!(xml.is_ok());
                assert!(FictionBook::new(&xml.unwrap()).is_ok());
            });
        });
        assert!(result.is_ok());
    }

}