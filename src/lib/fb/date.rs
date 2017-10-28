/*********************************************************************************************
 Элемент <date>
Описание

Дата
Версия FB

2.0 и выше
Поддерживается

TODO: где поддерживается?
Атрибуты

    xml:lang (опционально) - язык;
    name="value (опционально)

Подчиненные элементы

Нет подчиненных элементов, содержит текстовое представление даты.
Подчинен

Может содержаться в следующих элементах:

    <title-info> 0..1 (один, опционально);
    <src-title-info> 0..1 (один, опционально) с версии 2.1;
    <document-info> 0..1 (один, опционально);
    <poem> 0..1 (один, опционально).

Пример использования

<date value="2002-10-19">19.10.2002</date>
*********************************************************************************************/
use std::fmt;
use xmltree::Element;
use fb::util::HasFrom;


#[derive(Debug, PartialEq)]
pub struct Date {
    pub value: String,
    pub text: String,
}
impl HasFrom<Date> for Date {
    fn from(element: &Option<&Element>) -> Option<Self> {
        if let Some(ref node) = *element {
            Some(Date {
                value: node.attributes.get("value").unwrap_or(&String::new()).clone(),
                text: node.text.clone().unwrap_or_default(),
                })
        } else {
            None
        }
    }
}
impl fmt::Display for Date {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.value)
    }
}
