use sal;
use result::Fb2Result;
use tools;
use algorithm;

pub fn display(db_file_name: &str) -> Fb2Result<()> {
    let conn = sal::get_connection(db_file_name)?;
    println!("======== DISABLED ========");
    let mut grp = String::new();
    for (group, name) in sal::get_genres_disabled(&conn)? {
        if grp != group {
            println!("{}", tools::capitalize(group.clone()));
            grp = group.clone();
        }
        println!("\t{}", tools::capitalize(name));
    }
    println!("======== ENABLED ========");
    for (group, name) in sal::get_genres_enabled(&conn)? {
        if grp != group {
            println!("{}", tools::capitalize(group.clone()));
            grp = group.clone();
        }
        println!("\t{}", tools::capitalize(name));
    }
    println!("");
    Ok(())
}

pub fn enable(db_file_name: &str, name: &str, is_grp: bool) -> Fb2Result<()> {
    let conn = sal::get_connection(db_file_name)?;
    let re = algorithm::make_regex(name)?;
    if is_grp {
        for group in sal::get_genre_groups_disabled(&conn)? {
            if re.is_match(&group) {
                sal::enable_genre_group(&conn, &group)?;
                println!("The genre group '{}' was enabled", tools::capitalize(group));
            }
        }
    } else {
        for (_, genre) in sal::get_genres_disabled(&conn)? {
            if re.is_match(&genre) {
                sal::enable_genre(&conn, &genre)?;
                println!("The genre '{}' was enabled", tools::capitalize(genre));
            }
        }
    }
    Ok(())
}

pub fn disable(db_file_name: &str, name: &str, is_grp: bool) -> Fb2Result<()> {
    let conn = sal::get_connection(db_file_name)?;
    let re = algorithm::make_regex(name)?;
    if is_grp {
        for group in sal::get_genre_groups_enabled(&conn)? {
            if re.is_match(&group) {
                sal::disable_genre_group(&conn, &group)?;
                println!("The genre group '{}' was disabled", tools::capitalize(group));
            }
        }
    } else {
        for (_, genre) in sal::get_genres_enabled(&conn)? {
            if re.is_match(&genre) {
                sal::disable_genre(&conn, &genre)?;
                println!("The genre '{}' was disabled", tools::capitalize(genre));
            }
        }
    }
    Ok(())
}