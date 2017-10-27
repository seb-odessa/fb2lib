/*********************************************************************************************
 Элемент <author>
Описание

Информация об авторе книги, если элемент используется в <title-info> или <src-title-info>; или об авторе документа, если в <document-info>.
Версия FB

2.0 и выше
Поддерживается

    Всеми (обеими) библиотеками, ориентироваными на FB2.
    Библиотечным софтом.
    "Читалками" обеспечивающими поддержку стандарта на уровне выше, чем "выкусывание тэгов".

Атрибуты

Нет атрибутов.
Подчиненные элементы

Содержит в перечисленном порядке следующие элементы:

    <first-name> - 0..1 (один, обязателен при отсутствии <nickname>, иначе опционально) - имя;
    <middle-name> - 0..1 (один, опционально) - отчество;
    <last-name> - 0..1 (один, обязателен при отсутствии <nickname>, иначе опционально) - фамилия;
    <nickname> - 0..1 (один, обязателен при отсутствии <first-name> и <last-name>, иначе опционально);
    <home-page> - 0..n (любое число, опционально);
    <email> - 0..n (любое число, опционально);
    <id> - 0..1 (один, опционально) с версии 2.2 - идентификатор автора, присваивается библиотекой.

Подчинен

Может содержаться в следующих элементах:

    <title-info> 1..n (любое число, один обязателен);
    <src-title-info> 1..n (любое число, один обязателен) с версии 2.1;
    <document-info> 1..n (любое число, один обязателен);
*********************************************************************************************/
use std::fmt;
use xmltree::Element;
use fb::{FirstName, MiddleName, LastName, Nickname};
use fb::util::load;

#[derive(Debug, PartialEq)]
pub struct Author {
    pub first_name: Option<FirstName>,
    pub middle_name: Option<MiddleName>,
    pub last_name: Option<LastName>,
    pub nickname: Option<Nickname>,
}

impl Author {
    #[allow(dead_code)]
    pub fn from(element: &Option<&Element>) -> Option<Self> {
        if let Some(node) = *element {
            Some(Author {
                first_name: load(node, "first-name"),
                middle_name: load(node, "middle-name"),
                last_name: load(node, "last-name"),
                nickname: load(node, "nickname"),
            })
        } else {
            None
        }
    }
}

impl fmt::Display for Author {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut first = false;
        if let Some(ref value) = self.first_name {
            write!(fmt, "{}", value)?;
            first = true;
        }

        let mut middle = false;
        if let Some(ref value) = self.middle_name {
            if first {
                write!(fmt, " ")?;
            }
            write!(fmt, "{}", value)?;
            middle = true;
        }
        let mut last = false;
        if let Some(ref value) = self.last_name {
            if middle || first {
                write!(fmt, " ")?;
            }
            write!(fmt, "{}", value)?;
            last = true;
        }
        if let Some(ref value) = self.nickname {
            if last || first || middle {
                write!(fmt, " ")?;
            }
            write!(fmt, "{}", value)?;
        }
        write!(fmt, "")
    }
}