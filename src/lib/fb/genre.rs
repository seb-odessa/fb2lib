/*********************************************************************************************
 Элемент <genre>
Описание

Описывает жанровую принадлежность книги. Используется для помещения книги в рубрикатор библиотеки, по этой причине список возможных жанров жестко задается. Допускается указание нескольких жанров.
Версия FB

2.0 и выше
Поддерживается

    Всеми (обеими) библиотеками, ориентироваными на FB2.
    Библиотечным софтом.
    Многими "Читалками"

Атрибуты

    match (опциональный, значение по умолчанию "100") ? число от "1" до "100", задающее субъективное процентное соответствие данному жанру.

Подчиненные элементы

Нет дочерних элементов.

Содержит текст - обозначение жанра из списка жанров.
Подчинен

Может содержаться в следующих элементах:

    <title-info>
    <src-title-info> с версии 2.1
 *********************************************************************************************/

#[derive(Debug, PartialEq)]
pub struct Genre {

 }