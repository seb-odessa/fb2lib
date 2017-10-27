/*********************************************************************************************
Элемент <description>
Описание

Метаинформация о книге (автор(ы), название, выходные данные, ...) и файле (создатель(и), история, ссылки на источник, ...) Используется в основном библиотечным софтом, но может представлять интерес и рядовому читателю.

Рекомендуется серьезно отнесится к заполнению, поскольку неправильно (или неполно) заполненное описание приводит к некорректному позиционированию книги в библиотеке, что затрудняет ее поиск, приводит к появлению "двойников" авторов и просто не позволяет потенциальному читателю составить предварительное мнение о книге.
Версия FB

2.0 и выше
Поддерживается

    Всеми (обеими) библиотеками, ориентироваными на FB2.
    Библиотечным софтом.
    "Читалками" обеспечивающими поддержку стандарта на уровне выше, чем "выкусывание тэгов".

Атрибуты

Нет атрибутов.
Подчиненные элементы

Содержит в указанном порядке элементы:

    <title-info> - 1 (один, обязателен);
    <src-title-info> - 0..1 (один, опционально) с версии 2.1;
    <document-info> - 1 (один, обязателен);
    <publish-info> - 0..1 (один, опционально);
    <custom-info> - 0..n (любое число, опционально);
    <output> - 0..2 (опционально один или два) с версии 2.1.

Подчинен

Может содержаться в следующих элементах:

    <FictionBook> - 1 (один, обязателен)
*********************************************************************************************/
use std::fmt;
use xmltree::Element;
use fb::TitleInfo;
use fb::DocumentInfo;
use fb::PublishInfo;

#[derive(Debug, PartialEq)]
pub struct Description {
    pub title_info: Option<TitleInfo>,
    pub document_info: Option<DocumentInfo>,
    pub publish_info: Option<PublishInfo>,
}
impl Description {
    pub fn from(element: &Option<&Element>) -> Option<Self> {
        if let Some(ref node) = *element {
            Some(Description {
                title_info: TitleInfo::from(&node.get_child("title-info")),
                document_info: None,
                publish_info: None,
            })
        } else {
            None
        }
    }
}
impl fmt::Display for Description {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if let Some(ref title_info) = self.title_info {
            write!(fmt, "{}", title_info)
        } else {
            Ok(())
        }
    }
}
