
#[allow(dead_code)]
pub const TORRENTS_SUBSYSTEM: &'static str = "
	BEGIN;
	DROP TABLE IF EXISTS archives;
    CREATE TABLE IF NOT EXISTS archives (
	    id         	    INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    name   	        TEXT NOT NULL,
	    created    	    TEXT NOT NULL,
	    hash       	    TEXT NOT NULL UNIQUE,
	    total_length	INTEGER NOT NULL,
	    piece_length	INTEGER NOT NULL,
	    pieces_count	INTEGER NOT NULL
    );
	DROP TABLE IF EXISTS pieces;
    CREATE TABLE IF NOT EXISTS pieces (
	    id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    archive_id  	INTEGER NOT NULL, /* FK to archives.id */
	    piece_idx       INTEGER NOT NULL,
	    hash      	    TEXT NOT NULL
    );
	COMMIT;";


#[allow(dead_code)]
pub const PROGRESS_SUBSYSTEM: &'static str = "
	BEGIN;

	DROP TABLE IF EXISTS task;
	CREATE TABLE task (
    	id 		INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    	name 	TEXT NOT NULL UNIQUE ON CONFLICT IGNORE
	);
	INSERT OR IGNORE INTO task VALUES (1, 'Заполнение справочника языков');
	INSERT OR IGNORE INTO task VALUES (2, 'Заполнение справочника жанров');
	INSERT OR IGNORE INTO task VALUES (3, 'Заполнение справочника имен');
	INSERT OR IGNORE INTO task VALUES (4, 'Заполнение справочника названий');
	INSERT OR IGNORE INTO task VALUES (5, 'Заполнение справочника циклов');
    INSERT OR IGNORE INTO task VALUES (6, 'Заполнение справочника авторов');
    INSERT OR IGNORE INTO task VALUES (7, 'Заполнение таблицы описаний');

    DROP TABLE IF EXISTS status;
	CREATE TABLE status (
    	id 		INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    	name 	TEXT NOT NULL UNIQUE
	);
    INSERT OR IGNORE INTO status VALUES (1, 'обработка архива начата');
    INSERT OR IGNORE INTO status VALUES (2, 'обработка архива завершена');
	INSERT OR IGNORE INTO status VALUES (3, 'операция завершена');
    INSERT OR IGNORE INTO status VALUES (4, 'операция завершилась неудачей');

	DROP TABLE IF EXISTS progress;
	CREATE TABLE progress (
    	id 				INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
		archive_id      INTEGER NOT NULL,       /* FK to archives.id */
		task_id 		INTEGER NOT NULL,       /* FK to task.id */
		status_id 		INTEGER NOT NULL,       /* FK to status.id */
		registred		TEXT,
		UNIQUE (archive_id, task_id) ON CONFLICT REPLACE
	);

	DROP VIEW IF EXISTS progress_log;
	CREATE VIEW progress_log AS
	SELECT progress.id AS id, archives.name, task.name AS task, status.name AS status, registred
	FROM progress
	LEFT JOIN status ON progress.status_id = status.id
	LEFT JOIN task ON progress.task_id = task.id
	LEFT JOIN archives ON progress.archive_id = archives.id;

	DROP INDEX IF EXISTS progress_archive_index;
	CREATE INDEX progress_archive_index on progress (archive_id ASC);

	DROP TRIGGER IF EXISTS progress_on_insert;
	CREATE TRIGGER progress_on_insert AFTER INSERT ON progress
	BEGIN
		UPDATE progress SET registred = datetime('now') WHERE new.id = progress.id;
	END;

	DROP TRIGGER IF EXISTS progress_on_update;
	CREATE TRIGGER progress_on_update AFTER UPDATE ON progress
	BEGIN
		UPDATE progress SET registred = datetime('now') WHERE new.id = progress.id AND new.status_id != old.status_id;
	END;
    COMMIT;";

#[allow(dead_code)]
pub const FILTER_SUBSYSTEM: &'static str = "
	BEGIN;
    DROP TABLE IF EXISTS filters;
	CREATE TABLE IF NOT EXISTS filters (
	    id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    name      	    TEXT NOT NULL UNIQUE
	);
	DROP TABLE IF EXISTS filters_def;
	CREATE TABLE IF NOT EXISTS filters_def (
	    id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    filter_id       INTEGER NOT NULL,  	/* FK to filters.id */
		filtered_id     INTEGER NOT NULL   	/* FK to id  of the filtered table, e.g. languages.id*/
	);
	INSERT OR IGNORE INTO filters VALUES (1, 'lang');
    INSERT OR IGNORE INTO filters VALUES (2, 'genre');

	DROP INDEX IF EXISTS filter_def_index;
	CREATE INDEX filter_def_index on filters_def (filter_id ASC, filtered_id ASC);
	COMMIT;";


