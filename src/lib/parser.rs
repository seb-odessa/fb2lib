
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
    #[test]
    fn get_encoding() {
        assert_eq!(true);
    }
}