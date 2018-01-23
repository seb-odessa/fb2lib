# fb2lib
Tool for managing FB2 books.

## fb2lib archive
This part may be used for investigating content of the **archive.zip**

| Command | Description |
| ------- | ----------- |
| `$ ./fb2lib archive archive.zip ls`          | Print list of files in the **archive.zip** |
| `$ ./fb2lib archive archive.zip check`       | Checks that books in the **archive.zip** are FB2 convertable. |
| `$ ./fb2lib archive archive.zip xml <book>`    | Prints book description in XML format |
| `$ ./fb2lib archive archive.zip fb2 <book>`    | Prints book description into FictionBook structure |
| `$ ./fb2lib archive archive.zip inf <book>`    | Prints book brief description |
| `$ ./fb2lib archive archive.zip zip <book>`    | Prints compression info and the offset of the book in the archive |

The *book* argument may be replaced by regexps or wildcards, e.g:

`$ ./fb2lib archive archive.zip xml book*`

`$ ./fb2lib archive archive.zip xml b?ok.fb2`


## fb2lib database
This command allows to manage database.

| Command | Description |
| ------- | ----------- |
| `$ ./fb2lib database reset`  | Reinitialize tables and views |


## fb2lib torrent
This command allows to manage torrent meta information.

| Command | Description |
| ------- | ----------- |
| `$ ./fb2lib torrent [db] load archive.torrent`  | Load metainfo from the torrent file  into DB |
| `$ ./fb2lib torrent [db] check archive.zip`  | Check archive.zip integrity with  compare of the loaded metainfo |


## fb2lib filter lang
This command allows to define language filters.

| Command | Description |
| ------- | ----------- |
| `$ fb2lib filter [db] lang display`          | Print list of disabled and enabled languages |
| `$ fb2lib filter [db] lang ls archive.zip`   | Print sorted list of languages from the specified archive.zip |
| `$ fb2lib filter [db] lang load archive.zip` | Load languages to the database from the specified archive.zip |
| `$ fb2lib filter [db] lang disable lang`     | Add specified language to disabled list. Wildcards (*/./?) alowed |
| `$ fb2lib filter [db] lang enable lang`      | Remove specified language from disabled list. Wildcards (*/./?) alowed |

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

