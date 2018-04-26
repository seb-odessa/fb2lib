use sal;
use archive;
use algorithm;
use result::Fb2Result;
use visitor::acess::AccessGuard;
use fb2parser::FictionBook;
use types::FileDesc;
use types::People;

use zip::ZipFile;
use std::collections::HashSet;
use std::collections::HashMap;


pub struct Book {
    allowed: usize,
    counter: usize,
    archive_id: i64,
    guard: AccessGuard,
    books_added: HashSet<FileDesc>,
    books_handled: HashSet<FileDesc>,

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
            archive_id: 0,
            guard: access,
            books_added: HashSet::new(),
            books_handled: sal::load_books(&conn)?,
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
        self.archive_id = sal::get_archive_id_by_name(&self.connection, archive)?;
        Ok(())
    }
}
impl <'a> algorithm::Visitor<'a> for Book{
    type Type = ZipFile<'a> ;
    fn visit(&mut self, zip: &mut Self::Type) {
        self.counter += 1;
        match archive::load_fb2(zip) {
            Ok(book) => if self.archive_id != 0 && self.guard.is_allowed(&book) {
                self.allowed += 1;
                let desc: FileDesc = FileDesc::from((self.archive_id, zip));
                if !self.books_handled.contains(&desc) {
                    self.books_added.insert(desc);
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
        println!("Books registered in DB {}.", self.books_handled.len());
        println!("Books ready to save to the DB {}.", self.books_added.len());
        println!("Known people count {}.", self.people.len());
        println!("Known genres count {}.", self.genres.len());
        println!("Known languages count {}.", self.languages.len());
        println!("Known titles count {} ({}).", self.titles.len(), t);
        println!("Known sequences count {}.", self.sequences.len());
    }

}
