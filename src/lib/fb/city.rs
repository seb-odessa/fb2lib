/*********************************************************************************************
 Элемент <city>
Описание

Город, место издания оригинальной (бумажной) книги.
Версия FB

2.0 и выше
Поддерживается

TODO: где поддерживается?
Атрибуты

    xml:lang (опционально) - язык текста.

Подчиненные элементы

Нет подчиненных элементов, содержит текстовую строку ? собственно название города.
Подчинен

Может содержаться в следующих элементах:

    <publish-info> 0..1 (один, опционально).
*********************************************************************************************/
use std::fmt;
use xmltree::Element;
use fb::util::HasFrom;

#[derive(Debug, PartialEq)]
pub struct City {
    pub text: String,
}
impl HasFrom<City> for City {
    fn from(element: &Option<&Element>) -> Option<Self> {
        if let Some(ref node) = *element {
            Some(City { text: node.text.clone().unwrap_or_default() })
        } else {
            None
        }
    }
}
impl fmt::Display for City {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.text)
    }
}
