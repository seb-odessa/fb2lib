use sal;
use types;
use archive;
use result::Fb2Result;
use visitor::guard::Guard;
use types::{FileDescription, BlobDescription, BookDescription};

use zip::ZipFile;
use crypto::sha1::Sha1;
use crypto::digest::Digest;
use std::error::Error;
use std::collections::HashSet;

pub struct Description {
    arch_count: usize,
    book_count: usize,
    archive_id: i64,
    archive_name: String,
    accepted: Vec<BookDescription>,
    already_known: HashSet<String>,
    hasher: Sha1,
    connection: sal::Connection,
}
impl <'a> Description
{
    pub fn new(conn: sal::Connection, access: Guard) -> Fb2Result<Self> {
        Ok(
            Self {
                arch_count: 0,
                book_count: 0,
                archive_id: 0,
                archive_name: String::new(),
                accepted: Vec::new(),
                already_known: sal::load_known_books(&conn)?,
                hasher: Sha1::new(),
                connection: conn,
            }
        )
    }

    pub fn select_archive(&mut self, archive: &str) -> Fb2Result<()> {
        self.archive_id = sal::get_archive_id_by_name(&self.connection, archive)?;
        self.archive_name = String::from(archive);
        self.arch_count += 1;
        Ok(())
    }

    pub fn get_stat(&self)-> Fb2Result<sal::STATUS> {
        sal::get_archive_status(&self.connection, &self.archive_name, sal::TASK::DESC)
    }

    pub fn set_stat(&self, status: sal::STATUS)-> Fb2Result<()> {
        sal::set_archive_status(&self.connection, &self.archive_name, sal::get_task_id(sal::TASK::DESC), sal::get_status_id(status))
    }

    pub fn save_collected(&mut self) -> Fb2Result<()> {
        sal::save_books(&mut self.connection, &self.accepted)?;
        let saved: HashSet<String> = self.accepted.iter().map(|desc| desc.blob.sha1.clone()).collect();
        self.already_known = self.already_known.union(&saved).map(|s| s.clone()).collect();
        self.accepted.clear();
        Ok(())
    }
}

impl sal::Save for Description {
    fn save(&mut self, _: &sal::Connection) -> Fb2Result<()> {
        Ok(())
    }

    fn task(&self) -> sal::TASK {
        sal::TASK::DESC
    }
}

impl <'a> types::MutVisitor<'a> for Description{

    type Type = ZipFile<'a> ;
    fn visit(&mut self, zip: &mut Self::Type) {
        if self.archive_id != 0 {
            self.book_count += 1;
            match archive::load_fb2(zip) {
                Ok(book) => {
                    if let Some(bytes) = book.save() {
                        self.hasher.input(&bytes);
                        let sha1 = self.hasher.result_str();
                        let blob = BlobDescription::from(bytes, sha1);
                        self.hasher.reset();
                        if !self.already_known.contains(&blob.sha1) {
                            let file = FileDescription::from(zip);
                            let desc = BookDescription::from((self.archive_id, file, blob));
                            self.accepted.push(desc);
                        }
                    }
                },
                Err(e) => {
                    print!("\n\t Failed to process {} file in {} archive with error: {}.",
                             zip.name(),
                             self.archive_name,
                             e.description());
                },
            }
        }
    }


    fn get_visited(&self) -> usize {
        self.book_count
    }

    fn get_accepted(&self) -> usize {
        self.accepted.len()
    }

    fn get_already_known(&self) -> usize {
        self.already_known.len()
    }

    fn report(&self) {
        println!("Handled {} archives, and {} files", self.arch_count, self.book_count);
        println!("Files processed in this session {}.", self.accepted.len());
        println!("Total files processed {}.", self.already_known.len());
    }

}
