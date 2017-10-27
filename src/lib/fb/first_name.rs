/*********************************************************************************************
 Элемент <first-name>
Описание

Имя автора, переводчика или правообладателя.
Версия FB

2.0 и выше
Поддерживается

TODO: где поддерживается?
Атрибуты

    xml:lang (опциональный) - язык.

Подчиненные элементы

Нет дочерних элементов, содержит текст - собственно имя.
Подчинен

Может содержаться в следующих элементах:

    <author>;
    <translator>;
    <publisher> с версии 2.2.
*********************************************************************************************/
use std::fmt;
use fb::util::HasNew;


#[derive(Debug, PartialEq)]
pub struct FirstName {
    pub text: String,
}
impl HasNew<FirstName> for FirstName {
    fn new(value: &str) -> FirstName {
        FirstName { text: String::from(value) }
    }
}
impl fmt::Display for FirstName {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xmltree::Element;
    const TEST_DATA: &'static str = "<root><first-name>value</first-name></root>";

    #[test]
    fn from() {
        let root = Element::parse(TEST_DATA.as_bytes()).unwrap();
        let optional = FirstName::from(&root.children[0]);
        assert!(optional.is_some());
        assert_eq!("value", optional.unwrap().text);
    }
}
