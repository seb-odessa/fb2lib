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
use fb::BookTitle;
use fb::util::{HasFrom, load_all, load};

#[derive(Debug, PartialEq)]
pub struct TitleInfo {
    pub genres: Vec<Genre>,
    pub authors: Vec<Author>,
    pub book_title: Option<BookTitle>,
}
impl HasFrom<TitleInfo> for TitleInfo {
    fn from(element: &Option<&Element>) -> Option<Self> {
        if let Some(ref node) = *element {
            Some(TitleInfo {
                genres: Vec::new(),
                authors: load_all(node, "author"),
                book_title: load(node, "book-title"),
            })
        } else {
            None
        }
    }
}
impl fmt::Display for TitleInfo {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if let Some(ref book_title) = self.book_title {
            write!(fmt, "{} - ", book_title)?;
        }
        for author in &self.authors {
            write!(fmt, "{}", author)?;
        }
        Ok(())
    }
}
