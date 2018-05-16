
rm ../lib.rus.ec.db
../fb2lib database reset torrent
../fb2lib database reset progress
../fb2lib database reset filter
../fb2lib database reset langs
../fb2lib database reset genre
../fb2lib database reset authors
../fb2lib database reset titles
../fb2lib database reset sequences
../fb2lib database reset desc

../fb2lib torrent load ~/books/*.torrent
