pub use serde_xml_rs::deserialize;
pub use serde_xml_rs::Error;

pub type SerdeError = Error;

//  Элемент <sequence>
// Атрибуты
//     name (обязательный) - название серии;
//     number (опциональный) - номер книги в серии;
//     xml:lang (опциональный) - язык.
// Подчиненные элементы - НЕТ
// Подчинен
//     <title-info> - 0..n (любое число, опционально);
//     <src-title-info> - 0..n (любое число, опционально);
//     <publish-info> - 0..n (любое число, опционально).
#[derive(Debug, Deserialize, PartialEq)]
pub struct Sequence {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "number", default)]
    pub number: String,
    #[serde(rename = "xml:lang", default)]
    pub lang: String,
}

// Элемент <author>
// http://fictionbook.org/index.php?title=%D0%AD%D0%BB%D0%B5%D0%BC%D0%B5%D0%BD%D1%82_author&oldid=2752
//  Атрибуты - НЕТ
// Подчиненные элементы
//     <first-name> - 0..1 (один, обязателен при отсутствии <nickname>, иначе опционально) - имя;
//     <middle-name> - 0..1 (один, опционально) - отчество;
//     <last-name> - 0..1 (один, обязателен при отсутствии <nickname>, иначе опционально) - фамилия;
//     <nickname> - 0..1 (один, обязателен при отсутствии <first-name> и <last-name>, иначе опционально);
//     <home-page> - 0..n (любое число, опционально);
//     <email> - 0..n (любое число, опционально);
//     <id> - 0..1 (один, опционально) с версии 2.2 - идентификатор автора, присваивается библиотекой.
// Подчинен
//     <title-info> 1..n (любое число, один обязателен);
//     <src-title-info> 1..n (любое число, один обязателен) с версии 2.1;
//     <document-info> 1..n (любое число, один обязателен);
#[derive(Debug, Deserialize, PartialEq)]
pub struct Author {
    #[serde(rename = "first-name", default)]
    pub first_name: String,
    #[serde(rename = "middle-name", default)]
    pub middle_name: String,
    #[serde(rename = "last-name", default)]
    pub last_name: String,
    #[serde(rename = "nickname", default)]
    pub nick_name: String,
    #[serde(rename = "home-page", default)]
    pub home_page: String,
    #[serde(rename = "email", default)]
    pub email: String,
}

// Элемент <genre>
// http://fictionbook.org/index.php?title=%D0%AD%D0%BB%D0%B5%D0%BC%D0%B5%D0%BD%D1%82_genre&oldid=2774
// Example:
// <genre>adv_western</genre>
// <genre match="20">detective</genre>
type Genre = String;

// Элемент <title-info>
// http://fictionbook.org/index.php?title=%D0%AD%D0%BB%D0%B5%D0%BC%D0%B5%D0%BD%D1%82_title-info&oldid=2920
//  Атрибуты - НЕТ
// Подчиненные элементы
//     <genre> - 1..n (любое число, один обязaтелен);
//     <author> - 1..n (любое число, один обязaтелен);
//     <book-title> - 1 (один, обязателен);
//     <annotation> - 0..1 (один, опционально);
//     <keywords> - 0..1 (один, опционально);
//     <date> - 0..1 (один, опционально);
//     <coverpage> - 0..1 (один, опционально);
//     <lang> - 1 (один, обязателен);
//     <src-lang> - 0..1 (один, опционально);
//     <translator> - 0..n (любое число, опционально);
//     <sequence> - 0..n (любое число, опционально).
// Подчинен
//     <description> - 1 (один, обязателен)


#[derive(Debug, Deserialize, PartialEq)]
pub struct TitleInfo {
    #[serde(rename = "genre", default)]
    pub genre: Vec<Genre>,
    #[serde(rename = "author", default)]
    pub author: Vec<Author>,
    #[serde(rename = "book-title", default)]
    pub book_title: String,
    #[serde(rename = "date", default)]
    pub date: String,
    #[serde(rename = "translator", default)]
    pub translator: Vec<Author>,
    #[serde(rename = "sequence", default)]
    pub sequence: Vec<Sequence>,
    #[serde(rename = "lang", default)]
    pub lang: String,
    #[serde(rename = "src-lang", default)]
    pub src_lang: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct PublishInfo {
    #[serde(rename = "book-name", default)]
    pub book_name: String,
    #[serde(rename = "publisher", default)]
    pub publisher: String,
    #[serde(rename = "city", default)]
    pub city: String,
    #[serde(rename = "year", default)]
    pub year: String,
    #[serde(rename = "isbn", default)]
    pub isbn: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct DocumentInfo {
    #[serde(rename = "author", default)]
    pub author: Vec<Author>,
    #[serde(rename = "program-used", default)]
    pub program_used: String,
    #[serde(rename = "date", default)]
    pub date: String,
    #[serde(rename = "src-url", default)]
    pub src_url: Vec<String>,
    #[serde(rename = "src-ocr", default)]
    pub src_ocr: String,
    #[serde(rename = "version", default)]
    pub version: String,
    // #[serde(rename = "publisher", default)]
    // pub publisher: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Description {
    #[serde(rename = "title-info")]
    pub title_info: TitleInfo,
    // #[serde(rename = "document-info")]      src_title_info: OptionalTitleInfo,
    #[cfg(not(test))]
    #[serde(rename = "document-info")]
    pub document_info: DocumentInfo,
    #[cfg(not(test))]
    #[serde(rename = "publish-info", default)]
    pub publish_info: Vec<PublishInfo>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct FictionBook {
    #[serde(rename = "description")]
    pub description: Description,
}

