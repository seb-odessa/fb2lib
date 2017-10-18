#![feature(libc)]
#![feature(test)]
#![feature(const_fn)]
#![feature(type_ascription)]

#[cfg(bench)]
extern crate test;

extern crate zip;
extern crate libc;
extern crate regex;

#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

pub mod result;
pub mod algorithm;
pub mod subcommands;

mod fb;
mod tools;
mod iconv;
mod helper;
mod archive;
mod data;
