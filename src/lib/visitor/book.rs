use sal;
use archive;
use algorithm;
use result::Fb2Result;
use visitor::acess::AccessGuard;
use fb2parser::FictionBook;
use types::FileDesc;

use zip::ZipFile;

use std::collections::HashMap;
use std::convert::From;


#[derive(Debug, PartialEq, Eq, Hash)]
struct People {
    first_name: String,
    middle_name: String,
    last_name: String,
    nickname: String,
}
impl From<(String, String,String, String)> for People {
    fn from(src: (String, String,String, String)) -> Self {
        People {
            first_name: src.0,
            middle_name: src.1,
            last_name: src.2,
            nickname: src.3
        }
    }
}



pub struct Book {
    allowed: usize,
    counter: usize,
    archive: i64,
    access: AccessGuard,
    people: HashMap<People, i64>,
    genres: HashMap<String, i64>,
    languages: HashMap<String, i64>,
    titles: HashMap<String, i64>,
    sequences: HashMap<String, i64>,
    connection: sal::Connection,
}
impl <'a> Book {
    pub fn new(conn: sal::Connection, access: AccessGuard) -> Fb2Result<Self> {
        let book: Book = Book {
            allowed: 0,
            counter: 0,
            archive: 0,
            access: access,
            people: sal::load_people(&conn)?.into_iter().map(|(name, id)| (People::from(name), id)).collect(),
            genres: sal::load_id_by_name(&conn, sal::LOAD_ID_BY_GENRE)?,
            languages: sal::load_id_by_name(&conn, sal::LOAD_ID_BY_LANG)?,
            titles: sal::load_id_by_name(&conn, sal::LOAD_ID_BY_TITLE)?,
            sequences: sal::load_id_by_name(&conn, sal::LOAD_ID_BY_SEQUENCE)?,
            connection: conn,
        };
        Ok(book)
    }
    pub fn select_archive(&mut self, archive: &str) -> Fb2Result<()> {
        self.archive = sal::get_archive_id_by_name(&self.connection, archive)?;
        Ok(())
    }
}
impl <'a> algorithm::Visitor<'a> for Book{
    type Type = ZipFile<'a> ;
    fn visit(&mut self, zip: &mut Self::Type) {
        self.counter += 1;
        println!("Book::visit() <- {}", zip.name());
        match archive::load_fb2(zip) {
            Ok(book) => if self.archive != 0 && self.access.is_allowed(&book) {
                self.allowed += 1;
                let book: FileDesc = FileDesc::from(zip);

                match sal::register_book(&mut self.connection, self.archive, &book){
                    Ok(()) => {},
                    Err(e) => println!("{}", e)
                }

            },
            Err(err) => println!("{}", err),
        }

    }
    fn get_count(&self) -> usize {
        self.counter
    }
    fn report(&self) {
        let t = sal::load_hash_to_id(&self.connection, sal::LOAD_ID_BY_TITLE).unwrap().len();
        println!("Handled {} files in archive, and {} allowed.", self.counter, self.allowed);
        println!("Known people count {}.", self.people.len());
        println!("Known genres count {}.", self.genres.len());
        println!("Known languages count {}.", self.languages.len());
        println!("Known titles count {} ({}).", self.titles.len(), t);
        println!("Known sequences count {}.", self.sequences.len());
    }

}


#[cfg(test)]
mod tests {
    use super::People;

    #[test]
    fn people_from_tuple() {
        let src = (String::from("First"), String::from("Middle"), String::from("Last"), String::from("Nickname"));
        let people = People::from(src.clone());
        assert_eq!(people.first_name, src.0);
        assert_eq!(people.middle_name, src.1);
        assert_eq!(people.last_name, src.2);
        assert_eq!(people.nickname, src.3);

    }

}