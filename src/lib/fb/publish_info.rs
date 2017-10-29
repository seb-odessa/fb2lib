/*********************************************************************************************
Элемент <publish-info>
Описание

Информация о бумажном (или другом) издании, на основании которого создан FB2.x документ. Не рекомендуется заполнять данными от произвольного издания если не известен источник, за исключением случая, когда по нему проводилась выверка и документ приведен к виду этого издания. Если же источник неизвестен, то лучше вообще опустить данный элемент.
Версия FB

2.0 и выше
Поддерживается

    Всеми (обеими) библиотеками, ориентироваными на FB2.
    Библиотечным софтом.

Атрибуты

Нет атрибутов.
Подчиненные элементы

Должен содержать в перечисленном порядке:

    <book-name> - 0..1 (один, опционально) - название;
    <publisher> - 0..1 (один, опционально) - издательство;
    <city> - 0..1 (один, опционально)- место издания;
    <year> - 0..1 (один, опционально) - год издания;
    <isbn> - 0..1 (один, опционально) - ISBN издания;
    <sequence> - 0..n (любое число, опционально) - серия (серии) изданий, в которую входит книга.

Подчинен

Может включаться в следующие элементы:

    <description> 0..1 (один, опционально)
*********************************************************************************************/
use xmltree::Element;
use fb::{BookName, Publisher, City, Year, Isbn, Sequence};
use fb::util::{HasFrom, from};

#[derive(Debug, PartialEq)]
pub struct PublishInfo {
    pub book_name: Option<BookName>,
    pub publisher: Option<Publisher>,
    pub city: Option<City>,
    pub year: Option<Year>,
    pub isbn: Option<Isbn>,
    pub sequence: Option<Sequence>,
}
impl HasFrom<PublishInfo> for PublishInfo {
    fn from(element: &Option<&Element>) -> Option<Self> {
        if let Some(ref node) = *element {
            Some(PublishInfo {
                book_name: from(node, "book-name"),
                publisher: from(node, "publisher"),
                city: from(node, "city"),
                year: from(node, "year"),
                isbn: from(node, "isbn"),
                sequence: from(node, "sequence")
            })
        } else {
            None
        }
    }
}
