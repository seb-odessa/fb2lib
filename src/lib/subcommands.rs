
use tools;
use archive;
use result::Fb2Result;
use result::Fb2Error;
use zip::read::ZipFile;
use std::error::Error;

use tools::as_utf8;
use tools::create_fb2;


pub fn do_ls(archive_name: &str) -> Fb2Result<()> {
    let zip = archive::open(archive_name)?;
    archive::apply_all(zip, |file| {
        println!(
            "{:16}{:10}{:10}",
            file.name(),
            file.size(),
            file.compressed_size()
        );
        Ok(())
    })
}

fn print_desc(mut file: ZipFile) -> Fb2Result<()> {
    let xml = archive::load_xml(&mut file)?;
    println!("{}", xml);
    Ok(())
}

pub fn do_desc(archive_name: &str, file_name: &str) -> Fb2Result<()> {
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

pub fn do_fb(archive_name: &str, file_name: &str) -> Fb2Result<()> {
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

pub fn do_info(archive_name: &str, file_name: &str) -> Fb2Result<()> {
    let zip = archive::open(archive_name)?;
    if file_name.is_empty() {
        archive::apply_all(zip, print_info)
    } else {
        archive::apply_one(zip, file_name, print_info)
    }
}

pub fn do_parse(archive_name: &str) -> Fb2Result<()> {
    

//     let (sender1, receiver1) = sync_channel(1);
//     let (sender2, receiver2) = sync_channel(1);
// //    let (sender3, receiver3) = sync_channel(1);
// //    let (sender4, receiver4) = sync_channel(1);

//     thread::spawn(move|| {
//         loop {
//             let msg = receiver1.recv().expect("The receiver1 is broken\n");
//             match msg {
//                 Ok(file) => sender2.send(load_header(file)).expect("The sender2 is broken\n"),
//                 Err(Fb2Error::Done) => break,
//                 Err(err) => 
                
//             }
//         }
//     });

    // thread::spawn(move|| { 
    //     let arg = receiver2.recv().unwrap();
        
    //     sender3.send(temp).unwrap(); 
    // });

    // thread::spawn(move|| { 
    //     let arg: String = receiver3.recv().unwrap();
    //     let temp = make_fb(arg);
    //     sender4.send(temp).unwrap(); 
    // });
   
    let zip = archive::open(archive_name)?;
    // for i in 0..zip.len() {
    //     let file = archive.by_index(i)?;
    //     sender1.send(Ok(&mut file)).unwrap(); 
    // }

    archive::apply_all(zip, |file| {
        println!(
            "{:16}{:10}{:10}",
            file.name(),
            file.size(),
            file.compressed_size()
        );
        Ok(())
    })
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