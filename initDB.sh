rm lib.rus.ec.db
./fb2lib database reset torrent
./fb2lib database reset progress
./fb2lib database reset filter
./fb2lib database reset langs
./fb2lib database reset genre
./fb2lib database reset authors
./fb2lib database reset titles
./fb2lib database reset sequences

./fb2lib torrent load ~/books/*.torrent
./fb2lib database load langs ~/books/*.zip
./fb2lib filter lang disable '*'
./fb2lib filter lang enable ''
./fb2lib filter lang enable 'r*'
./fb2lib filter lang display

#./fb2lib filter genre display
