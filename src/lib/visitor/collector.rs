use sal;
use algorithm;
use result::Fb2Result;
use visitor::acess::AccessGuard;
use fb2parser::FictionBook;

use visitor::author::Author;
use visitor::sequence::Sequence;
use visitor::title::Title;

pub struct Collector {
    counter: usize,
    access: AccessGuard,
    authors: Author,
    sequences: Sequence,
    titles: Title,
}
impl Collector {
    pub fn new(access: AccessGuard, conn: &sal::Connection) -> Fb2Result<Self> {
        Ok(Collector {
            counter: 0,
            access: access,
            authors: Author::new(AccessGuard::all(), sal::select_people(&conn)?),
            sequences: Sequence::new(AccessGuard::all(), sal::select_sequences(&conn)?),
            titles: Title::new(AccessGuard::all(), sal::select_titles(&conn)?),
        })
    }
}
impl sal::Save for Collector {
    fn save(&mut self, conn: &sal::Connection) -> Fb2Result<()> {
        self.authors.save(conn)?;
        self.sequences.save(conn)?;
        self.titles.save(conn)?;
        Ok(())
    }
    fn task(&self) -> sal::TASK {
        sal::TASK::UNDEFINED
    }
    fn get_new_count(&self) -> usize {
        self.authors.get_new_count() + self.sequences.get_new_count() + self.titles.get_new_count()
    }
    fn get_stored_count(&self) -> usize {
        self.authors.get_stored_count() + self.sequences.get_stored_count() + self.titles.get_stored_count()
    }
    fn set_status(&self, conn: &sal::Connection, archive: &str, status: sal::STATUS) -> Fb2Result<()> {
        let status_id = sal::get_status_id(status);
        sal::set_archive_status(conn, archive, sal::get_task_id(self.authors.task()), status_id)?;
        sal::set_archive_status(conn, archive, sal::get_task_id(self.sequences.task()), status_id)?;
        sal::set_archive_status(conn, archive, sal::get_task_id(self.titles.task()), status_id)
    }
}
impl algorithm::Visitor<FictionBook> for Collector {
    fn visit(&mut self, book: &mut FictionBook) {
        self.counter += 1;
        if self.access.is_allowed(book) {
            self.authors.visit(book);
            self.sequences.visit(book);
            self.titles.visit(book);
        }
    }
    fn get_count(&self) -> usize {
        self.counter
    }
    fn report(&self) {
        self.authors.report();
        self.sequences.report();
        self.titles.report();
    }
}
