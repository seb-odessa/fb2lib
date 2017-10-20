use std::fmt;
use xmltree::Element;
use result::{Fb2Error, Fb2Result};


#[derive(Debug, PartialEq)]
pub struct FictionBook {
    pub root: Box<Element>,
}
impl FictionBook {
    pub fn new(xml: &[u8]) -> Fb2Result<Self> {
        match Element::parse(xml) {
            Ok(fb) => Ok(FictionBook { root: Box::new(fb) }),
            Err(e) => Err(Fb2Error::Custom(format!("{}", e))),
        }
    }

    pub fn get_book_title(&self) -> Option<String> {
        self.query("description/title-info/book-title").map_or(
            None,
            |ref e| {
                e.text.clone()
            },
        )
    }

    pub fn query_path<'a>(root: &Option<&'a Element>, path: &[&str]) -> Option<&'a Element> {
        if let &Some(node) = root {
            let len = path.len();
            if len == 0 {
                return None;
            } else if len == 1 {
                return node.get_child(path[0]);
            } else {
                return FictionBook::query_path(&node.get_child(path[0]), &path[1..]);
            }
        }
        None
    }

    #[allow(dead_code)]
    pub fn query(&self, path: &str) -> Option<&Element> {
        let nodes: Vec<&str> = path.split('/').collect::<Vec<_>>();
        FictionBook::query_path(&Some(&self.root), &nodes)
    }
}

impl fmt::Display for FictionBook {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.get_book_title().unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use data::bench::XML;
    use xmltree::Element;

    #[test]
    fn description() {
        let xmltree = Element::parse(XML.as_bytes()).unwrap();
        let description = xmltree.get_child("description");
        assert!(description.is_some());
    }

    #[test]
    fn title_info() {
        let xmltree = Element::parse(XML.as_bytes()).unwrap();
        let description = xmltree.get_child("description").unwrap();
        let title_info = description.get_child("title-info");
        assert!(title_info.is_some());
    }

    #[test]
    fn genre() {
        let xmltree = Element::parse(XML.as_bytes()).unwrap();
        let description = xmltree.get_child("description").unwrap();
        let title_info = description.get_child("title-info").unwrap();
        let genre = title_info.get_child("genre");
        assert!(genre.is_some());
        assert_eq!(Some(String::from("sf_space")), genre.unwrap().text);
    }

    #[test]
    fn fb_query() {
        use super::FictionBook;
        let fb = FictionBook::new(XML.as_bytes()).unwrap();
        assert!(fb.query("description").is_some());
        assert!(fb.query("description/title-info").is_some());
        assert!(fb.query("description/title-info/author").is_some());
    }

}

#[cfg(test)]
mod bench {
    extern crate test;
    use self::test::Bencher;
    use data::bench::*;
    use xmltree::{Element, ParseError};

    #[bench]
    fn parse_fiction_book(bencher: &mut Bencher) {
        let xml = XML.as_bytes();
        bencher.iter(|| {
            let _: Result<Element, ParseError> = Element::parse(xml);
        });
    }
    #[bench]
    fn parse_description(bencher: &mut Bencher) {
        let xml = DESCRIPTION.as_bytes();
        bencher.iter(|| {
            let _: Result<Element, ParseError> = Element::parse(xml);
        });
    }

    #[bench]
    fn parse_title_info(bencher: &mut Bencher) {
        let xml = TITLE_INFO.as_bytes();
        bencher.iter(|| {
            let _: Result<Element, ParseError> = Element::parse(xml);
        });
    }

    #[bench]
    fn parse_author(bencher: &mut Bencher) {
        let xml = AUTHOR.as_bytes();
        bencher.iter(|| {
            let _: Result<Element, ParseError> = Element::parse(xml);
        });
    }

}