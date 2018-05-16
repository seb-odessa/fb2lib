pub mod acess;
pub mod author;
pub mod lang;
pub mod genre;
pub mod title;
pub mod sequence;
pub mod header;
pub mod collector;
pub mod book;
pub mod name;
pub mod description;

use std::hash::Hash;
use std::collections::HashSet;

pub fn discover<T: Eq + Hash>(known: &mut HashSet<T>, discovered: &mut HashSet<T>, value: T) {
    if !known.contains(&value) {
        discovered.insert(value);
    }
}

pub fn merge<T: Eq + Hash + Clone>(known: &mut HashSet<T>, discovered: &mut HashSet<T>, count: &mut usize) {
    *known = known.union(&discovered).map(|s| s.clone()).collect();
    discovered.clear();
    *count = 0;
}