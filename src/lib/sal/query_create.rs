
#[allow(dead_code)]
pub const ARCHIVES: &'static str = "
    CREATE TABLE IF NOT EXISTS archives (
	    id         	    INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    name   	        TEXT NOT NULL,
	    created    	    TEXT NOT NULL,
	    hash       	    TEXT NOT NULL UNIQUE,
	    total_length	INTEGER NOT NULL,
	    piece_length	INTEGER NOT NULL,
	    pieces_count	INTEGER NOT NULL
    );";

#[allow(dead_code)]
pub const PIECES: &'static str = "
    CREATE TABLE IF NOT EXISTS pieces (
	    id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    archive_id  	INTEGER NOT NULL, /* FK to archives.id */
	    piece_idx       INTEGER NOT NULL,
	    hash      	    TEXT NOT NULL
    );";


#[allow(dead_code)]
pub const LANGUAGES: &'static str = "
    CREATE TABLE IF NOT EXISTS languages (
	    id  	        INTEGER NOT NULL PRIMARY KEY UNIQUE,
	    name      	    TEXT NOT NULL UNIQUE
    );";

#[allow(dead_code)]
pub const LANGUAGES_AUTO: &'static str = "
	CREATE TRIGGER IF NOT EXISTS languages_auto AFTER INSERT ON languages
	BEGIN
	    UPDATE	languages
    	SET 	id = (SELECT max(id) + 1 FROM languages)
    	WHERE   ROWID = new.ROWID;
	END;";

