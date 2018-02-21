#[allow(dead_code)]
pub const ARCHIVES: &'static str = "DROP TABLE archives;";
#[allow(dead_code)]
pub const PIECES: &'static str = "DROP TABLE pieces;";


#[allow(dead_code)]
pub const LANGUAGES: &'static str = "
	BEGIN;
    DROP TABLE IF EXISTS languages;
    DELETE FROM progress WHERE progress.task_id = 1;
    COMMIT;";

#[allow(dead_code)]
pub const FILTER_SUBSYSTEM: &'static str = "
	BEGIN;
    DROP INDEX IF EXISTS filter_def_index;
    DROP TABLE IF EXISTS filters;
    DROP TABLE IF EXISTS filters_def;
    COMMIT;";

#[allow(dead_code)]
pub const LANGUAGES_DISABLED: &'static str = "DROP VIEW IF EXISTS languages_disabled;";
#[allow(dead_code)]
pub const LANGUAGES_ENABLED: &'static str = "DROP VIEW IF EXISTS languages_enabled;";

#[allow(dead_code)]
pub const GENRE_SUBSYSTEM: &'static str = "
	BEGIN;
        DROP VIEW IF EXISTS genres;
        DROP VIEW IF EXISTS genres_enabled;
        DROP VIEW IF EXISTS genres_disabled;
        DROP TABLE IF EXISTS genre_synonyms;
        DROP TABLE IF EXISTS genre_groups;
        DROP TABLE IF EXISTS genre_names;
    COMMIT;";

#[allow(dead_code)]
pub const PEOPLE_SUBSYSTEM: &'static str = "
	BEGIN;
        DROP VIEW IF EXISTS authors;
        DROP TABLE IF EXISTS people;
        DELETE FROM progress WHERE progress.task_id = 3;
    COMMIT;";


#[allow(dead_code)]
pub const PROGRESS_SUBSYSTEM: &'static str = "
	BEGIN;
    DROP VIEW IF EXISTS progress_log;
	DROP TABLE IF EXISTS task;
    DROP TABLE IF EXISTS status;
    DROP TABLE IF EXISTS progress;
    COMMIT;";

#[allow(dead_code)]
pub const TITLES_SUBSYSTEM: &'static str = "
	BEGIN;
	DROP TABLE IF EXISTS titles;
    DELETE FROM progress WHERE progress.task_id = 4;
    COMMIT;";

#[allow(dead_code)]
pub const SEQUENCES_SUBSYSTEM: &'static str = "
	BEGIN;
	DROP TABLE IF EXISTS sequences;
    DELETE FROM progress WHERE progress.task_id = 5;
    COMMIT;";


/*********************** Untested ***********************/

#[allow(dead_code)]
pub const BOOKS: &'static str = "DROP TABLE books;";

#[allow(dead_code)]
pub const TITLES: &'static str = "DROP TABLE titles;";


