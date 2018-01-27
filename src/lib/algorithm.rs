
use tools;
use fb2parser::FictionBook;
use archive;
use archive::{ZipArchive, ZipFile};
use regex::Regex;
use result::{Fb2Result, Fb2Error};
use std::error::Error;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use crossbeam;

pub trait Visitor<T> {
    fn visit(&mut self, target: &T);
}

pub fn visit(archive_name: &str, visitor: &mut Visitor<FictionBook>) -> Fb2Result<()> {
    let zip = archive::open(archive_name)?;
    let (sender, receiver) = channel();
    apply_and_collect(zip, "*.fb2", sender, tools::into_fb2)?;
    for fb2 in receiver.iter() {
        if let Some(book) = fb2.ok() {
            visitor.visit(&book);
        }
    }
    Ok(())
}

pub fn apply<F>(mut zip: ZipArchive, file_mask: &str, visitor: F) -> Fb2Result<()>
where
    F: Fn(String, String) -> () + Send + Copy,
{
    let re = make_regex(file_mask)?;
    crossbeam::scope(|scope| for i in 0..zip.len() {
        if let Some(mut file) = zip.by_index(i).ok() {
            if re.is_match(file.name()) {
                if let Some(xml) = archive::load_header(&mut file)
                    .and_then(tools::into_utf8)
                    .ok()
                {
                    let name = String::from(file.name());
                    scope.spawn(move || visitor(name, xml));
                }
            }
        }
    });
    Ok(())
}

pub fn apply_and_collect<F, R>(mut zip: ZipArchive, file_mask: &str, out: Sender<R>, visitor: F) -> Fb2Result<()>
where
    F: Fn(String) -> R + Send + Copy,
    R: Send
{
    let re = make_regex(file_mask)?;
    crossbeam::scope(|scope| for i in 0..zip.len() {
        if let Some(mut file) = zip.by_index(i).ok() {
            if re.is_match(file.name()) {
                if let Some(xml) = archive::load_header(&mut file)
                    .and_then(tools::into_utf8)
                    .ok()
                {
                    let channel = out.clone();
                    scope.spawn(move || channel.send(visitor(xml)));
                }
            }
        }
    });
    Ok(())
}

pub fn apply_to_xml<F>(mut zip: ZipArchive, file_mask: &str, mut visitor: F) -> Fb2Result<()>
where
    F: FnMut(String, String) -> (),
{
    let re = make_regex(file_mask)?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        if re.is_match(file.name()) {
            let name = String::from(file.name());
            let xml = archive::load_header(&mut file).and_then(tools::into_utf8)?;
            visitor(name, xml);
        }
    }
    Ok(())
}

pub fn apply_to_file<F>(mut zip: ZipArchive, file_mask: &str, mut visitor: F) -> Fb2Result<()>
where
    F: FnMut(&ZipFile) -> Fb2Result<()>,
{
    let re = make_regex(file_mask)?;
    for i in 0..zip.len() {
        let file = zip.by_index(i)?;
        if re.is_match(file.name()) {
            visitor(&file)?;
        }
    }
    Ok(())
}


pub fn make_regex(file_mask: &str) -> Fb2Result<Regex> {
    Regex::new(&wildcards_to_regex(file_mask)).map_err(|e| {
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
