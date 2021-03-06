
#[allow(dead_code)]
pub const PEOPLE_SUBSYSTEM: &'static str = "
	BEGIN;
        DROP VIEW IF EXISTS authors;
        DROP VIEW IF EXISTS authors_fixed;
        DROP TABLE IF EXISTS people;
        DROP TABLE IF EXISTS people_links;
        DROP TABLE IF EXISTS names;
        DELETE FROM progress WHERE progress.task_id = 3;
    COMMIT;";

#[allow(dead_code)]
pub const TITLES_SUBSYSTEM: &'static str = "
	BEGIN;
	DROP TABLE IF EXISTS titles;
    DROP TABLE IF EXISTS titles_links;
    DROP VIEW IF EXISTS titles_joined;
    DELETE FROM progress WHERE progress.task_id = 4;
    COMMIT;";

#[allow(dead_code)]
pub const SEQUENCES_SUBSYSTEM: &'static str = "
	BEGIN;
	DROP TABLE IF EXISTS sequences;
    DELETE FROM progress WHERE progress.task_id = 5;
    COMMIT;";

#[allow(dead_code)]
pub const VERSION_SUBSYSTEM: &'static str = "
	BEGIN;
	DROP TABLE IF EXISTS versions;
    /*DELETE FROM progress WHERE progress.task_id = 5;*/
    COMMIT;";

#[allow(dead_code)]
pub const DESC_SUBSYSTEM: &'static str = "
	BEGIN;
	DROP TABLE IF EXISTS books;
    DROP TABLE IF EXISTS book_titles;
    DROP TABLE IF EXISTS book_langs;
    DROP TABLE IF EXISTS book_genres;
    DROP TABLE IF EXISTS book_authors;
    DROP TABLE IF EXISTS book_sequences;
    DROP TABLE IF EXISTS book_translators;
    COMMIT;";


/*********************** Untested ***********************/
