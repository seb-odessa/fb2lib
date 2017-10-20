/*********************************************************************************************
Элемент <FictionBook>
Описание

Корневой элемент документа.
Версия FB

2.0 и выше
Поддерживается

По своей природе поддерживается любой программой, претендующей на минимальную поддержку FB.
Атрибуты

Атрибутов нет.

Поскольку является корневым элементом, то здесь задаются пространства имен (что выглядит похоже на аттрибуты).
Подчиненные элементы

Должен содержать в перечисленном порядке:

    <stylesheet> - 0..n (любое число, опционально);
    <description> - 1 (один, обязателен);
    <body> - 1..n (любое число, один обязaтелен);
    <binary> - 0..n (любое число, опционально).

Подчинен

Поскольку является корневым элементом, то никому не подчинен.
*********************************************************************************************/

use std::fmt;
use xmltree::Element;
use result::{Fb2Error, Fb2Result};
use fb::util::query;
use fb::util::query_path;
use fb::Description;


#[derive(Debug, PartialEq)]
pub struct FictionBook {
    pub root: Box<Element>,
    pub description: Option<Description>,
}
impl FictionBook {
    pub fn new(xml: &[u8]) -> Fb2Result<Self> {
        match Element::parse(xml) {
            Ok(fb) => Ok(FictionBook { root: Box::new(fb), description: None }),
            Err(e) => Err(Fb2Error::Custom(format!("{}", e))),
        }
    }

    pub fn get_book_title(&self) -> String {
        query(&self.root, "description/title-info/book-title")
            .map_or(None, |ref e| e.text.clone())
            .unwrap_or_default()
    }

    pub fn get_book_authors(&self) -> Vec<String> {
        let mut authors = Vec::new();
        if let Some(ref titile_info) = query(&self.root,"description/title-info") {
            for child in &titile_info.children {
                let fitst_name = query_path(&Some(&child), &["first-name"])
                    .map_or(None, |ref e| e.text.clone())
                    .unwrap_or_default();

                let name = format!("!!{}!!", fitst_name);
                authors.push(name);
            }
        }
        return authors;
    }
}

impl fmt::Display for FictionBook {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.get_book_title())?;
        write!(fmt, "{}", self.get_book_authors()[0])
    }
}


#[cfg(test)]
mod tests {
    use data::bench::XML;
    use fb::FictionBook;

    #[test]
    fn get_book_title() {
        let fb = FictionBook::new(XML.as_bytes()).unwrap();
        assert_eq!("Тень его мыслей", &fb.get_book_title());
    }
}