#[allow(dead_code)]
pub const LANGUAGES_DISABLED: &'static str = "
	CREATE VIEW IF NOT EXISTS languages_disabled AS
		SELECT languages.id, languages.name
		FROM languages LEFT JOIN filters_def
		ON filters_def.filter_id = (select id from filters where name = \"lang\") AND languages.id = filters_def.filtered_id
		WHERE filters_def.filtered_id IS NOT NULL;";

#[allow(dead_code)]
pub const LANGUAGES_ENABLED: &'static str = "
	CREATE VIEW IF NOT EXISTS languages_enabled AS
		SELECT languages.id, languages.name
		FROM languages LEFT JOIN filters_def
		ON filters_def.filter_id = (select id from filters where name = \"lang\") AND languages.id = filters_def.filtered_id
		WHERE filters_def.filtered_id IS NULL;
		;";

#[allow(dead_code)]
pub const FILTER_SUBSYSTEM: &'static str = "
	BEGIN;
	CREATE TABLE IF NOT EXISTS filters (
	    id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    name      	    TEXT NOT NULL UNIQUE
	);
	CREATE TABLE IF NOT EXISTS filters_def (
	    id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    filter_id       INTEGER NOT NULL,  	/* FK to filters.id */
		filtered_id     INTEGER NOT NULL   	/* FK to id  of the filtered table, e.g. languages.id*/
	);
	CREATE INDEX filter_def_index on filters_def (filter_id ASC, filtered_id ASC);
	COMMIT;";


#[allow(dead_code)]
pub const GENRE_SUBSYSTEM: &'static str = "
	BEGIN;
        CREATE TABLE genre_groups (
			id		INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    		name	TEXT NOT NULL
		);

        CREATE TABLE genre_names (
			id			INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
			group_id	INTEGER NOT NULL,       /* FK to genre_groups.id */
    		code		TEXT NOT NULL UNIQUE,
    		name		TEXT NOT NULL
		);

		CREATE TABLE genre_synonyms (
			id			INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
			code		TEXT NOT NULL UNIQUE,	/* code */
			synonym_id 	INTEGER NOT NULL        /* FK to genre_names.id */
		);

		CREATE VIEW genres AS
		SELECT A.id, C.code as code, B.name as type, A.name as name
		FROM genre_names A LEFT JOIN genre_groups B ON A.group_id = B.id JOIN genre_synonyms C ON A.id = C.synonym_id
		UNION
		SELECT A.id, A.code as code, B.name as type, A.name as name
		FROM genre_names A LEFT JOIN genre_groups B ON A.group_id = B.id;

		CREATE VIEW IF NOT EXISTS genres_enabled AS
		SELECT genre_names.id, genre_groups.name AS group_name, genre_names.name AS genre_name
		FROM genre_names
		JOIN genre_groups ON genre_names.group_id = genre_groups.id
		LEFT JOIN filters_def ON genre_names.id = filtered_id AND filter_id = (SELECT id FROM filters WHERE name = 'genre')
		WHERE filtered_id IS NULL;

		CREATE VIEW IF NOT EXISTS genres_disabled AS
		SELECT genre_names.id, genre_groups.name AS group_name, genre_names.name AS genre_name
		FROM genre_names
		JOIN genre_groups ON genre_names.group_id = genre_groups.id
		LEFT JOIN filters_def ON genre_names.id = filtered_id AND filter_id = (SELECT id FROM filters WHERE name = 'genre')
		WHERE filtered_id IS NOT NULL;


    COMMIT;";

#[allow(dead_code)]
pub const PEOPLE_SUBSYSTEM: &'static str = "
	BEGIN;
    CREATE TABLE people (
	    id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
        use_id          INTEGER, /* use row with id == this.use_id instead */
	    first_name 	    TEXT NOT NULL,
        middle_name	    TEXT NOT NULL,
        last_name	    TEXT NOT NULL,
        nickname	    TEXT NOT NULL,
		UNIQUE (first_name, middle_name, last_name, nickname) ON CONFLICT IGNORE
    );
	CREATE VIEW IF NOT EXISTS authors AS
		SELECT id, use_id, nickname AS name, last_name, first_name, middle_name, nickname
		FROM people WHERE last_name =='' AND first_name =='' AND middle_name == ''
		UNION
		SELECT id, use_id, last_name AS name, last_name, first_name, middle_name, nickname
		FROM people WHERE last_name !='' AND first_name =='' AND middle_name == ''
		UNION
		SELECT id, use_id, last_name ||' '|| first_name AS name, last_name, first_name, middle_name, nickname
		FROM people WHERE last_name !='' AND first_name !='' AND middle_name == ''
		UNION
		SELECT id, use_id, last_name ||' '|| first_name ||' '|| middle_name AS name, last_name, first_name, middle_name, nickname
		FROM people WHERE last_name !='' AND first_name !='' AND middle_name != ''
		UNION
		SELECT id, use_id, first_name ||' '|| middle_name AS name, last_name, first_name, middle_name, nickname
		FROM people WHERE last_name =='' AND first_name !='' AND middle_name != ''
		UNION
		SELECT id, use_id, last_name ||' '|| middle_name AS name, last_name, first_name, middle_name, nickname
		FROM people WHERE last_name !='' AND first_name =='' AND middle_name != ''
		UNION
		SELECT id, use_id, middle_name AS name, last_name, first_name, middle_name, nickname
		FROM people WHERE last_name =='' AND first_name =='' AND middle_name != '';

    COMMIT;";

#[allow(dead_code)]
pub const PROGRESS_SUBSYSTEM: &'static str = "
	BEGIN;
	CREATE TABLE task (
    	id 		INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    	name 	TEXT NOT NULL UNIQUE
	);

	CREATE TABLE status (
    	id 		INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    	name 	TEXT NOT NULL UNIQUE
	);

	CREATE TABLE progress (
    	id 				INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,    	
		archive_id      INTEGER NOT NULL,       /* FK to archives.id */
		task_id 		INTEGER NOT NULL,       /* FK to task.id */
		status_id 		INTEGER NOT NULL,       /* FK to status.id */
		registred		TEXT,
		UNIQUE (archive_id) ON CONFLICT REPLACE
	);
	CREATE VIEW progress_log AS
	SELECT progress.id AS id, archives.name, task.name AS task, status.name AS status, registred
	FROM progress 
	LEFT JOIN status ON progress.status_id = status.id 
	LEFT JOIN task ON progress.task_id = task.id
	LEFT JOIN archives ON progress.archive_id = archives.id;

	CREATE INDEX progress_archive_index on progress (archive_id ASC);
	CREATE TRIGGER progress_on_insert AFTER INSERT ON progress
	BEGIN
		UPDATE progress SET registred = datetime('now') WHERE new.id = progress.id;
	END;
	CREATE TRIGGER progress_on_update AFTER UPDATE ON progress
	BEGIN
		UPDATE progress SET registred = datetime('now') WHERE new.id = progress.id AND new.status_id != old.status_id;
	END;
    COMMIT;";

#[allow(dead_code)]
pub const TITLES_SUBSYSTEM: &'static str = "
	BEGIN;
	CREATE TABLE titles (
   		id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
        use_id          INTEGER, /* use row with id == this.use_id instead */
	    title       	TEXT NOT NULL UNIQUE
    );
    COMMIT;";

/*********************** Untested ***********************/
#[allow(dead_code)]
pub const BOOKS: &'static str = "
    CREATE TABLE books (
        id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
        archive_id      INTEGER NOT NULL,       /* FK to archives.id */
        file_name       TEXT NOT NULL UNIQUE,   /* e.g.: book.fb2 */
        method          INTEGER,
        packed_size     INTEGER,
        unpacked_size   INTEGER,
        file_offset     INTEGER
    );";


#[allow(dead_code)]
pub const TITLES: &'static str = "
    CREATE TABLE titles (
	    id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    text      	    TEXT NOT NULL UNIQUE
    );";


#[allow(dead_code)]
pub const CREATE_SEQUENCES: &'static str = "
    CREATE TABLE sequences (
	    id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    text      	    TEXT NOT NULL UNIQUE
    );";

#[allow(dead_code)]
pub const CREATE_PUBLISHERS: &'static str = "
    CREATE TABLE publishers (
	    id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    text      	    TEXT NOT NULL UNIQUE
    );";

#[allow(dead_code)]
pub const CREATE_CITIES: &'static str = "
    CREATE TABLE cities (
	    id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    text      	    TEXT NOT NULL UNIQUE
    );";

#[allow(dead_code)]
pub const CREATE_ISBNS: &'static str = "
    CREATE TABLE isbns (
	    id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    text      	    TEXT NOT NULL UNIQUE
    );";

#[allow(dead_code)]
pub const CREATE_PROGRAMS: &'static str = "
    CREATE TABLE programs (
	    id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    text      	    TEXT NOT NULL UNIQUE
    );";

// Defines any people sets, e.g. Authors, Translators, etc.
#[allow(dead_code)]
pub const CREATE_PEOPLE_SET: &'static str = "
    CREATE TABLE people_set (
	    id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    people_id       INTEGER
    );";

#[allow(dead_code)]
pub const CREATE_TITLE_INFO: &'static str = "
    CREATE TABLE title_info (
	    id  	            INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    genre_id            INTEGER,    /* FK to genre.id */
        author_set_id       INTEGER,    /* FK to people_set.id */
        title_id            INTEGER,    /* FK to title.id */
        lang_id             INTEGER,    /* FK to languages.id */
        src_lang_id         INTEGER,    /* FK to languages.id */
        translator_set_id   INTEGER,    /* FK to people_set.id */
        sequence_id         INTEGER,    /* FK to sequences.id */
        sequence_number     INTEGER     /* the number of book in sequence */
    );";

#[allow(dead_code)]
pub const CREATE_DOCUMENT_INFO: &'static str = "
    CREATE TABLE document_info (
	    id  	            INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
		author_set_id       INTEGER,    /* FK to people_set.id */
		program_id          INTEGER,    /* FK to programs.id */
		created				TEXT,
		publishers_id       INTEGER     /* FK to publishers.id */
	);";

#[allow(dead_code)]
pub const CREATE_PUBLICH_INFO: &'static str = "
    CREATE TABLE publish_info (
	    id  	            INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
		book_name_id        INTEGER,    /* FK to title.id */
		created				TEXT,
		publishers_id       INTEGER,    /* FK to publishers.id */
		city_id       		INTEGER,    /* FK to cities.id */
		year				TEXT,
		isbn_id       		INTEGER,    /* FK to isbns.id */
        sequence_id         INTEGER,    /* FK to sequences.id */
        sequence_number     INTEGER     /* the number of book in sequence */
	);";

#[allow(dead_code)]
pub const CREATE_DESCRIPTION: &'static str = "
    CREATE TABLE description (
	    id  	             INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
		title_info_id        INTEGER,    /* FK to title_info.id */
		document_info_id     INTEGER,    /* FK to document_info.id */
        publish_info_id      INTEGER     /* FK to publish_info.id */
	);";

