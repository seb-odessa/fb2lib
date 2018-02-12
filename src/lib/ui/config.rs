use ui;
//use handler;
use result::Fb2Result;

use clap::{App, Arg, SubCommand, ArgMatches};

pub const CMD: &'static str = "config";
const CMD_HELP: &'static str = "Use to work manage archives paths";
const NAME: &'static str = "name";
const NAME_HELP: &'static str = "The name of the path";
const PATH: &'static str = "path";
const PATH_HELP: &'static str = "The path for archives";

const ADD: &'static str = "add";
const ADD_HELP: &'static str = "Add named path to the configuration";
const REG: &'static str = "register";
const REG_HELP: &'static str = "Register arhive(s) in the selected path";
const CHK: &'static str = "check";
const CHK_HELP: &'static str = "Check existing all registred archives in the selected path";
const DSP: &'static str = "display";
const DSP_HELP: &'static str = "Display registred paths";
const SET: &'static str = "set";
const SET_HELP: &'static str = "Set named path as current for the configuration";


pub fn add<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    let name = Arg::with_name(NAME).help(NAME_HELP).required(true);
    let path = Arg::with_name(PATH).help(PATH_HELP).required(true);
    let arch = Arg::with_name(ui::ARCH_FILE).help(ui::ARCH_FILE_HELP).required(true);

    app.subcommand(
        SubCommand::with_name(CMD).about(CMD_HELP)
        .subcommand(SubCommand::with_name(ADD).about(ADD_HELP).arg(name.clone()).arg(path.clone()))
        .subcommand(SubCommand::with_name(REG).about(REG_HELP).arg(arch.clone()))
        .subcommand(SubCommand::with_name(CHK).about(CHK_HELP))
        .subcommand(SubCommand::with_name(DSP).about(DSP_HELP))
        .subcommand(SubCommand::with_name(SET).about(SET_HELP).arg(arch.clone()))
    )
}

pub fn handle<'a>(arg: &ArgMatches<'a>) -> Fb2Result<()> {
    //let archive = arg.value_of(ui::ARCH_FILE).unwrap_or("").to_string();
    match arg.subcommand() {
        (_, _) => {
            ui::usage(arg)
        }
    }
}

