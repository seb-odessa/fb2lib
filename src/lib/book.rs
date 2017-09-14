// source: http://fictionbook.org/index.php/%D0%9E%D0%BF%D0%B8%D1%81%D0%B0%D0%BD%D0%B8%D0%B5_%D1%84%D0%BE%D1%80%D0%BC%D0%B0%D1%82%D0%B0_FB2_%D0%BE%D1%82_Sclex


type OptionalId = Option<u32>;
type OptionalName = Option<Name>;
type OptionalString = Option<String>;
type OptionalTitleInfo = Option<TitleInfo>;
type OptionalAnnotation = Option<Annotation>;

pub struct Genre {
    attr_match: u8,
    value: String,
}

pub struct Name {
    attr_lang: OptionalString,
    value: String,
}

pub struct Annotation {
    attr_id: OptionalId,
    attr_lang: OptionalString,
    value: String,
}

pub struct Sequence {
    attr_num: OptionalId,
    attr_name: OptionalString,
    attr_lang: OptionalString,
}

pub struct Author {
    first_name: OptionalName,
    middle_name: OptionalName,
    last_name: OptionalName,
    nick_name: OptionalName,
    home_page: OptionalString,
    email: OptionalString,
}

pub struct TitleInfo {
    genre: Vec<Genre>,
    author: Vec<Author>,
    book_title: OptionalName,
    annotation: OptionalAnnotation,
    // keywords
    // date
    // coverpage
    lang: OptionalString,
    src_lang: OptionalString,
    translator: Vec<Author>,
    sequence: Vec<Sequence>,
}

pub struct DocumentInfo {
    author: Vec<Author>,
    program_used: OptionalString, 
    src_url: OptionalString, 
    src_ocr: OptionalString, 
    id: OptionalString,
    version: OptionalString,
    // history
    // publisher
}

pub struct PublishInfo {
    book_name: OptionalName,
    publisher: OptionalName,
    sity: OptionalName,
    year: OptionalName,
    isbn: OptionalName,
    sequence: Vec<Sequence>,
}

pub struct Description {
    title_info: TitleInfo,
    src_title_info: OptionalTitleInfo,
    document_info: DocumentInfo,
    publish_info: Vec<PublishInfo>,
    //<custom-info> - 0..n (любое число, опционально);
    // <output> - 0..2 (опционально один или два) с версии 2.1. 
}

pub struct FictionBook {
    description: Description,
}