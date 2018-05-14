use sal;
use algorithm;
use result::Fb2Result;
use visitor::acess::AccessGuard;
use fb2parser::FictionBook;

use visitor::name::Name;
use visitor::title::Title;
use visitor::sequence::Sequence;

pub struct Collector {
    counter: usize,
    access: AccessGuard,
    names: Name,
    titles: Title,
    sequences: Sequence,

}
impl Collector {
    pub fn new(access: AccessGuard, conn: &sal::Connection) -> Fb2Result<Self> {
        Ok(Collector {
            counter: 0,
            access: access,
            names: Name::new(AccessGuard::all(), sal::load_names(&conn)?),
            titles: Title::new(AccessGuard::all(), sal::load_titles(&conn)?),
            sequences: Sequence::new(AccessGuard::all(), sal::load_sequences(&conn)?),

        })
    }
}
impl sal::Save for Collector {
    fn save(&mut self, conn: &sal::Connection) -> Fb2Result<()> {
        self.names.save(conn)?;
        self.titles.save(conn)?;
        self.sequences.save(conn)?;
        Ok(())
    }
    fn task(&self) -> sal::TASK {
        sal::TASK::UNDEFINED
    }
    fn get_new_count(&self) -> usize {
        self.names.get_new_count() + self.titles.get_new_count() + self.sequences.get_new_count()
    }
    fn get_stored_count(&self) -> usize {
        self.names.get_stored_count() + self.titles.get_stored_count() + self.sequences.get_stored_count()
    }
    fn set_status(&self, conn: &sal::Connection, archive: &str, status: sal::STATUS) -> Fb2Result<()> {
        let status_id = sal::get_status_id(status);
        sal::set_archive_status(conn, archive, sal::get_task_id(self.names.task()), status_id)?;
        sal::set_archive_status(conn, archive, sal::get_task_id(self.titles.task()), status_id)?;
        sal::set_archive_status(conn, archive, sal::get_task_id(self.sequences.task()), status_id)?;
        Ok(())
    }
}
impl <'a> algorithm::Visitor<'a> for Collector {
    type Type = FictionBook;
    fn visit(&mut self, book: &mut FictionBook) {
        self.counter += 1;
        if self.access.is_allowed(book) {
            self.names.visit(book);
            self.titles.visit(book);
            self.sequences.visit(book);
        }
    }
    fn get_count(&self) -> usize {
        self.counter
    }
    fn report(&self) {
        self.names.report();
        self.titles.report();
        self.sequences.report();
    }
}
