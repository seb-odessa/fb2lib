pub mod util;
pub mod fictionbook;
pub use fb::fictionbook::FictionBook;
pub mod description;
pub use fb::description::Description;
pub mod title_info;
pub use fb::title_info::TitleInfo;
pub mod document_info;
pub use fb::document_info::DocumentInfo;
pub mod publish_info;
pub use fb::publish_info::PublishInfo;
pub mod genre;
pub use fb::genre::Genre;
pub mod author;
pub use fb::author::Author;
pub mod translator;
pub use fb::translator::Translator;
pub mod book_title;
pub use fb::book_title::BookTitle;
pub mod lang;
pub use fb::lang::Lang;
pub mod src_lang;
pub use fb::src_lang::SrcLang;
pub mod first_name;
pub use fb::first_name::FirstName;
pub mod last_name;
pub use fb::last_name::LastName;
pub mod middle_name;
pub use fb::middle_name::MiddleName;
pub mod nickname;
pub use fb::nickname::Nickname;
pub mod sequence;
pub use fb::sequence::Sequence;
pub mod program_used;
pub use fb::program_used::ProgramUsed;
pub mod date;
pub use fb::date::Date;
pub mod publisher;
pub use fb::publisher::Publisher;
pub mod book_name;
pub use fb::book_name::BookName;
pub mod city;
pub use fb::city::City;
pub mod year;
pub use fb::year::Year;
pub mod isbn;
pub use fb::isbn::Isbn;

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
