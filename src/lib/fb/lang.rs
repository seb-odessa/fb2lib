/*********************************************************************************************
 Элемент <lang>
Описание

Язык книги (произведения), если содержится в <title-info>; либо язык оригинала (для переводов), если в <src-title-info>.
Версия FB

2.0 и выше
Поддерживается

TODO: где поддерживается?
Атрибуты

Нет атрибутов.
Подчиненные элементы

Нет дочерних элементов, содержит текст - двухбуквенный код языка.
Подчинен

Может содержаться в следующих элементах:

    <title-info> 1 (один, обязателен);
    <src-title-info> 1 (один, обязателен) с версии 2.1.
*********************************************************************************************/
use std::fmt;
use fb::util::HasNew;


#[derive(Debug, PartialEq)]
pub struct Lang {
    pub text: String,
}
impl HasNew<Lang> for Lang {
    fn new(value: &str) -> Lang {
        Lang { text: String::from(value) }
    }
}
impl fmt::Display for Lang {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xmltree::Element;
    use fb::util::load;
    const TEST_DATA: &'static str = r##"<root><lang>ru</lang></root>"##;

    #[test]
    fn get_child() {
        let root = Element::parse(TEST_DATA.as_bytes()).unwrap();
        let element = root.get_child("lang").unwrap();

        assert_eq!("ru", &element.text.clone().unwrap_or_default());
        assert_ne!("en", &element.text.clone().unwrap_or_default());
    }

    #[test]
    fn from() {
        let root = Element::parse(TEST_DATA.as_bytes()).unwrap();
        assert_eq!(Lang::new("ru"), load(&root, "lang").unwrap());
    }
}
