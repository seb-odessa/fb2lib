#[cfg(test)]
pub mod bench {
    pub const XML: &str =
    "<?xml version=\"1.0\" encoding=\"utf-8\"?>
    <FictionBook
    xmlns=\"http://www.gribuser.ru/xml/fictionbook/2.0\" xmlns:l=\"http://www.w3.org/1999/xlink\">
    <description>
        <title-info>
            <genre>sf_space</genre>
            <author>
                <first-name>Дж. Майкл</first-name>
                <last-name>Стражинский</last-name>
            </author>
            <book-title>Тень его мыслей</book-title>
            <annotation>
                <p>Данный перевод был впервые опубликован на сайте http://beyond.babylonfive.ru/</p>
                <empty-line/>
                <p>Опyбликовано в 597 номере (2 выпуск 71 года издания) журнала Amazing Stories, лето 1999</p>
            </annotation>
            <keywords>Вавилон 5</keywords>
            <date>1999</date>
            <coverpage>
                <image l:href=\"Any2FbImgLoader0\"/>
            </coverpage>
            <lang>ru</lang>
        </title-info>
        <document-info>
            <author>
                <first-name></first-name>
                <last-name></last-name>
            </author>
            <program-used></program-used>
            <date value=\"2008-03-06\">2008-03-06</date>
            <id></id>
            <version></version>
        </document-info>
    </description>
    </FictionBook>";


}
