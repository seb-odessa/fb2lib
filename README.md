# fb2lib
Tool for managing FB2 books in ZIP archives.
E.g. fb2-612000-614999.zip - official archive of the lib.rus.ec online library.

### Usage
```
$ fb2lib
fb2lib v0.4.1
seb <seb@ukr.net>
FictionBook Library Archive Manager

USAGE:
    fb2lib [fb_archive.zip] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <fb_archive.zip>    Zip archive with books in FB2 format

SUBCOMMANDS:
    check    Try parse all archive and print only failured books
    help     Prints this message or the help of the given subcommand(s)
    ls       List archive contents
    parse    Try parse xml into fb2 and print it
    show     Request to extract and print some kind of content
```

### List contet
```
$ fb2lib data/arch.zip ls

book1.fb2            241143       562015
book2.fb2            235547       783762
book4.fb2             85787       160987
book3.fb2            463975      1118067
book5.fb2            631985      1505856
```

### Show description part for concrete FB2 book
```
$ fb2lib data/arch.zip show xml book1.fb2

<?xml version="1.0" encoding="Windows-1251"?>
<FictionBook xmlns="http://www.gribuser.ru/xml/fictionbook/2.0" xmlns:l="http://www.w3.org/1999/xlink">
 <description>
  <title-info>
   <genre>sf</genre>
   <genre>sf_history</genre>
   <author>
    <first-name>Константин</first-name>
    <middle-name>Георгиевич</middle-name>
    <last-name>Калбанов</last-name>
    <home-page>http://samlib.ru/k/kalbazow_k_g/</home-page>
    <email>mahoni928@yandex.ru</email>
   </author>
   <book-title>Робинзоны</book-title>
   <annotation>
    <p>Каменный век и немного прогрессорства.</p>
   </annotation>
   <date></date>
   <coverpage>
    <image l:href="#ac6591c8dd81e13edc3209e7273fd309.jpg"/></coverpage>
   <lang>ru</lang>
   <sequence name="Робинзоны" number="1"/>
  </title-info>
  <document-info>
   <author>
    <nickname>54321876875</nickname>
   </author>
   <program-used>FictionBook Editor Release 2.6</program-used>
   <date value="2012-09-06">06-09-2012</date>
   <src-url>http://samlib.ru/k/kalbazow_k_g/kot12.shtml</src-url>
   <src-ocr>СИ</src-ocr>
   <id>A555657E-48F3-4B79-8A46-0F9D18908A3C</id>
   <version>1.2</version>
   <history>
    <p>v. 1.2 — создание FB2 (54321876875)</p>
   </history>
  </document-info>
 </description>
</FictionBook>
```

### Show FictionBook structure for concrete FB2 book
```
$ fb2lib data/arch.zip show xml book1.fb2

FictionBook {
    description: Description {
        title_info: TitleInfo {
            author: [
                Author {
                    first_name: "Кристофер",
                    middle_name: "",
                    last_name: "Паолини",
                    nick_name: "",
                    home_page: "",
                    email: ""
                }
            ],
            book_title: "Эрагон. Возвращение",
            lang: "ru",
            src_lang: "en",
            translator: [
                Author {
                    first_name: "Ирина",
                    middle_name: "",
                    last_name: "Тогоева",
                    nick_name: "",
                    home_page: "",
                    email: ""
                }
            ],
            sequence: [
                Sequence {
                    name: "Эрагон",
                    number: "2",
                    lang: ""
                }
            ]
        }
    }
}
```

### Show Book Title and Author(s)
```
$ fb2lib data/arch.zip show info book5.fb2
'Эрагон. Возвращение' - Кристофер  Паолини 

```

### Performance tests
```
test fb::bench::deserialize_author       ... bench:      10,836 ns/iter (+/- 865)
test fb::bench::deserialize_description  ... bench:      34,267 ns/iter (+/- 5,313)
test fb::bench::deserialize_fiction_book ... bench:      80,209 ns/iter (+/- 7,791)
test fb::bench::deserialize_title_info   ... bench:      29,837 ns/iter (+/- 2,241)
test tools::bench::extract_xml_prolog    ... bench:          64 ns/iter (+/- 5)
test tools::bench::find_positions        ... bench:           2 ns/iter (+/- 0)
test tools::bench::get_encoding          ... bench:         200 ns/iter (+/- 19)
test tools::bench::into_utf8             ... bench:       1,136 ns/iter (+/- 100)

```


### Links
- https://en.wikipedia.org/wiki/FictionBook
- http://fictionbook.org/index.php/FictionBook
- http://fictionbook.org/index.php?title=%D0%9E%D0%BF%D0%B8%D1%81%D0%B0%D0%BD%D0%B8%D0%B5_%D1%84%D0%BE%D1%80%D0%BC%D0%B0%D1%82%D0%B0_FB2_%D0%BE%D1%82_Sclex&oldid=2972

