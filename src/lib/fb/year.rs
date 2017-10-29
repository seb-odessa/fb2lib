/*********************************************************************************************
  Элемент <year>
Описание

Год издания оригинальной (бумажной) книги.
Версия FB

2.0 и выше
Поддерживается

TODO: где поддерживается?
Атрибуты

Нет атрибутов.
Подчиненные элементы

Нет подчиненных элементов.

Содержит текстовую строку ? собственно год издания книги.
Подчинен

Может содержаться в следующих элементах:

    <publish-info> (опционально).
*********************************************************************************************/
use std::fmt;
use xmltree::Element;
use fb::util::HasFrom;

#[derive(Debug, PartialEq)]
pub struct Year {
    pub text: String,
}
impl HasFrom<Year> for Year {
    fn from(element: &Option<&Element>) -> Option<Self> {
        if let Some(ref node) = *element {
            Some(Year { text: node.text.clone().unwrap_or_default() })
        } else {
            None
        }
    }
}
impl fmt::Display for Year {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.text)
    }
}
