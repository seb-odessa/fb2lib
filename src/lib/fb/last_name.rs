/*********************************************************************************************
 Элемент <last-name>
Описание

Фамилия автора, переводчика или правообладателя.
Версия FB

2.0 и выше
Поддерживается

TODO: где поддерживается?
Атрибуты

    xml:lang (опциональный) - язык.

Подчиненные элементы

Нет дочерних элементов, содержит текст - собственно фамилия.
Подчинен

Может содержаться в следующих элементах:

    <author>;
    <translator>;
    <publisher> с версии 2.2.
*********************************************************************************************/
use std::fmt;
use xmltree::Element;

#[derive(Debug, PartialEq)]
pub struct LastName {
    pub text: String,
}
impl LastName {
    #[allow(dead_code)]
    pub fn from(e: &Element) -> Option<Self> {
        if e.name == "last-name" {
            return Some(LastName { text: e.text.clone().unwrap_or_default() })
        }
        None
    }
}
impl fmt::Display for LastName {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xmltree::Element;
    const TEST_DATA: &'static str = "<root><last-name>value</last-name></root>";

    #[test]
    fn from() {
        let root = Element::parse(TEST_DATA.as_bytes()).unwrap();
        let optional = LastName::from(&root.children[0]);
        assert!(optional.is_some());
        assert_eq!("value", optional.unwrap().text);
    }
}