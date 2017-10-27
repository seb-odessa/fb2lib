/*********************************************************************************************
 Элемент <middle-name>
Описание

Отчество или второе имя автора, переводчика или правообладателя.
Версия FB

2.0 и выше
Поддерживается

TODO: где поддерживается?
Атрибуты

    xml:lang (опциональный) - язык.

Подчиненные элементы

Нет дочерних элементов, содержит текст - собственно отчество.
Подчинен

Может содержаться в следующих элементах:

    <author>;
    <translator>;
    <publisher> с версии 2.2.
*********************************************************************************************/
use std::fmt;
use fb::util::HasNew;


#[derive(Debug, PartialEq)]
pub struct MiddleName {
    pub text: String,
}
impl HasNew<MiddleName> for MiddleName {
    fn new(value: &str) -> MiddleName {
        MiddleName { text: String::from(value) }
    }
}
impl fmt::Display for MiddleName {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xmltree::Element;
    use fb::util::load;
    const TEST_DATA: &'static str = "<root><middle-name>value</middle-name></root>";

    #[test]
    fn from() {
        let root = Element::parse(TEST_DATA.as_bytes()).unwrap();
        assert_eq!(
            MiddleName::new("value"),
            load(&root, "middle-name").unwrap()
        );
    }
}