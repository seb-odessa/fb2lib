# fb2lib
Tool for managing FB2 books.

## fb2lib archive
This part may be used for investigating content of the **archive.zip**
Will print usage if no subcommand was specified.
```
$ ./fb2lib archive data/arch.zip 
USAGE:
    fb2lib archive <archive.zip> [SUBCOMMAND]
```
### fb2lib archive ls
Print list of files in the **archive.zip**
```
$ ./fb2lib archive archive.zip ls
book1.fb2           562015    241143
book2.fb2           783762    235547
book4.fb2           160987     85787
book3.fb2          1118067    463975
book5.fb2          1505856    631985
...
```
### fb2lib archive check
Checks books in the **archive.zip**. Program take each book one by one from archive, extracts fb2 description, converts to UTF8 if needed and then parse result into the internal FictionBook representation.
```
$ ./fb2lib archive archive.zip check
Progress:  29%
The archive.zip file contained unsupported FB2 file 126390.fb2
Progress:  29%
The archive.zip file contained unsupported FB2 file 126570.fb2
Progress:  29%
The archive.zip file contained unsupported FB2 file 126620.fb2
Progress:  30%
The archive.zip file contained unsupported FB2 file 129813.fb2
Progress:  85%
The archive.zip file contained unsupported FB2 file 474428.fb2
Progress: 100%
Succeeded 828/833 (99%)
```
### fb2lib archive xml book1.fb2
This command extracts file description from zip file.
```
$ ./fb2lib archive archive.zip xml book2.fb2
<?xml version="1.0" encoding="utf-8"?>
<FictionBook xmlns="http://www.gribuser.ru/xml/fictionbook/2.0" xmlns:l="http://www.w3.org/1999/xlink">
 <description>
  <title-info>
   <genre>prose_military</genre>
   <author>
    <first-name>Сергей</first-name>
    <middle-name>Владимирович</middle-name>
    <last-name>Арсеньев</last-name>
    <home-page>http://samlib.ru/a/arsenxew_s_w/</home-page>
    <email>sv_6789@rambler.ru</email>
   </author>
   <book-title>Ленка-пенка</book-title>
   <date value="2012-09-06">2012-09-06</date>
   <coverpage>
    <image l:href="#cover.jpg"/></coverpage>
   <lang>ru</lang>
   <src-lang>ru</src-lang>
  </title-info>
  <document-info>
   <author>
    <nickname>eternal-return</nickname>
   </author>
   <program-used>doc2fb, Fiction Book Designer, FictionBook Editor 2.6.6</program-used>
   <date value="2012-09-06">06.09.2012</date>
   <src-url>http://samlib.ru/a/arsenxew_s_w/cbuffersvet37doc.shtml</src-url>
   <id>3B414024-275C-41E4-A5AD-6F719A37E4D8</id>
   <version>1.5</version>
   <history>
    <p>1.5 —UTF-8, многоточия, длинные тире, «», оформление цитат и стихов, обработка скриптами, удаление пробелов в начале строк (Namenlos)</p>
   </history>
  </document-info>
  <publish-info>
   <book-name>Ленка-пенка</book-name>
   <year>2012</year>
  </publish-info>
 </description>
</FictionBook>
```

