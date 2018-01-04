#![feature(libc)]
#![feature(test)]
#![feature(const_fn)]
#![feature(type_ascription)]

#[cfg(bench)]
extern crate test;

extern crate zip;
extern crate libc;
extern crate time;
extern crate regex;
extern crate crypto;
extern crate rusqlite;
extern crate fb2parser;
extern crate crossbeam;
extern crate rustc_serialize;


pub mod result;
pub mod algorithm;
pub mod subcommands;

mod out;
mod sal;
mod tools;
mod iconv;
mod archive;
mod filesystem;
