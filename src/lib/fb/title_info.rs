/*********************************************************************************************
Элемент <title-info>
Описание

Описание информации о произведении (с учетом перевода, но без учета издания).
Версия FB

2.0 и выше
Поддерживается

    Всеми (обеими) библиотеками, ориентироваными на FB2.
    Библиотечным софтом.
    "Читалками" обеспечивающими поддержку стандарта на уровне выше, чем "выкусывание тэгов".

Атрибуты

Нет атрибутов.
Подчиненные элементы

Должен содержать в перечисленном порядке:

    <genre> - 1..n (любое число, один обязaтелен);
    <author> - 1..n (любое число, один обязaтелен);
    <book-title> - 1 (один, обязателен);
    <annotation> - 0..1 (один, опционально);
    <keywords> - 0..1 (один, опционально);
    <date> - 0..1 (один, опционально);
    <coverpage> - 0..1 (один, опционально);
    <lang> - 1 (один, обязателен);
    <src-lang> - 0..1 (один, опционально);
    <translator> - 0..n (любое число, опционально);
    <sequence> - 0..n (любое число, опционально).

Подчинен

Может содержаться в следующих элементах:

    <description> - 1 (один, обязателен)
**********************************************************************************************/
use std::fmt;
use xmltree::Element;
use fb::Genre;
use fb::Author;
use fb::Booktitle;
use fb::util::load;

#[derive(Debug, PartialEq)]
pub struct TitleInfo {
    pub genres: Option<Vec<Genre>>,
    pub authors: Option<Vec<Author>>,
    pub book_title: Option<Booktitle>,
}
impl TitleInfo {
    pub fn from(element: &Option<&Element>) -> Option<Self> {
        if let Some(ref node) = *element {
            Some(TitleInfo {
                genres: None,
                authors: TitleInfo::authors(&node),
                book_title: load(node, "book-title"),
            })
        } else {
            None
        }
    }
    fn authors(node: &Element) -> Option<Vec<Author>> {
        let mut authors = Vec::new();
        authors.push(Author::from(&node.get_child("author")).unwrap());
        // if &e.name == "author" {
        //     let mut author = Author::new();
        //     for child in &e.children {
        //         match child.name.as_str() {
        //             "first-name" => author.first_name = FirstName::from(&child),
        //             "middle-name" => author.middle_name = MiddleName::from(&child),
        //             "last-name" => author.last_name = LastName::from(&child),
        //             "nickname" => author.nickname = Nickname::from(&child),
        //             _ => {}
        //         }
        //     }
        //     return Some(author);
        // }

        Some(authors)
    }
}
impl fmt::Display for TitleInfo {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if let Some(ref book_title) = self.book_title {
            write!(fmt, "{}", book_title)?;
        }
        if let Some(ref authors) = self.authors {
            for author in authors {
                write!(fmt, " - {}", author)?;
            }
        }
        Ok(())
    }
}