### fb2lib archive fb2 book1.fb2
This command extracts file description from zip file like in prior operation and parses it into FictionBook structure
```
$ ./fb2lib archive archive.zip fb2 book2.fb2
FictionBook {
    description: Some(
        Description {
            title_info: Some(
                TitleInfo {
                    genres: [
                        Genre {
                            text: "prose_military"
                        }
                    ],
                    authors: [
                        Author {
                            first_name: Some(
                                FirstName {
                                    text: "Сергей"
                                }
                            ),
                            middle_name: Some(
                                MiddleName {
                                    text: "Владимирович"
                                }
                            ),
                            last_name: Some(
                                LastName {
                                    text: "Арсеньев"
                                }
                            ),
                            nickname: None
                        }
                    ],
                    translators: [],
                    sequences: [],
                    book_title: Some(
                        BookTitle {
                            text: "Ленка-пенка"
                        }
                    ),
                    date: Some(
                        Date {
                            value: "2012-09-06",
                            text: "2012-09-06"
                        }
                    ),
                    lang: Some(
                        Lang {
                            text: "ru"
                        }
                    ),
                    src_lang: Some(
                        SrcLang {
                            text: "ru"
                        }
                    )
                }
            ),
            document_info: Some(
                DocumentInfo {
                    authors: [
                        Author {
                            first_name: None,
                            middle_name: None,
                            last_name: None,
                            nickname: Some(
                                Nickname {
                                    text: "eternal-return"
                                }
                            )
                        }
                    ],
                    program_used: Some(
                        ProgramUsed {
                            text: "doc2fb, Fiction Book Designer, FictionBook Editor 2.6.6"
                        }
                    ),
                    date: Some(
                        Date {
                            value: "2012-09-06",
                            text: "06.09.2012"
                        }
                    ),
                    publishers: []
                }
            ),
            publish_info: Some(
                PublishInfo {
                    book_name: Some(
                        BookName {
                            text: "Ленка-пенка"
                        }
                    ),
                    publisher: None,
                    city: None,
                    year: Some(
                        Year {
                            text: "2012"
                        }
                    ),
                    isbn: None,
                    sequences: []
                }
            )
        }
    )
}
```

### fb2lib archive inf book1.fb2
This command extracts brief book description
```
$ ./fb2lib archive archive.zip inf book2.fb2
book2.fb2           : Ленка-пенка - Сергей Владимирович Арсеньев
```

### fb2lib archive zip book1.fb2
This command extracts phisical description of the book in the archive.zip
```
$ ./fb2lib archive archive.zip zip book2.fb2
book2.fb2  (Deflated) :   235547/  783762 crc32:    532651912, offset: 241210
```

## fb2lib database

## fb2lib torrent


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

## Work with database
### Initialize database structure
```
$ ./fb2lib database cleanup
db_cleanup(lib.rus.ec.db)
```
### Work with Metainfo (torrent) data
TODO

### Work with Languages
```
USAGE:
    fb2lib lang [lib.rus.ec.db] [SUBCOMMAND]

ARGS:
    <lib.rus.ec.db>    a sqlite database file name

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    ignore    Add language to ignore list
    load      Load languages from archive into DB
    show      Print unique sorted list of languages from archive

```
#### Show all unique languages in archive
SUBCOMMAND: show
```
$ ./fb2lib lang show ~/books/fb2-000065-572310_lost.zip 
lang_show(lib.rus.ec.db, /home/seb/books/fb2-000065-572310_lost.zip)
extract_langs(lib.rus.ec.db, /home/seb/books/fb2-000065-572310_lost.zip)
    'uk'
    'de'
    ''
    'pl'
    'ru'
    'bg'
    'ru-RU'
    'en'
```
#### Load unique languages from archive into database
SUBCOMMAND: load
```
$ ./fb2lib lang load ~/books/fb2-000065-572310_lost.zip 
lang_show(lib.rus.ec.db, /home/seb/books/fb2-000065-572310_lost.zip)
extract_langs(lib.rus.ec.db, /home/seb/books/fb2-000065-572310_lost.zip)
```
#### Add language to the ignore list
SUBCOMMAND: ignore
```
$ ./fb2lib lang ignore bg
lang_ignore(lib.rus.ec.db, bg)
```

### Links
- https://en.wikipedia.org/wiki/FictionBook
- http://fictionbook.org/index.php/FictionBook
- http://fictionbook.org/index.php?title=%D0%9E%D0%BF%D0%B8%D1%81%D0%B0%D0%BD%D0%B8%D0%B5_%D1%84%D0%BE%D1%80%D0%BC%D0%B0%D1%82%D0%B0_FB2_%D0%BE%D1%82_Sclex&oldid=2972

