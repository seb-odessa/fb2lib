/*********************************************************************************************
 Элемент <isbn>
Описание

ISBN оригинальной (бумажной) книги.
Версия FB

2.0 и выше
Поддерживается

TODO: где поддерживается?
Атрибуты

    xml:lang (опционально) - язык.

Подчиненные элементы

Нет подчиненных элементов, содержит текстовую строку ? собственно ISBN книги.
Подчинен

Может содержаться в следующих элементах:

    <publish-info> (опционально).
*********************************************************************************************/
use std::fmt;
use xmltree::Element;
use fb::util::HasFrom;

#[derive(Debug, PartialEq)]
pub struct Isbn {
    pub text: String,
}
impl HasFrom<Isbn> for Isbn {
    fn from(element: &Option<&Element>) -> Option<Self> {
        if let Some(ref node) = *element {
            Some(Isbn { text: node.text.clone().unwrap_or_default() })
        } else {
            None
        }
    }
}
impl fmt::Display for Isbn {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.text)
    }
}
