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
use xmltree::Element;
use fb::util::HasFrom;

#[derive(Debug, PartialEq)]
pub struct Nickname {
    pub text: String,
}
impl HasFrom<Nickname> for Nickname {
    fn from(element: &Option<&Element>) -> Option<Self> {
        if let Some(ref node) = *element {
            Some(Nickname { text: node.text.clone().unwrap_or_default() })
        } else {
            None
        }
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
    use fb::util::from;
    const TEST_DATA: &'static str = "<root><nickname>value</nickname></root>";

    #[test]
    fn from_trait_impl() {
        let root = Element::parse(TEST_DATA.as_bytes()).unwrap();
        assert_eq!(
            Nickname { text: "value".to_owned() },
            from(&root, "nickname").unwrap()
        );
    }
}
