use result;
use clap::ArgMatches;

pub mod archive;
pub mod database;
pub mod torrent;

pub fn usage<'a>(args: &ArgMatches<'a>) -> result::Fb2Result<()> {
    println!("{}", args.usage());
    Ok(())
}
