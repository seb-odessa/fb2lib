use std::fmt;
use xmltree::Element;
use result::{Fb2Error, Fb2Result};


#[derive(Debug, PartialEq)]
pub struct FictionBook {
    pub fb: Box<Element>,
}
impl FictionBook {
    pub fn new(xml: &[u8]) -> Fb2Result<Self> {
        match Element::parse(xml) {
            Ok(fb) => Ok(FictionBook { fb: Box::new(fb) }),
            Err(e) => Err(Fb2Error::Custom(format!("{}", e))),
        }
    }

    pub fn find_element(&self, path: &str) -> Option<&Element> {
        None
    }
}

impl fmt::Display for FictionBook {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", "FictionBook")
    }
}



#[cfg(test)]
mod tests {
    use data::bench::XML;
    use xmltree::Element;

    #[test]
    fn get_description() {
        let mut xmltree = Element::parse(XML.as_bytes()).unwrap();
        let description = xmltree.get_mut_child("description");
        assert!(description.is_some());
    }

    #[test]
    fn get_title_info() {
        let mut xmltree = Element::parse(XML.as_bytes()).unwrap();
        let description = xmltree.get_mut_child("description").unwrap();
        let title_info = description.get_mut_child("title-info");
        assert!(title_info.is_some());
    }

    #[test]
    fn get_genre() {
        let mut xmltree = Element::parse(XML.as_bytes()).unwrap();
        let description = xmltree.get_mut_child("description").unwrap();
        let title_info = description.get_mut_child("title-info").unwrap();
        let genre = title_info.get_mut_child("genre");
        assert!(genre.is_some());
        assert_eq!(Some(String::from("sf_space")), genre.unwrap().text);
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