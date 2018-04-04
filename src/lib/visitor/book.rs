use sal;
use archive;
use algorithm;
use result::Fb2Result;
use visitor::acess::AccessGuard;
use fb2parser::FictionBook;
use zip::ZipFile;

use std::collections::HashMap;
use std::convert::From;


#[derive(Debug, PartialEq, Eq, Hash)]
pub struct People {
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
    connection: sal::Connection,
    access: AccessGuard,
    people: HashMap<People, i64>,
    genres: HashMap<String, i64>,
    langs: HashMap<String, i64>,
    sequences: HashMap<String, i64>,
    titles: HashMap<String, i64>,
}
impl <'a> Book {
    pub fn new(conn: sal::Connection, access: AccessGuard) -> Fb2Result<Self> {
        let mut book = Book {
            allowed: 0,
            counter: 0,
            archive: 0,
            connection: conn,
            access: access,
            people: HashMap::new(),
            genres: HashMap::new(),
            langs: HashMap::new(),
            sequences: HashMap::new(),
            titles: HashMap::new(),
        };
        book.load_dictionaries()?;
        Ok(book)
    }
    pub fn select_archive(&mut self, archive: &str) -> Fb2Result<()> {
        self.archive = sal::get_archive_id_by_name(&self.connection, archive)?;
        Ok(())
    }
    fn load_dictionaries(&mut self) -> Fb2Result<()> {
        let people = sal::load_people(&self.connection)?;
        for (name, id) in &people {
           self.people.insert(People::from(name.clone()), *id);
        }
        Ok(())
    }
}
impl <'a> algorithm::Visitor<'a> for Book{
    type Type = ZipFile<'a> ;
    fn visit(&mut self, zip: &mut Self::Type) {
        self.counter += 1;
        match archive::load_fb2(zip) {
            Ok(book) => if self.archive != 0 && self.access.is_allowed(&book) {
                self.allowed += 1;
                let file_name = zip.name();
                let compression_method = zip.compression();
                let compressed_size = zip.compressed_size();
                let original_size = zip.size();
                let src32 = zip.crc32();
                let offset = zip.offset();
            },
            Err(err) => println!("{}", err),
        }

    }
    fn get_count(&self) -> usize {
        self.counter
    }
    fn report(&self) {
        println!("Handled {} files in archive, and {} allowed.", self.counter, self.allowed);
        println!("Known people count {}.", self.people.len());
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