extern crate lib;
extern crate clap;

use clap::{App, AppSettings};
use lib::ui;

// use lib::ui::archive;
// use lib::ui::database;
// use lib::ui::torrent;
// use lib::ui::filter;
// use lib::ui::Adapter;

const VERSION: &'static str = "v0.5.5";
const AUTHOR: &'static str = "seb <seb@ukr.net>";
const ABOUT: &'static str = "FictionBook Library Archive Manager";


fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let program = std::path::Path::new(&arguments[0])
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    let app = App::new(program)
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT)
        .setting(AppSettings::ArgRequiredElseHelp);

    let app = ui::Adapter::new(app)
                .attach(ui::archive::add)
                .attach(ui::database::add)
                .attach(ui::torrent::add)
                .attach(ui::filter::add)
                .unwrap();
    //------------------------------------------------------------------------------------------------------//
    let matches = app.get_matches();
    let result = match matches.subcommand() {
        (ui::archive::CMD, Some(arg)) => ui::archive::handle(arg),
        (ui::database::CMD, Some(arg)) => ui::database::handle(arg),
        (ui::torrent::CMD, Some(arg)) => ui::torrent::handle(arg),
        (ui::filter::CMD, Some(arg)) => ui::filter::handle(arg),
        (_,_) => {
            matches.usage();
            Ok(())
        }
    };
    //------------------------------------------------------------------------------------------------------//
    if result.is_err() {
        println!("Error: {}", result.unwrap_err());
    }
}
