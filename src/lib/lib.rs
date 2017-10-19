#![feature(libc)]
#![feature(test)]
#![feature(const_fn)]
#![feature(type_ascription)]

#[cfg(bench)]
extern crate test;

extern crate zip;
extern crate libc;
extern crate regex;
extern crate xmltree;


pub mod result;
pub mod algorithm;
pub mod subcommands;

mod out;
mod tools;
mod iconv;
mod archive;
mod parser;
mod data;