#[allow(dead_code)]
pub const LANGUAGE_SUBSYSTEM: &'static str = "
	BEGIN;

	DELETE FROM progress WHERE progress.task_id = 1;

    DROP TABLE IF EXISTS languages;

    CREATE TABLE IF NOT EXISTS languages (
	    id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    name      	    TEXT NOT NULL UNIQUE
    );

	DROP VIEW IF EXISTS languages_disabled;

	CREATE VIEW IF NOT EXISTS languages_disabled AS
		SELECT languages.id, languages.name
		FROM languages LEFT JOIN filters_def
		ON filters_def.filter_id = (select id from filters where name = \"lang\")
		AND languages.id = filters_def.filtered_id
		WHERE filters_def.filtered_id IS NOT NULL;

	DROP VIEW IF EXISTS languages_enabled;

	CREATE VIEW IF NOT EXISTS languages_enabled AS
		SELECT languages.id, languages.name
		FROM languages LEFT JOIN filters_def
		ON filters_def.filter_id = (select id from filters where name = \"lang\")
		AND languages.id = filters_def.filtered_id
		WHERE filters_def.filtered_id IS NULL;

	COMMIT;";


#[allow(dead_code)]
pub const GENRE_SUBSYSTEM: &'static str = "
	BEGIN;
        DROP TABLE IF EXISTS genre_groups;
        CREATE TABLE genre_groups (
			id		INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    		name	TEXT NOT NULL
		);

		DROP TABLE IF EXISTS genre_names;
        CREATE TABLE genre_names (
			id			INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
			group_id	INTEGER NOT NULL,       /* FK to genre_groups.id */
    		code		TEXT NOT NULL UNIQUE,
    		name		TEXT NOT NULL
		);

		DROP TABLE IF EXISTS genre_synonyms;
		CREATE TABLE genre_synonyms (
			id			INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
			code		TEXT NOT NULL UNIQUE,	/* code */
			synonym_id 	INTEGER NOT NULL        /* FK to genre_names.id */
		);

		DROP VIEW IF EXISTS genres;
		CREATE VIEW genres AS
		SELECT A.id, C.code as code, B.name as type, A.name as name
		FROM genre_names A LEFT JOIN genre_groups B ON A.group_id = B.id JOIN genre_synonyms C ON A.id = C.synonym_id
		UNION
		SELECT A.id, A.code as code, B.name as type, A.name as name
		FROM genre_names A LEFT JOIN genre_groups B ON A.group_id = B.id;

		DROP VIEW IF EXISTS genres_enabled;
		CREATE VIEW IF NOT EXISTS genres_enabled AS
		SELECT genre_names.id, genre_groups.name AS group_name, genre_names.name AS genre_name
		FROM genre_names
		JOIN genre_groups ON genre_names.group_id = genre_groups.id
		LEFT JOIN filters_def ON genre_names.id = filtered_id AND filter_id = (SELECT id FROM filters WHERE name = 'genre')
		WHERE filtered_id IS NULL;

		DROP VIEW IF EXISTS genres_disabled;
		CREATE VIEW IF NOT EXISTS genres_disabled AS
		SELECT genre_names.id, genre_groups.name AS group_name, genre_names.name AS genre_name
		FROM genre_names
		JOIN genre_groups ON genre_names.group_id = genre_groups.id
		LEFT JOIN filters_def ON genre_names.id = filtered_id AND filter_id = (SELECT id FROM filters WHERE name = 'genre')
		WHERE filtered_id IS NOT NULL;

    COMMIT;";

#[allow(dead_code)]
pub const NAMES_SUBSYSTEM: &'static str = "
    BEGIN;
    DELETE FROM progress WHERE progress.task_id = 3;
	DROP TABLE IF EXISTS names;
	CREATE TABLE names (
    	id 		INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    	name 	TEXT NOT NULL
	);
    COMMIT;
";


#[allow(dead_code)]
pub const PEOPLE_SUBSYSTEM: &'static str = "
	BEGIN;

    DELETE FROM progress WHERE progress.task_id = 6;
    DROP TABLE IF EXISTS people;
    CREATE TABLE people (
	    id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    first_name_id   INTEGER NOT NULL,	/* FK to names.id */
        middle_name_id  INTEGER NOT NULL,	/* FK to names.id */
        last_name_id	INTEGER NOT NULL,	/* FK to names.id */
        nick_name_id    INTEGER NOT NULL,	/* FK to names.id */
		UNIQUE (first_name_id, middle_name_id, last_name_id, nick_name_id) ON CONFLICT IGNORE
    );

    DROP VIEW IF EXISTS authors;
	CREATE VIEW IF NOT EXISTS authors AS
		SELECT
	        p.id as id,
	        l.name || ' ' || f.name || ' ' || m.name as full_name,
	        f.name as first_name,
	        m.name as middle_name,
	        l.name as last_name,
	        n.name as nick_name
        FROM people p
	        JOIN names f on (p.first_name_id = f.id)
	        JOIN names m on (p.middle_name_id = m.id)
	        JOIN names l on (p.last_name_id = l.id)
	        JOIN names n on (p.nick_name_id = n.id);

    COMMIT;";


#[allow(dead_code)]
pub const TITLES_SUBSYSTEM: &'static str = "
	BEGIN;
    DROP TABLE IF EXISTS titles;
	CREATE TABLE titles (
   		id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    title       	TEXT NOT NULL,
		UNIQUE (title) ON CONFLICT IGNORE
    );
    COMMIT;";

#[allow(dead_code)]
pub const SEQUENCES_SUBSYSTEM: &'static str = "
	BEGIN;
    DROP TABLE IF EXISTS sequences;
	CREATE TABLE sequences (
   		id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    sequence       	TEXT NOT NULL,
		UNIQUE (sequence) ON CONFLICT IGNORE
    );
    COMMIT;";

#[allow(dead_code)]
pub const VERSION_SUBSYSTEM: &'static str = "
	BEGIN;
	CREATE TABLE versions (
    	id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    	name TEXT NOT NULL,
		date TEXT NOT NULL,
    	description TEXT
	);
	CREATE TRIGGER versions_on_insert AFTER INSERT ON versions
	BEGIN
		UPDATE versions SET date = datetime('now') WHERE new.id = versions.id;
	END;
    COMMIT;";


pub const LINKS_SUBSYSTEM: &'static str = "
    BEGIN;
    DROP TABLE IF EXISTS people_links;
	CREATE TABLE people_links (
    	src_id 		INTEGER NOT NULL,	/* FK to people.id */
    	dst_id 		INTEGER NOT NULL,	/* FK to people.id */
        UNIQUE (src_id, dst_id) ON CONFLICT IGNORE
	);

    DROP VIEW IF EXISTS people_joined;
    CREATE VIEW people_joined AS
        SELECT
	        ifnull(B.id,  A.id) AS id,
	        ifnull(B.first_name_id,  A.first_name_id) AS first_name_id,
	        ifnull(B.middle_name_id,  A.middle_name_id) AS middle_name_id,
	        ifnull(B.last_name_id,  A.last_name_id) AS last_name_id,
	        ifnull(B.nick_name_id,  A.nick_name_id) AS nick_name_id
        FROM people A LEFT JOIN people_links ON src_id = A.id LEFT JOIN people B ON dst_id = B.id;

    DROP VIEW IF EXISTS authors_joined;
    CREATE VIEW IF NOT EXISTS authors_joined AS
		SELECT DISTINCT
	        p.id as id,
	        l.name || ' ' || f.name || ' ' || m.name as full_name,
	        f.name as first_name,
	        m.name as middle_name,
	        l.name as last_name,
	        n.name as nick_name
        FROM people_joined p
	        JOIN names f on (p.first_name_id = f.id)
	        JOIN names m on (p.middle_name_id = m.id)
	        JOIN names l on (p.last_name_id = l.id)
	        JOIN names n on (p.nick_name_id = n.id);

    DROP TABLE IF EXISTS sequences_links;
	CREATE TABLE sequences_links (
    	src_id 		INTEGER NOT NULL,	/* FK to sequences.id */
    	dst_id 		INTEGER NOT NULL,	/* FK to sequences.id */
        UNIQUE (src_id, dst_id) ON CONFLICT IGNORE
	);

    DROP VIEW IF EXISTS sequences_joined;
	CREATE VIEW sequences_joined AS
		SELECT DISTINCT
	        ifnull(B.id, A.id) AS id,
	        ifnull(B.sequence, A.sequence) AS sequence
        FROM sequences A LEFT JOIN sequences_links ON src_id = A.id LEFT JOIN sequences B ON dst_id = B.id;

    DROP TABLE IF EXISTS titles_links;
	CREATE TABLE titles_links (
    	src_id 		INTEGER NOT NULL,	/* FK to titles.id */
    	dst_id 		INTEGER NOT NULL,	/* FK to titles.id */
        UNIQUE (src_id, dst_id) ON CONFLICT IGNORE
	);

    DROP VIEW IF EXISTS titles_joined;
	CREATE VIEW titles_joined AS
		SELECT DISTINCT
	        ifnull(B.id, A.id) AS id,
	        ifnull(B.title, A.title) AS title
        FROM titles A LEFT JOIN titles_links ON src_id = A.id LEFT JOIN titles B ON dst_id = B.id;
    COMMIT;";


/*********************************************************************************************************************
Current hierarcy of supported/parsed data
<description>
	<title-info> - 1 (один, обязателен);
		<genre> - 1..n (любое число, один обязaтелен);
			text: String
		<author> - 1..n (любое число, один обязaтелен);
		    <first-name> - 0..1 (один, обязателен при отсутствии <nickname>, иначе опционально) - имя;
				text: String
			<middle-name> - 0..1 (один, опционально) - отчество;
				text: String
			<last-name> - 0..1 (один, обязателен при отсутствии <nickname>, иначе опционально) - фамилия;
				text: String
			<nickname> - 0..1 (один, обязателен при отсутствии <first-name> и <last-name>, иначе опционально);
				text: String
		<book-title> - 1 (один, обязателен);
			text: String
		<lang> - 1 (один, обязателен);
			text: String
		<src-lang> - 0..1 (один, опционально);
			text: String
		<translator> - 0..n (любое число, опционально);
		    <first-name> - 0..1 (один, обязателен при отсутствии <nickname>, иначе опционально) - имя;
				text: String
			<middle-name> - 0..1 (один, опционально) - отчество;
				text: String
			<last-name> - 0..1 (один, обязателен при отсутствии <nickname>, иначе опционально) - фамилия;
				text: String
			<nickname> - 0..1 (один, обязателен при отсутствии <first-name> и <last-name>, иначе опционально);
				text: String
		<sequence> - 0..n (любое число, опционально).
				number: Number
				name:	String
    <document-info> - 1 (один, обязателен);
	    <author> - 1..n (любое число, один обязaтелен);
		    <first-name> - 0..1 (один, обязателен при отсутствии <nickname>, иначе опционально) - имя;
				text: String
			<middle-name> - 0..1 (один, опционально) - отчество;
				text: String
			<last-name> - 0..1 (один, обязателен при отсутствии <nickname>, иначе опционально) - фамилия;
				text: String
			<nickname> - 0..1 (один, обязателен при отсутствии <first-name> и <last-name>, иначе опционально);
				text: String
		<program-used> - 0..1 (один, опционально);
			text: String
		<date> - 1 (один, обязателен);
			value: String
			text: String
		<publisher> - 0..n (любое число, опционально) с версии 2.2.
			text: String
	<publish-info> - 0..1 (один, опционально);
		<book-name> - 0..1 (один, опционально) - название;
			text: String
		<publisher> - 0..1 (один, опционально) - издательство;
			text: String
			... ?
		<city> - 0..1 (один, опционально)- место издания;
			text: String
		<year> - 0..1 (один, опционально) - год издания;
			text: String
		<isbn> - 0..1 (один, опционально) - ISBN издания;
			text: String
		<sequence> - 0..n (любое число, опционально) - серия (серии) изданий, в которую входит книга.
			number: Number
			name:	String
*********************************************************************************************************************/

/*********************************************************************************************************************
Book
	Title
	Lang
	Genres : array
	Authors : array
	Sequence : array
	Translator : array
*********************************************************************************************************************/
#[allow(dead_code)]
pub const DESC_SUBSYSTEM: &'static str = "
	BEGIN;

	CREATE TABLE books (
        id              	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
        archive_id      	INTEGER NOT NULL,       /* FK to archives.id */
		file_name 			TEXT NOT NULL,
        compression_method	INTEGER,
        compressed_size     INTEGER,
        original_size       INTEGER,
		src32         		INTEGER,
        offset              INTEGER,
        size                INTEGER,
        description         BLOB,
        sha1                TEXT NOT NULL,
        UNIQUE (file_name, src32) ON CONFLICT IGNORE
    );
	CREATE TABLE book_titles (
		id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
		book_id      	INTEGER NOT NULL,       /* FK to books.id */
		title_id      	INTEGER NOT NULL        /* FK to titles.id */
	);
	CREATE TABLE book_langs (
		id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
		book_id      	INTEGER NOT NULL,       /* FK to books.id */
		lang_id      	INTEGER NOT NULL        /* FK to languages.id */
	);
	CREATE TABLE book_genres (
		id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
		book_id      	INTEGER NOT NULL,       /* FK to books.id */
		genre_id      	INTEGER NOT NULL        /* FK to genres.id */
	);
	CREATE TABLE book_authors (
		id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
		book_id      	INTEGER NOT NULL,       /* FK to books.id */
		author_id      	INTEGER NOT NULL        /* FK to authors.id */
	);
	CREATE TABLE book_sequences (
		id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
		book_id      	INTEGER NOT NULL,       /* FK to books.id */
		sequence_id     INTEGER NOT NULL,       /* FK to sequence.id */
		sequence_number	INTEGER					/* the number of the book in the sequence */
	);
	CREATE TABLE book_translators (
		id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
		book_id      	INTEGER NOT NULL,       /* FK to books.id */
		author_id      	INTEGER NOT NULL        /* FK to authors.id */
	);
    COMMIT;";



/*********************** Untested ***********************/

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
