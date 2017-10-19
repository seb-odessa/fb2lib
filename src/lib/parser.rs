extern crate xmltree;

#[derive(Debug, PartialEq, Default)]
pub struct FictionBook {
    pub description: Description,
}

#[derive(Debug, PartialEq, Default)]
pub struct Description {
    pub title_info: TitleInfo, // title-info
    pub document_info: DocumentInfo, // document-info
    pub publish_info: Vec<PublishInfo>, // publish-info
}

#[derive(Debug, PartialEq, Default)]
pub struct TitleInfo {
    pub genre: Vec<Genre>, // genre
    pub author: Vec<Author>, // author
    pub book_title: String, // book-title
    pub annotation: String, // annotation
    pub keywords: String, // keywords
    pub date: String, // data
    pub lang: String, // lang
    pub src_lang: String, // src-lang
    pub translator: Vec<Translator>, // translator
    pub sequence: Vec<Sequence>, // sequence
}

#[derive(Debug, PartialEq, Default)]
pub struct DocumentInfo {
    pub author: Vec<Author>, // author
    pub program_used: String, // program-used
    pub date: String, // date
    pub src_url: Vec<String>, // src-url
    pub src_ocr: String, // src-ocr
    pub version: String, // version
    pub publisher: Vec<String>, // publisher
}

#[derive(Debug, PartialEq, Default)]
pub struct PublishInfo {
    pub book_name: String, // book-name
    pub publisher: String, // publisher
    pub city: String, // city
    pub year: String, // year
    pub isbn: String, // isbn
}

type Genre = String;

#[derive(Debug, PartialEq, Default)]
pub struct Author {
    pub first_name: String, // first-name
    pub middle_name: String, // middle-name
    pub last_name: String, // last-name
    pub nick_name: String, // nickname
    pub home_page: String, // home-page
    pub email: String, // email
}

#[derive(Debug, PartialEq, Default)]
pub struct Translator {
    pub first_name: String, // first-name
    pub middle_name: String, // middle-name
    pub last_name: String, // last-name
    pub nick_name: String, // nickname
    pub home_page: String, // home-page
    pub email: String, // email
}

#[derive(Debug, PartialEq, Default)]
pub struct Sequence {
    pub name: String, // attr name
    pub number: String, // attr number
    pub lang: String, // attr xml:lang
}

#[cfg(test)]
mod tests {
    use data::bench::XML;
    use super::xmltree::Element;

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
    use super::xmltree::{Element, ParseError};

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