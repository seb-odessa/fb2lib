/*********************************************************************************************
Элемент <document-info>
Описание

Описание информации о конкретном FB2.x документе (создатель(и), история и т.д.).
Версия FB

2.0 и выше
Поддерживается

    Всеми (обеими) библиотеками, ориентироваными на FB2.
    Библиотечным софтом.

Атрибуты

Нет атрибутов.
Подчиненные элементы

Содержит в перечисленном порядке элементы:

    <author> - 1..n (любое число, один обязaтелен);
    <program-used> - 0..1 (один, опционально);
    <date> - 1 (один, обязателен);
    <src-url> - 0..n (любое число, опционально);
    <src-ocr> - 0..1 (один, опционально);
    <id> - 1 (один, обязателен);
    <version> - 1 (один, обязателен);
    <history> - 0..1 (один, опционально);
    <publisher> - 0..n (любое число, опционально) с версии 2.2.

Подчинен

Может содержаться в следующих элементах:

    <description> - 1 (один, обязателен)
*********************************************************************************************/
use xmltree::Element;
use fb::{Author};
use fb::{ProgramUsed, Date};
use fb::util::{HasFrom, all_from, from};

#[derive(Debug, PartialEq)]
pub struct DocumentInfo{
    pub authors: Vec<Author>,
    pub program_used: Option<ProgramUsed>,
    pub date: Option<Date>
}
impl HasFrom<DocumentInfo> for DocumentInfo {
    fn from(element: &Option<&Element>) -> Option<Self> {
        if let Some(ref node) = *element {
            Some(DocumentInfo {
                authors: all_from(node, "author"),
                program_used: from(node, "program-used"),
                date: from(node, "date"),
            })
        } else {
            None
        }
    }
}
