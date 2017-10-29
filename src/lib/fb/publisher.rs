/*********************************************************************************************
 Элемент <publisher>
Описание

Используется в двух местах документа для разных целей.

Изначально содержится в <publish-info>, и содержит название издателя оригинальной (бумажной) книги (текстовую строку).

С версии 2.2 может также содержаться в <document-info>, в этом случае содержит информацию о правообладателе документа (ФИО или псевдоним, а также присваиваемый библиотекой идентификатор). Как выразился Грибов, "Кому баппки отдавать, если таковые будут".
Версия FB

2.0 и выше, 2.2
Поддерживается

TODO: где поддерживается?
Атрибуты

Если содержится в <publish-info>, то

    xml:lang (опционально) - язык текста.

Если содержится в <document-info>, то нет атрибутов.
Подчиненные элементы

Если содержится в <publish-info>, то нет подчиненных элементов, содержит текстовую строку ? собственно название издателя книги.

Если содержится в <document-info>, то должен содержать элементы описанные в <author>.
Подчинен

Может содержаться в следующих элементах:

    <publish-info> (опционально);
    <document-info> (опционально).

Пример использования

<publish-info>
  <book-name>Долгин А.Б. Экономика символического обмена</book-name>
  <publisher>Инфра-М</publisher>
  <city>Москва</city>
  <year>2006</year>
  <isbn>5-16-002911-7</isbn>
</publish-info>
*********************************************************************************************/
use std::fmt;
use xmltree::Element;
use fb::util::{HasFrom, from};
use fb::{FirstName, MiddleName, LastName, Nickname};

#[derive(Debug, PartialEq)]
pub struct Publisher {
    pub text: String,
    pub first_name: Option<FirstName>,
    pub middle_name: Option<MiddleName>,
    pub last_name: Option<LastName>,
    pub nickname: Option<Nickname>,
}
impl HasFrom<Publisher> for Publisher {
    fn from(element: &Option<&Element>) -> Option<Self> {
        if let Some(ref node) = *element {
            Some(Publisher {
                text: node.text.clone().unwrap_or_default(),
                first_name: from(node, "first-name"),
                middle_name: from(node, "middle-name"),
                last_name: from(node, "last-name"),
                nickname: from(node, "nickname"),
                })
        } else {
            None
        }
    }
}
impl fmt::Display for Publisher {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.text)
    }
}
