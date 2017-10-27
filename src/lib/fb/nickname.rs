/*********************************************************************************************
 Элемент <nickname>
Описание

Ник (псевдоним т.п. имена, не укладывающиеся в ФИО) автора, переводчика или правообладателя.
Версия FB

2.0 и выше
Поддерживается

TODO: где поддерживается?
Атрибуты

    xml:lang (опциональный) - язык.

Подчиненные элементы

Нет подчиненных элементов, содержит текст.
Подчинен

Может содержаться в следующих элементах:

    <author>;
    <translator>;
    <publisher> с версии 2.2.
*********************************************************************************************/
use std::fmt;
use fb::util::HasNew;

#[derive(Debug, PartialEq)]
pub struct Nickname {
    pub text: String,
}
impl HasNew<Nickname> for Nickname {
    fn new(value: &str) -> Nickname {
        Nickname { text: String::from(value) }
    }
}
impl fmt::Display for Nickname {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xmltree::Element;
    const TEST_DATA: &'static str = "<root><nickname>value</nickname></root>";

    #[test]
    fn from() {
        let root = Element::parse(TEST_DATA.as_bytes()).unwrap();
        let optional = Nickname::from(&root.children[0]);
        assert!(optional.is_some());
        assert_eq!("value", optional.unwrap().text);
    }
}
