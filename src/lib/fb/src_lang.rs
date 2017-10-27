/*********************************************************************************************
 Элемент <src-SrcLang>
Описание

Язык оригинала (для переводов).
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

    <title-info> - 0..1 (один, опционально);
    <src-title-info>- 0..1 (один, опционально). 
*********************************************************************************************/
use std::fmt;
use xmltree::Element;
use fb::util::HasFrom;

#[derive(Debug, PartialEq)]
pub struct SrcLang {
    pub text: String,
}
impl HasFrom<SrcLang> for SrcLang {
    fn from(element: &Option<&Element>) -> Option<Self> {
        if let Some(ref node) = *element {
            Some(SrcLang { text: node.text.clone().unwrap_or_default() })
        } else {
            None
        }
    }
}
impl fmt::Display for SrcLang {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xmltree::Element;
    use fb::util::from;
    const TEST_DATA: &'static str = r##"<root><src-lang>ru</src-lang></root>"##;

    #[test]
    fn get_child() {
        let root = Element::parse(TEST_DATA.as_bytes()).unwrap();
        let element = root.get_child("src-lang").unwrap();
        assert_eq!("ru", &element.text.clone().unwrap_or_default());
        assert_ne!("en", &element.text.clone().unwrap_or_default());
    }

    #[test]
    fn from_trait_impl() {
        let root = Element::parse(TEST_DATA.as_bytes()).unwrap();
        assert_eq!(
            SrcLang { text: "ru".to_owned() },
            from(&root, "src-lang").unwrap()
        );
    }
}
