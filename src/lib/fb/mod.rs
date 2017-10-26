pub mod util;
pub mod fictionbook;
pub mod description;
pub mod title_info;
pub mod document_info;
pub mod publish_info;
pub mod genre;
pub mod author;
pub mod book_title;
pub mod lang;
pub mod first_name;
pub mod last_name;
pub mod middle_name;
pub mod nickname;

pub use fb::fictionbook::FictionBook;
pub use fb::description::Description;
pub use fb::title_info::TitleInfo;
pub use fb::document_info::DocumentInfo;
pub use fb::publish_info::PublishInfo;
pub use fb::genre::Genre;
pub use fb::author::Author;
pub use fb::book_title::Booktitle;
pub use fb::lang::Lang;
pub use fb::first_name::FirstName;
pub use fb::middle_name::MiddleName;
pub use fb::last_name::LastName;
pub use fb::nickname::Nickname;

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
