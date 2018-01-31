use ui;
use sal;
use result::Fb2Result;
use clap::{App, Arg, SubCommand, ArgMatches};

pub const CMD: &'static str = "book";
const CMD_HELP: &'static str = "Use to work with books";

const LOAD: &'static str = "load";
const LOAD_HELP: &'static str = "Load books from archive into the database";

pub fn add<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    let db = Arg::with_name(ui::DB_FILE).help(ui::DB_FILE_HELP).required(false);
    let archive = Arg::with_name(ui::ARCH_FILE).help(ui::ARCH_FILE_HELP).required(true).multiple(true);

    app.subcommand(
        SubCommand::with_name(CMD).about(CMD_HELP).arg(db)
        .subcommand(SubCommand::with_name(LOAD).about(LOAD_HELP).arg(archive.clone()))
    )
}

pub fn handle<'a>(arg: &ArgMatches<'a>) -> Fb2Result<()> {
    let database = arg.value_of(ui::DB_FILE).unwrap_or(ui::DB_FILE);
    match arg.subcommand() {
        (LOAD, Some(arg)) => {
            if let Some(archives) = arg.values_of(ui::ARCH_FILE) {
                load(database, &archives.collect::<Vec<&str>>())
            } else {
                ui::usage(arg)
            }
        },
        (_, _) => ui::usage(arg)
    }
}

pub fn load(db: &str, archives: &Vec<&str>) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    //let mut collector = GenreCollector::new();
    for archive in archives {
        println!("{}", archive);
        //algorithm::visit(archive, &mut collector)?;
    }
    Ok(())
}
