extern crate zip;
extern crate lib;
extern crate clap;

use clap::{Arg, App, SubCommand};
use lib::arch;
use std::error::Error;

const VERSION: &'static str = "v0.1.0";
const AUTHOR: &'static str = "seb <seb@ukr.net>";

fn main()
{
    let arguments: Vec<String> = std::env::args().collect();
    let program = std::path::Path::new(&arguments[0]).file_name().unwrap().to_str().unwrap();
    let matches = App::new(program)
                          .version(VERSION)
                          .author(AUTHOR)
                          .about("FictionBook 2.0 Database CLI tool")
                          .arg(Arg::with_name("database")
                               .short("D")
                               .long("database")
                               .value_name("FILE")
                               .help("Sets a custom database file, e.g.: fb2lib.db")
                               .takes_value(true))
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .subcommand(SubCommand::with_name("show")
                                      .about("List content of the zip container")
                                      .version(VERSION)
                                      .author(AUTHOR)
                                      .arg(Arg::with_name("file.zip")
                                        .help("Container with FB2 files, e.g.: fb2-618000-620999.zip")
                                        .required(true)
                                        .index(1)))
                          .subcommand(SubCommand::with_name("load")
                                      .about("Load content of the zip container into DB")
                                      .version(VERSION)
                                      .author(AUTHOR)
                                      .arg(Arg::with_name("file.zip")
                                        .help("Container with FB2 files, e.g.: fb2-618000-620999.zip")
                                        .required(true)
                                        .index(1)))
                          .get_matches();

    let verbose = match matches.occurrences_of("v") {
        0 => false,
        1 => true,
        _ => { println!("Only one verbose level implemented."); true }
    };

    let database = matches.value_of("database").unwrap_or("fb2lib.db");
    if verbose {
        println!("FictionBook 2.0 Library DataBase: {}", database);
    }

    if let Some(matches) = matches.subcommand_matches("show") {
        let filename = matches.value_of("file.zip").unwrap();
        if verbose {
            println!("Using input ZIP file: {}", filename);
        }
        match arch::open_zip(&filename) {
            Ok(mut archive) => arch::show(&mut archive),
            Err(e) => println!("Caused by: {}", e.cause().unwrap())
        }
    }
}


