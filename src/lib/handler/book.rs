use sal;
use algorithm;
use result::Fb2Result;
use visitor::acess;
use visitor::author::Author;
use visitor::list::Book;

fn create_access_guard(db: &str)-> Fb2Result<acess::AccessGuard> {
    let conn = sal::get_connection(db)?;
    let langs: Vec<String> = sal::get_languages_disabled(&conn)?;
    let genres: Vec<String> = sal::get_genre_codes_disabled(&conn)?;
    let mut access = acess::AccessGuard::new();
    access.disable_langs(langs);
    access.disable_genres(genres);
    Ok(access)
}

fn load_genres(db: &str)-> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    Ok(())
}



pub fn ls(db: &str, archives: &Vec<&str>) -> Fb2Result<()> {
    let access_guard = create_access_guard(db)?;
    let mut visitor = Book::new(access_guard);
    for archive in archives {
        algorithm::visit(archive, &mut visitor)?;
    }
    visitor.report();
    Ok(())
}


pub fn authors(db: &str, load: bool, archives: &Vec<&str>) -> Fb2Result<()> {
    let access_guard = create_access_guard(db)?;
    let mut visitor = Author::new(access_guard);
    for archive in archives {
        algorithm::visit(archive, &mut visitor)?;
    }
    if load {

    } else {
        visitor.report();
    }

    Ok(())
}