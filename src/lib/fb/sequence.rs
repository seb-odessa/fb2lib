/*********************************************************************************************
 Элемент <sequence>
Описание

Серия изданий, в которую входит книга, и номер в серии.
Версия FB

2.0 и выше
Поддерживается

TODO: где поддерживается?
Атрибуты

    name (обязательный) - название серии;
    number (опциональный) - номер книги в серии;
    xml:lang (опциональный) - язык.

Подчиненные элементы

Нет дочерних элементов.
Подчинен

Может содержаться в следующих элементах:

    <title-info> - 0..n (любое число, опционально);
    <src-title-info> - 0..n (любое число, опционально);
    <publish-info> - 0..n (любое число, опционально).

Пример использования

<sequence name="Грегор Эйзенхорн" number="2"/>
 *********************************************************************************************/
use std::fmt;
use xmltree::Element;
use fb::util::HasFrom;


#[derive(Debug, PartialEq)]
pub struct Sequence {
    pub name: String,
    pub number: u32,
}
impl HasFrom<Sequence> for Sequence {
    fn from(element: &Option<&Element>) -> Option<Self> {
        if let Some(ref node) = *element {
            Some(Sequence {
                name: node.attributes.get("name").unwrap_or(&String::new()).clone(),
                number: node.attributes.get("number").unwrap_or(&String::new()).parse().unwrap_or_default(),
                })
        } else {
            None
        }
    }
}
impl fmt::Display for Sequence {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{} {}", self.name, self.number)
    }
}
