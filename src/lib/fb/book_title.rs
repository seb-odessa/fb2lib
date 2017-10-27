/*********************************************************************************************
 Элемент <book-title>
Описание

Название произведения. Может как совпадать с названием книги (<book-name>), так и отличаться (например, когда под одной обложкой находится несколько произведений).
Версия FB

2.0 и выше
Поддерживается

    Всеми (обеими) библиотеками, ориентироваными на FB2.
    Библиотечным софтом.
     ?Читалками? обеспечивающими поддержку стандарта на уровне выше, чем ?выкусывание тэгов?.

Атрибуты

    xml:lang (опционально) ? язык контента.

Подчиненные элементы

Нет подчиненных элементов, содержит текстовую строку ? собственно название произведения.
Подчинен

Может содержаться в следующих элементах:

    <title-info> 1 (один, обязателен);
    <src-title-info> 1 (один, обязателен)
*********************************************************************************************/
use std::fmt;
use xmltree::Element;
use fb::util::HasFrom;

#[derive(Debug, PartialEq)]
pub struct BookTitle {
    pub text: String,
}
impl HasFrom<BookTitle> for BookTitle {
    fn from(element: &Option<&Element>) -> Option<Self> {
        if let Some(ref node) = *element {
            Some(BookTitle { text: node.text.clone().unwrap_or_default() })
        } else {
            None
        }
    }
}
impl fmt::Display for BookTitle {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xmltree::Element;
    use fb::util::from;
    const TEST_DATA: &'static str = "<root><book-title>value</book-title></root>";

    #[test]
    fn from_trait_impl() {
        let root = Element::parse(TEST_DATA.as_bytes()).unwrap();
        assert_eq!(
            BookTitle { text: "value".to_owned() },
            from(&root, "book-title").unwrap()
        );
    }
}