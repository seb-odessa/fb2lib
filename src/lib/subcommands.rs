use tools;
use archive;
use result::Fb2Result;
use result::Fb2Error;
use zip::read::ZipFile;
use std::error::Error;

use tools::as_utf8;
use tools::create_fb2;

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io;

fn print_file_info(file: ZipFile) -> Fb2Result<()> {
    println!(
        "{:16}{:10}{:10}",
        file.name(),
        file.size(),
        file.compressed_size()
    );
    Ok(())
}

pub fn do_ls(archive_name: &str) -> Fb2Result<()> {
    let zip = archive::open(archive_name)?;
    archive::apply(zip, "*", print_file_info)
}

fn print_desc(mut file: ZipFile) -> Fb2Result<()> {
    let xml = archive::load_xml(&mut file)?;
    println!("{}", xml);
    Ok(())
}

pub fn show_xml(archive_name: &str, file_name: &str) -> Fb2Result<()> {
    let zip = archive::open(archive_name)?;
    archive::apply(zip, file_name, print_desc)
}

fn print_fb(mut file: ZipFile) -> Fb2Result<()> {
    let fb = archive::load_header(&mut file).and_then(as_utf8).and_then(
        create_fb2,
    )?;
    println!("{:#?}", fb);
    Ok(())
}

pub fn show_fb2(archive_name: &str, file_name: &str) -> Fb2Result<()> {
    let zip = archive::open(archive_name)?;
    archive::apply(zip, file_name, print_fb)
}

fn print_info(mut file: ZipFile) -> Fb2Result<()> {
    match archive::load_fb2(&mut file) {
        Ok(fb) => println!("{:20}: {}", file.name(), tools::fmt_info(&fb.description)),
        Err(err) => {
            println!(
                "Can't parse {} with error {} ",
                file.name(),
                err.description()
            )
        }
    }
    Ok(())
}

pub fn show_inf(archive_name: &str, file_name: &str) -> Fb2Result<()> {
    let zip = archive::open(archive_name)?;
    archive::apply(zip, file_name, print_info)
}

fn read_file(file_name: &str) -> io::Result<Vec<u8>> {
    let mut buffer = Vec::new();
    let mut file = File::open(file_name)?;
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn do_parse(file_name: &str) -> Fb2Result<()> {
    let fb = match read_file(file_name) {
        Ok(xml) => tools::as_utf8(xml).and_then(tools::create_fb2),
        Err(_) => Err(Fb2Error::FileNotFound(String::from(file_name))),
    }?;
    tools::fmt_book(&fb);
    Ok(())
}

pub fn do_check(archive_name: &str) -> Fb2Result<()> {
    let mut zip = archive::open(archive_name)?;
    let book_count = zip.len();
    let mut succ = 0;
    print!("Progress:   %");
    for i in 0..book_count {
        let mut file = zip.by_index(i)?;
        match archive::load_fb2(&mut file) {
            Ok(_) => succ += 1,
            Err(_) => println!("\n{}", file.name()),
        }
        print!("\r");
        print!("Progress: {:3}%", 100 * (1 + i) / book_count);
        io::stdout().flush()?;
    }
    println!(
        "\nSucceeded {}/{} ({}%)",
        succ,
        book_count,
        100 * succ / book_count
    );
    Ok(())
}

///*************************************************************************************************************************//
#[cfg(bench)]
mod tests {
    use super::*;
    use test::Bencher;
    const ARCHIVE_NAME: &str = "data/arch.zip";

    #[bench]
    fn bench_each_book_count(bencher: &mut Bencher) {
        bencher.iter(|| {
            let mut cnt: u32 = 0;
            let zip = archive::open(ARCHIVE_NAME);
            assert!(zip.is_ok());
            archive::apply_all(zip.unwrap(), |_| {
                cnt += 1;
                Ok(())
            }).unwrap();
            assert_eq!(5, cnt);;
        });
    }

    #[bench]
    fn bench_each_book_load_header(bencher: &mut Bencher) {
        let mut result = Ok(());

        bencher.iter(|| {
            let zip = archive::open(ARCHIVE_NAME);
            assert!(zip.is_ok());
            result = archive::apply_all(zip.unwrap(), |mut book| {
                assert!(archive::load_header(&mut book).is_ok());
                Ok(())
            });
        });
        assert!(result.is_ok());
    }

    #[bench]
    fn bench_each_book_load_as_xml(bencher: &mut Bencher) {
        let mut result = Ok(());
        bencher.iter(|| {
            let zip = archive::open(ARCHIVE_NAME);
            assert!(zip.is_ok());
            result = archive::apply_all(zip.unwrap(), |mut book| {
                assert!(archive::load_xml(&mut book).is_ok());
                Ok(())
            });
        });
        assert!(result.is_ok());
    }

    #[bench]
    fn bench_each_book_load_as_fb2(bencher: &mut Bencher) {
        let mut result = Ok(());
        bencher.iter(|| {
            let zip = archive::open(ARCHIVE_NAME);
            assert!(zip.is_ok());
            result = archive::apply_all(zip.unwrap(), |mut book| {
                assert!(archive::load_fb2(&mut book).is_ok());
                Ok(())
            });
        });
        assert!(result.is_ok());
    }

}