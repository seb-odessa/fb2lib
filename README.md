# fb2lib
Tool for managing FB2 books.


## Работа с архивами книг

Анализ содержимого архивов с книгами в fb2 формате, например [fb2-632000-634999.zip](http://trec.to/viewtopic.php?t=34961);
Комманда: archive 

| Command | Description |
| ------- | ----------- |
| `$ fb2lib archive ls <archive.zip>`          | Вывести список файлов в архиве |
| `$ fb2lib archive check <archive.zip>`       | Проверить возможность распарсить книги в архиве |
| `$ fb2lib archive show xml <archive.zip> [book]`    | Извлечь описание книг(и) в XML формате [FB2](http://fictionbook.org/) |
| `$ fb2lib archive show fb2 <archive.zip> [book]`    | Извлечь описание в виде дампа внутренней структуры []FictionBook](https://github.com/seb-odessa/fb2parser) |
| `$ fb2lib archive show inf <archive.zip> [book]`    | Извлечь однострочное описание книги |
| `$ fb2lib archive show zip <archive.zip> [book]`    | Извлечь описание физического расположения файла в архиве |

The *book* argument may be replaced by regexps or wildcards, e.g:

`$ ./fb2lib archive archive.zip xml book*`

`$ ./fb2lib archive archive.zip xml b?ok.fb2`


## fb2lib database
This command allows to manage database.

| Command | Description |
| ------- | ----------- |
| `$ ./fb2lib database [db] reset`  | Reinitialize tables and views |


## fb2lib torrent
This command allows to manage torrent meta information.

| Command | Description |
| ------- | ----------- |
| `$ ./fb2lib torrent [db] load <archive.torrent>`  | Load metainfo from the torrent file  into DB |
| `$ ./fb2lib torrent [db] check <archive.zip>`  | Check archive.zip integrity with  compare of the loaded metainfo |


## fb2lib filter lang
This command allows to define language filters.

| Command | Description |
| ------- | ----------- |
| `$ fb2lib filter [db] lang display`          | Print list of disabled and enabled languages |
| `$ fb2lib filter [db] lang ls <archive.zip>`   | Print sorted list of languages from the specified archive.zip |
| `$ fb2lib filter [db] lang load <archive.zip>` | Load languages to the database from the specified archive.zip |
| `$ fb2lib filter [db] lang disable <lang>`     | Add specified language to disabled list. Wildcards (*/./?) alowed |
| `$ fb2lib filter [db] lang enable <lang>`      | Remove specified language from disabled list. Wildcards (*/./?) alowed |

## fb2lib filter genre
This command allows to define genre filters.

| Command | Description |
| ------- | ----------- |
| `$ fb2lib filter [db] genre display`          | Print list of disabled and enabled genres |
| `$ fb2lib filter [db] genre [unknown] <archive.zip>`   | Print list of unknown genres from the specified archive.zip |


fb2lib filter genre display

## Init flow
Prepare Database infrastructure
```
./fb2lib database reset torrent
./fb2lib database reset progress
./fb2lib database reset filter
./fb2lib database reset langs
./fb2lib database reset genre
./fb2lib database reset authors
./fb2lib database reset titles
./fb2lib database reset sequences
./fb2lib database reset books

./fb2lib torrent load ~/books/*.torrent
./fb2lib database load langs ~/books/*.zip
./fb2lib filter lang disable '*'
./fb2lib filter lang enable ''
./fb2lib filter lang enable 'r*'
./fb2lib filter lang display
./fb2lib filter genre display
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
