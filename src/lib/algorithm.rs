
use tools;
use archive;
use archive::ZipArchive;
use regex::Regex;
use result::{Fb2Result, Fb2Error};

use std::error::Error;
use std::sync::Mutex;
use std::collections::VecDeque;
//use std::sync::mpsc::Receiver;
//use std::sync::mpsc::SyncSender;
//use std::sync::mpsc::sync_channel;
//use std::thread;
//use std::sync::mpsc::channel;


//pub type ZipFile<'a> = Mutex<zip::read::ZipFile<'a>>;
pub type BoxedBytes = Mutex<Box<Vec<u8>>>;

#[allow(dead_code)]
enum Message<'a, T> {
    Quit,
    Skip(Fb2Error),
    Work(Fb2Result<T>),
    Task((Box<FnMut(BoxedBytes) -> Fb2Result<T> + 'a>, BoxedBytes)),
}


pub fn run<'a, F, O>(mut zip: ZipArchive, file_name: &str, worker: F) -> Fb2Result<()>
where
    F: FnMut(BoxedBytes) -> Fb2Result<O> + Copy + 'a,
{
    let mut deq: VecDeque<Message<O>> = VecDeque::new();
    let re = make_regex(file_name)?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        if re.is_match(file.name()) {
            let header = archive::load_header(&mut file)?;
            let arg = Mutex::new(Box::new(header));
            deq.push_back(Message::Task((Box::new(worker), arg)));
        }
    }
    Ok(())
}

pub fn apply<F>(mut zip: ZipArchive, file_name: &str, mut visitor: F) -> Fb2Result<()>
where
    F: FnMut(&str, String) -> Fb2Result<()>,
{
    let re = make_regex(file_name)?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        if re.is_match(file.name()) {
            let xml = archive::load_header(&mut file).and_then(tools::into_utf8)?;
            visitor(file.name(), xml)?;
        }
    }
    Ok(())
}

fn make_regex(file_name: &str) -> Fb2Result<Regex> {
    Regex::new(&wildcards_to_regex(file_name)).map_err(|e| {
        Fb2Error::Custom(String::from(e.description()))
    })
}

fn wildcards_to_regex(arg: &str) -> String {
    let reg = String::from("^") + arg + "$";
    reg.replace(".", "\\.")
        .replace("\\*", "\0")
        .replace("*", "(.*)")
        .replace("\0", "\\*")
        .replace("\\?", "\0")
        .replace("?", "(.{1})")
        .replace("\0", "\\?")
}


#[cfg(test)]
mod tests {
    extern crate regex;

    #[test]
    fn expand_asterix_to_regexp() {
        assert_eq!("^file\\.txt$", &super::wildcards_to_regex("file.txt"));
        assert_eq!("^file(.*)\\.txt$", &super::wildcards_to_regex("file*.txt"));
        assert_eq!(
            "^file\\*(.*)\\.txt$",
            &super::wildcards_to_regex("file\\**.txt")
        );
    }

    #[test]
    fn expand_question_to_regexp() {
        assert_eq!("^file\\.txt$", &super::wildcards_to_regex("file.txt"));
        assert_eq!(
            "^file(.{1})\\.txt$",
            &super::wildcards_to_regex("file?.txt")
        );
        assert_eq!(
            "^file\\?(.{1})\\.txt$",
            &super::wildcards_to_regex("file\\??.txt")
        );
    }

    #[test]
    fn regex_asterix() {
        let re = regex::Regex::new("^file(.*).txt$").unwrap();
        assert!(re.is_match("file.txt"));
        assert!(re.is_match("file_long_name.txt"));
        assert!(re.is_match("file*.txt"));
        assert!(re.is_match("file..txt"));
    }

    #[test]
    fn regex_question() {
        let re = regex::Regex::new("^file(.{1})txt$").unwrap();
        assert!(re.is_match("file.txt"));
        assert!(!re.is_match("filetxt"));
        assert!(re.is_match("file_txt"));
        assert!(re.is_match("file*txt"));
    }

    #[test]
    fn regex_user_input_asterix() {
        let re = regex::Regex::new(&super::wildcards_to_regex("fil*.txt")).unwrap();
        assert!(re.is_match("file.txt"));
        assert!(re.is_match("file1.txt"));
        assert!(re.is_match("file_with_long_name.txt"));
        assert!(re.is_match("filefile.txt"));
        assert!(re.is_match("file.txt.file.txt"));
    }

    #[test]
    fn regex_user_input_question() {
        let re = regex::Regex::new(&super::wildcards_to_regex("fil??txt")).unwrap();
        assert!(re.is_match("file.txt"));
        assert!(re.is_match("fil__txt"));
        assert!(!re.is_match("file_with_long_name.txt"));
        assert!(!re.is_match("filefile.txt"));
        assert!(!re.is_match("file.txt.file.txt"));
    }

    #[test]
    fn regex_user_input_wo_wildcards() {
        let re = regex::Regex::new(&super::wildcards_to_regex("file.txt")).unwrap();
        assert!(re.is_match("file.txt"));
        assert!(!re.is_match(".file.txt"));
        assert!(!re.is_match("file.txt."));
        assert!(!re.is_match("fil__txt"));
        assert!(!re.is_match("file_with_long_name.txt"));
        assert!(!re.is_match("filefile.txt"));
        assert!(!re.is_match("file.txt.file.txt"));
    }
}
