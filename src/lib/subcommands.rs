use pipe;
use tools;
use archive;
use result::Fb2Result;
use result::Fb2Error;
use zip::read::ZipFile;
use std::error::Error;

use tools::as_utf8;
use tools::create_fb2;

use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::sync_channel;
use std::thread;

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
    archive::apply_all(zip, print_file_info)
}

fn print_desc(mut file: ZipFile) -> Fb2Result<()> {
    let xml = archive::load_xml(&mut file)?;
    println!("{}", xml);
    Ok(())
}

pub fn show_xml(archive_name: &str, file_name: &str) -> Fb2Result<()> {
    let zip = archive::open(archive_name)?;
    if file_name.is_empty() {
        archive::apply_all(zip, print_desc)
    } else {
        archive::apply_one(zip, file_name, print_desc)
    }
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
    if file_name.is_empty() {
        archive::apply_all(zip, print_fb)
    } else {
        archive::apply_one(zip, file_name, print_fb)
    }
}

fn print_info(mut file: ZipFile) -> Fb2Result<()> {
    match archive::load_fb2(&mut file) {
        Ok(fb) => println!("{}", tools::fmt_info(&fb.description)),
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
    if file_name.is_empty() {
        archive::apply_all(zip, print_info)
    } else {
        archive::apply_one(zip, file_name, print_info)
    }
}

fn is_done<T>(msg: &Result<T, Fb2Error>) -> bool {
    match *msg {
        Err(Fb2Error::Done) => true,
        _ => false,
    }
}

fn worker<I, O, F>(
    receiver: Receiver<Result<I, Fb2Error>>,
    sender: SyncSender<Result<O, Fb2Error>>,
    mut processor: F,
) where
    F: FnMut(Result<I, Fb2Error>) -> Result<O, Fb2Error>,
{
    let mut have_tasks = true;
    while have_tasks {
        let input = receiver.recv().expect("The channel is broken\n");
        have_tasks = !is_done(&input);
        let output = processor(input);
        sender.send(output).expect("The channel is broken\n");
    }
}

pub fn do_parse(archive_name: &str) -> Fb2Result<()> {
    let (sender1, receiver1) = sync_channel(100);
    let (sender2, receiver2) = sync_channel(100);
    let (sender3, receiver3) = sync_channel(100);

    thread::spawn(move || { worker(receiver1, sender2, pipe::converter); });
    thread::spawn(move || { worker(receiver2, sender3, pipe::maker); });

    let mut zip = archive::open(archive_name)?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let header = archive::load_header(&mut file);
        sender1.send(header).expect("The channel is broken\n");
        let msg = receiver3.recv().expect("The channel is broken\n");
        match msg {
            Ok(fb) => println!("{}", tools::fmt_book(&fb)),
            Err(Fb2Error::Done) => break,
            Err(err) => println!("!!! {} -> {}", file.name(), err.description()),
        }
    }
    sender1.send(Err(Fb2Error::Done)).expect(
        "The channel is broken\n",
    );
    Ok(())
}


///*************************************************************************************************************************//
#[cfg(test)]
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