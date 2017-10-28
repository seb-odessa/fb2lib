/*********************************************************************************************
 Элемент <program-used>
Описание

Перечисляются программы, которые использовались при подготовке документа.
Версия FB

2.0 и выше
Поддерживается

TODO: где поддерживается?
Атрибуты

    xml:lang (опционально) - язык.

Подчиненные элементы

Нет дочерних элементов, содержит текст.
Подчинен

Может сожержаться в следующих элементах:

    <document-info> (опционально).

Пример использования

<program-used>Dn/2, Opera 8.50, Bred3</program-used>
 *********************************************************************************************/
use std::fmt;
use xmltree::Element;
use fb::util::HasFrom;


#[derive(Debug, PartialEq)]
pub struct ProgramUsed {
    pub text: String,
}
impl HasFrom<ProgramUsed> for ProgramUsed {
    fn from(element: &Option<&Element>) -> Option<Self> {
        if let Some(ref node) = *element {
            Some(ProgramUsed { text: node.text.clone().unwrap_or_default() })
        } else {
            None
        }
    }
}
impl fmt::Display for ProgramUsed {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.text)
    }
}
