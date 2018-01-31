use result::Fb2Result;
use clap::{App, ArgMatches};

pub mod archive;
pub mod database;
pub mod torrent;
pub mod filter;
// pub mod genre;
// pub mod lang;
pub mod book;

pub struct Adapter <'a, 'b>  where 'a: 'b {
    app: App<'a, 'b>
}
impl <'a, 'b> Adapter <'a, 'b> {
    pub fn new(app: App<'a, 'b>) -> Self {
        Adapter{ app }
    }

    pub fn attach<F>(self, add: F) -> Self where F: Fn(App<'a, 'b>) -> App<'a, 'b> {
        Adapter{ app: add(self.app) }
    }

    pub fn unwrap(self) -> App<'a, 'b> {
        self.app
    }
}


pub fn usage<'a>(args: &ArgMatches<'a>) -> Fb2Result<()> {
    println!("{}", args.usage());
    Ok(())
}

pub const DB_FILE: &'static str = "lib.rus.ec.db";
pub const DB_FILE_HELP: &'static str = "Database file name";
pub const ARCH_FILE: &'static str = "archive.zip";
pub const ARCH_FILE_HELP: &'static str = "Archive file name with books in FB2 format";
pub const BOOK_FILE: &'static str = "book.fb2";
pub const BOOK_FILE_HELP: &'static str = "Book's file name in the archive";
pub const TORRENT_FILE: &'static str = "file.torrent";
pub const TORRENT_FILE_HELP: &'static str = "Torrent file name points to the archive.zip";


