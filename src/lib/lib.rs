#![feature(libc)]
#![feature(test)]
#![feature(const_fn)]
#![feature(type_ascription)]

#[cfg(bench)]
extern crate test;

extern crate zip;
extern crate torrent;
extern crate fb2parser;

extern crate libc;
extern crate time;
extern crate clap;
extern crate regex;
extern crate crypto;
extern crate rusqlite;
extern crate crossbeam;
extern crate rustc_serialize;

pub mod ui;
pub mod result;
pub mod algorithm;

mod out;
mod sal;
mod tools;
mod iconv;
mod handler;
mod visitor;
mod archive;
mod filesystem;
