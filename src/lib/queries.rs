/*
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
*/

#[allow(dead_code)]
pub const CREATE_ARCHIVES: &'static str = "
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
pub const CREATE_PIECES: &'static str = "
    CREATE TABLE IF NOT EXISTS pieces (
	    id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    archive_id  	INTEGER NOT NULL, /* FK to archives.id */
	    piece_idx       INTEGER NOT NULL,
	    hash      	    TEXT NOT NULL
    );";

#[allow(dead_code)]
pub const CREATE_BOOKS: &'static str = "
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
pub const CREATE_GENRES: &'static str = "
    CREATE TABLE genres (
	    id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    text      	    TEXT NOT NULL UNIQUE
    );";

#[allow(dead_code)]
pub const CREATE_PEOPLE: &'static str = "
    CREATE TABLE people (
	    id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
        use_id          INTEGER, /* use rew with id == this.use_id instead */
	    first_name 	    TEXT NOT NULL,
        middle_name	    TEXT NOT NULL,
        last_name	    TEXT NOT NULL,
        nickname	    TEXT NOT NULL
    );";

#[allow(dead_code)]
pub const CREATE_TITLES: &'static str = "
    CREATE TABLE titles (
	    id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    text      	    TEXT NOT NULL UNIQUE
    );";

#[allow(dead_code)]
pub const CREATE_LANGUAGES: &'static str = "
    CREATE TABLE languages (
	    id  	        INTEGER NOT NULL PRIMARY KEY UNIQUE,
	    text      	    TEXT NOT NULL UNIQUE
    );
	CREATE TRIGGER languages_auto AFTER INSERT ON languages
	BEGIN
	    UPDATE	languages
    	SET 	id = (SELECT max(id) + 1 FROM languages)
    	WHERE   ROWID = new.ROWID;
	END;";

#[allow(dead_code)]
pub const CREATE_IGNORED_LANGUAGES: &'static str = "
	CREATE TABLE ignored_languages (
	    id  	            INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    language_id         INTEGER    /* FK to languages.id */
    );";

#[allow(dead_code)]
pub const CREATE_EXPECTED_LANGUAGES: &'static str = "
	CREATE VIEW expected_languages
		AS SELECT languages.id, languages.text
		FROM languages LEFT JOIN ignored_language ON languages.id = ignored_language.language_id
		WHERE language_id is null;";

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


/*********************** Insert or Update Queries ***********************/
pub const INSERT_ARCHIVE: &'static str = "
	INSERT OR IGNORE
	INTO archives (name, created, hash, total_length, piece_length, pieces_count)
	VALUES (?, ?, ?, ?, ?, ?)
	";

pub const INSERT_PIECE: &'static str = "
	INSERT INTO pieces (archive_id, piece_idx, hash) VALUES (?, ?, ?)";

pub const INSERT_LANGUAGES: &'static str = "
    INSERT OR IGNORE INTO languages (id, text) VALUES (0, ?);";

/**************************** Select Queries ****************************/
pub const GET_ID_BY_HASH: &'static str = "
	SELECT id FROM archives WHERE hash = ?1";

pub const GET_INDEX_AND_HASH_BY_ARCH_ID: &'static str = "
	SELECT piece_idx, hash FROM pieces WHERE archive_id = ?1";

pub const GET_ARCH_SIZES_BY_NAME: &'static str = "
	SELECT id, total_length, piece_length, pieces_count FROM archives WHERE name = ?1";

pub const GET_HASH_BY_ARCH_ID_AND_INDEX: &'static str = "
	SELECT hash FROM pieces WHERE archive_id = ?1 AND piece_idx = ?2";
