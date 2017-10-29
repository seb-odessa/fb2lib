/*********************************************************************************************
 Элемент <book-name>
Описание

Название оригинальной (бумажной) книги.
Версия FB

2.0 и выше
Поддерживается

TODO: где поддерживается?
Атрибуты

    xml:lang (опционально) - язык названия.

Подчиненные элементы

Нет подчиненных элементов, содержит текстовую строку ? собственно название книги.
Подчинен

Может содержаться в следующих элементах:

    <publish-info> (опционально).

Пример использования

<publish-info>
  <book-name>Долгин А.Б. Экономика символического обмена</book-name>
  <publisher>Инфра-М</publisher>
  <city>Москва</city>
  <year>2006</year>
  <isbn>5-16-002911-7</isbn>
</publish-info>
*********************************************************************************************/
use std::fmt;
use xmltree::Element;
use fb::util::HasFrom;

#[derive(Debug, PartialEq)]
pub struct BookName {
    pub text: String,
}
impl HasFrom<BookName> for BookName {
    fn from(element: &Option<&Element>) -> Option<Self> {
        if let Some(ref node) = *element {
            Some(BookName { text: node.text.clone().unwrap_or_default() })
        } else {
            None
        }
    }
}
impl fmt::Display for BookName {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.text)
    }
}
