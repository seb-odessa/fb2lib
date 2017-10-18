#[cfg(test)]
pub mod bench {
    pub const XML: &str = "
    <?xml version=\"1.0\" encoding=\"utf-8\"?>
    <FictionBook xmlns=\"http://www.gribuser.ru/xml/fictionbook/2.0\" xmlns:l=\"http://www.w3.org/1999/xlink\">
    <description>
        <title-info>
            <genre>sf_space</genre>
            <author>
                <first-name>Дж. Майкл</first-name>
                <last-name>Стражинский</last-name>
            </author>
            <book-title>Тень его мыслей</book-title>
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

    pub const DESCRIPTION: &str = "
    <description>
        <title-info>
            <genre>sf_space</genre>
            <author>
                <first-name>Дж. Майкл</first-name>
                <last-name>Стражинский</last-name>
            </author>
            <book-title>Тень его мыслей</book-title>
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
    </description>";

    pub const TITLE_INFO: &str = "
    <title-info>
        <genre>sf_space</genre>
        <author>
            <first-name>Дж. Майкл</first-name>
            <last-name>Стражинский</last-name>
        </author>
        <book-title>Тень его мыслей</book-title>
        <keywords>Вавилон 5</keywords>
        <date>1999</date>
        <coverpage>
            <image l:href=\"Any2FbImgLoader0\"/>
        </coverpage>
        <lang>ru</lang>
    </title-info>";

    pub const AUTHOR: &str = "
        <author>
            <first-name>Дж. Майкл</first-name>
            <last-name>Стражинский</last-name>
        </author>";

}
