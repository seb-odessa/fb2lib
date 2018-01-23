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
		WHERE filters_def.filtered_id IS NULL;";

#[allow(dead_code)]
pub const FILTERS: &'static str = "
	CREATE TABLE IF NOT EXISTS filters (
	    id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    name      	    TEXT NOT NULL UNIQUE
	);";

#[allow(dead_code)]
pub const FILTERS_DEF: &'static str = "
	CREATE TABLE IF NOT EXISTS filters_def (
	    id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	    filter_id       INTEGER NOT NULL,  	/* FK to filters.id */
		filtered_id     INTEGER NOT NULL   	/* FK to id  of the filtered table, e.g. languages.id*/
	);";

#[allow(dead_code)]
pub const FILL_FILTER: &'static str = "
	INSERT OR IGNORE INTO filters VALUES (?, ?);";

#[allow(dead_code)]
pub const GENRE_SUBSYSTEM: &'static str = "
	BEGIN;
        CREATE TABLE genre_map (
			id		INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    		code	TEXT NOT NULL UNIQUE,
    		name	TEXT NOT NULL
		);
		INSERT INTO genre_map (code, name) VALUES ('sf_history', 	'альтернативная история');
		INSERT INTO genre_map (code, name) VALUES ('sf_action', 	'боевая фантастика');
		INSERT INTO genre_map (code, name) VALUES ('sf_epic', 		'эпическая фантастика');
		INSERT INTO genre_map (code, name) VALUES ('sf_heroic', 	'героическая фантастика');
		INSERT INTO genre_map (code, name) VALUES ('sf_detective', 	'детективная фантастика');
		INSERT INTO genre_map (code, name) VALUES ('sf_cyberpunk', 	'киберпанк');
		INSERT INTO genre_map (code, name) VALUES ('sf_space', 		'космическая фантастика');
		INSERT INTO genre_map (code, name) VALUES ('sf_social', 	'социально-психологическая фантастика');
		INSERT INTO genre_map (code, name) VALUES ('sf_horror', 	'ужасы и мистика');
		INSERT INTO genre_map (code, name) VALUES ('sf_humor', 		'юмористическая фантастика');
		INSERT INTO genre_map (code, name) VALUES ('humor_fantasy',	'юмористическая фэнтези');
		INSERT INTO genre_map (code, name) VALUES ('sf_fantasy', 	'фэнтези');
		INSERT INTO genre_map (code, name) VALUES ('fantasy', 		'фэнтези');
		INSERT INTO genre_map (code, name) VALUES ('sf', 			'научная фантастика');
		INSERT INTO genre_map (code, name) VALUES ('fantastic',		'фантастика');
		INSERT INTO genre_map (code, name) VALUES ('sf_litrpg',		'литературное RPG');
		INSERT INTO genre_map (code, name) VALUES ('love_sf',		'романтическая фантастика');
		INSERT INTO genre_map (code, name) VALUES ('popadanec',		'попаданцы');
		

		INSERT INTO genre_map (code, name) VALUES ('det_classic', 	'классический детектив');
		INSERT INTO genre_map (code, name) VALUES ('det_police', 	'полицейский детектив');
		INSERT INTO genre_map (code, name) VALUES ('det_action', 	'боевик');
		INSERT INTO genre_map (code, name) VALUES ('det_irony', 	'иронический детектив');
		INSERT INTO genre_map (code, name) VALUES ('det_history', 	'исторический детектив');
		INSERT INTO genre_map (code, name) VALUES ('det_espionage', 'шпионский детектив');
		INSERT INTO genre_map (code, name) VALUES ('det_crime', 	'криминальный детектив');
		INSERT INTO genre_map (code, name) VALUES ('det_political', 'политический детектив');
		INSERT INTO genre_map (code, name) VALUES ('det_maniac', 	'маньяки');
		INSERT INTO genre_map (code, name) VALUES ('det_hard', 		'крутой детектив');
		INSERT INTO genre_map (code, name) VALUES ('thriller', 		'триллер');
		INSERT INTO genre_map (code, name) VALUES ('detective', 	'детектив');

		INSERT INTO genre_map (code, name) VALUES ('prose_classic', 		'классическая проза');
		INSERT INTO genre_map (code, name) VALUES ('prose_history', 		'историческая проза');
		INSERT INTO genre_map (code, name) VALUES ('prose_contemporary', 	'современная проза');
		INSERT INTO genre_map (code, name) VALUES ('prose_counter', 		'контркультура');
		INSERT INTO genre_map (code, name) VALUES ('prose_military', 		'военная проза');
		INSERT INTO genre_map (code, name) VALUES ('prose_rus_classic', 	'русская классическая проза');
		INSERT INTO genre_map (code, name) VALUES ('prose_su_classics', 	'советская классическая проза');
		INSERT INTO genre_map (code, name) VALUES ('prose_su_classic', 		'советская классическая проза');		
		INSERT INTO genre_map (code, name) VALUES ('prose', 				'прочая проза');

		INSERT INTO genre_map (code, name) VALUES ('love_contemporary', 'современные любовные романы');
		INSERT INTO genre_map (code, name) VALUES ('love_history', 		'исторические любовные романы');
		INSERT INTO genre_map (code, name) VALUES ('love_detective', 	'остросюжетные любовные романы');
		INSERT INTO genre_map (code, name) VALUES ('love_short', 		'короткие любовные романы');
		INSERT INTO genre_map (code, name) VALUES ('love_erotica', 		'эротика');

		INSERT INTO genre_map (code, name) VALUES ('adv_western', 	'вестерн');
		INSERT INTO genre_map (code, name) VALUES ('adv_history', 	'исторические приключения');
		INSERT INTO genre_map (code, name) VALUES ('adv_indian', 	'приключения про индейцев');
		INSERT INTO genre_map (code, name) VALUES ('adv_maritime', 	'морские приключения');
		INSERT INTO genre_map (code, name) VALUES ('adv_geo', 		'путешествия и география');
		INSERT INTO genre_map (code, name) VALUES ('adv_animal', 	'природа и животные');
		INSERT INTO genre_map (code, name) VALUES ('adventure', 	'прочие приключения');

		INSERT INTO genre_map (code, name) VALUES ('child_tale', 		'сказка');
		INSERT INTO genre_map (code, name) VALUES ('child_verse', 		'детские стихи');
		INSERT INTO genre_map (code, name) VALUES ('child_prose', 		'детская проза');
		INSERT INTO genre_map (code, name) VALUES ('child_sf', 			'детская фантастика');
		INSERT INTO genre_map (code, name) VALUES ('child_det',			'детские остросюжетные');
		INSERT INTO genre_map (code, name) VALUES ('child_adv', 		'детские приключения');
		INSERT INTO genre_map (code, name) VALUES ('child_education', 	'детская образовательная литература');
		INSERT INTO genre_map (code, name) VALUES ('children', 			'прочая детская литература');

		INSERT INTO genre_map (code, name) VALUES ('poetry', 		'поэзия');
		INSERT INTO genre_map (code, name) VALUES ('dramaturgy', 	'драматургия');

		INSERT INTO genre_map (code, name) VALUES ('antique_ant', 		'античная литература');
		INSERT INTO genre_map (code, name) VALUES ('antique_european', 	'европейская старинная литература');
		INSERT INTO genre_map (code, name) VALUES ('antique_russian', 	'древнерусская литература');
		INSERT INTO genre_map (code, name) VALUES ('antique_east', 		'древневосточная литература');
		INSERT INTO genre_map (code, name) VALUES ('antique_myths', 	'мифы, легенды, эпос');
		INSERT INTO genre_map (code, name) VALUES ('antique', 			'прочая старинная литература');

		INSERT INTO genre_map (code, name) VALUES ('sci_history', 		'история');
		INSERT INTO genre_map (code, name) VALUES ('sci_psychology', 	'психология');
		INSERT INTO genre_map (code, name) VALUES ('sci_culture', 		'культурология');
		INSERT INTO genre_map (code, name) VALUES ('sci_religion', 		'религиоведение');
		INSERT INTO genre_map (code, name) VALUES ('sci_philosophy',	'философия');
		INSERT INTO genre_map (code, name) VALUES ('sci_politics', 		'политика');
		INSERT INTO genre_map (code, name) VALUES ('sci_business', 		'деловая литература');
		INSERT INTO genre_map (code, name) VALUES ('sci_juris', 		'юриспруденция');
		INSERT INTO genre_map (code, name) VALUES ('sci_linguistic', 	'языкознание');
		INSERT INTO genre_map (code, name) VALUES ('sci_medicine', 		'медицина');
		INSERT INTO genre_map (code, name) VALUES ('sci_phys', 			'физика');
		INSERT INTO genre_map (code, name) VALUES ('sci_math', 			'математика');
		INSERT INTO genre_map (code, name) VALUES ('sci_chem', 			'химия');
		INSERT INTO genre_map (code, name) VALUES ('sci_biology', 		'биология');
		INSERT INTO genre_map (code, name) VALUES ('sci_philology',		'филология');
		INSERT INTO genre_map (code, name) VALUES ('sci_tech', 			'технические науки');
		INSERT INTO genre_map (code, name) VALUES ('sci_popular', 		'научно-популярная литература');
		INSERT INTO genre_map (code, name) VALUES ('sci_economic', 		'экономика');
		INSERT INTO genre_map (code, name) VALUES ('science', 			'прочая научная литература');
    
		INSERT INTO genre_map (code, name) VALUES ('comp_www', 			'интернет');
		INSERT INTO genre_map (code, name) VALUES ('comp_programming', 	'программирование');
		INSERT INTO genre_map (code, name) VALUES ('comp_hard', 		'компьютерное железо (аппаратное обеспечение)');
		INSERT INTO genre_map (code, name) VALUES ('comp_soft', 		'программы');
		INSERT INTO genre_map (code, name) VALUES ('comp_db', 			'базы данных');
		INSERT INTO genre_map (code, name) VALUES ('comp_osnet', 		'ос и сети');
		INSERT INTO genre_map (code, name) VALUES ('network_literature','сети');
		INSERT INTO genre_map (code, name) VALUES ('computers', 		'прочая околокомпьтерная литература');

		INSERT INTO genre_map (code, name) VALUES ('ref_encyc', 	'энциклопедии');
		INSERT INTO genre_map (code, name) VALUES ('ref_dict', 		'словари');
		INSERT INTO genre_map (code, name) VALUES ('ref_ref', 		'справочники');
		INSERT INTO genre_map (code, name) VALUES ('ref_guide', 	'руководства');
		INSERT INTO genre_map (code, name) VALUES ('reference', 	'прочая справочная литература');

		INSERT INTO genre_map (code, name) VALUES ('nonf_biography', 	'биографии и Мемуары');
		INSERT INTO genre_map (code, name) VALUES ('nonf_publicism',	'публицистика');
		INSERT INTO genre_map (code, name) VALUES ('nonf_criticism', 	'критика');		
		INSERT INTO genre_map (code, name) VALUES ('nonfiction', 		'прочая документальная литература');

		INSERT INTO genre_map (code, name) VALUES ('religion_rel', 			'религия');
		INSERT INTO genre_map (code, name) VALUES ('religion_esoterics', 	'эзотерика');
		INSERT INTO genre_map (code, name) VALUES ('religion_orthodoxy', 	'православие');
		INSERT INTO genre_map (code, name) VALUES ('religion_protestantism','протестантизм');
		INSERT INTO genre_map (code, name) VALUES ('religion_budda', 		'буддизм');
		INSERT INTO genre_map (code, name) VALUES ('religion_self', 		'самосовершенствование');
		INSERT INTO genre_map (code, name) VALUES ('religion', 				'прочая религионая литература');

		INSERT INTO genre_map (code, name) VALUES ('humor_anecdote', 	'анекдоты');
		INSERT INTO genre_map (code, name) VALUES ('humor_prose', 		'юмористическая проза');
		INSERT INTO genre_map (code, name) VALUES ('humor_verse', 		'юмористические стихи');
		INSERT INTO genre_map (code, name) VALUES ('humor', 			'прочий юмор');

		INSERT INTO genre_map (code, name) VALUES ('home_cooking', 		'кулинария');
		INSERT INTO genre_map (code, name) VALUES ('home_pets', 		'домашние животные');
		INSERT INTO genre_map (code, name) VALUES ('home_crafts', 		'хобби и ремесла');
		INSERT INTO genre_map (code, name) VALUES ('home_entertain',	'развлечения');
		INSERT INTO genre_map (code, name) VALUES ('home_health', 		'здоровье');
		INSERT INTO genre_map (code, name) VALUES ('home_garden', 		'сад и огород');
		INSERT INTO genre_map (code, name) VALUES ('home_diy', 			'сделай сам');
		INSERT INTO genre_map (code, name) VALUES ('home_sport', 		'спорт');
		INSERT INTO genre_map (code, name) VALUES ('home_sex', 			'эротика, секс');
		INSERT INTO genre_map (code, name) VALUES ('home', 				'прочиее домоводство');

		INSERT INTO genre_map (code, name) VALUES ('design', 			'искусство и дизайн');
		INSERT INTO genre_map (code, name) VALUES ('scenarios', 		'сценарии');
		INSERT INTO genre_map (code, name) VALUES ('erotica', 			'эротика');
		INSERT INTO genre_map (code, name) VALUES ('love', 				'любовные романы');

		CREATE TABLE genres (
	    	id  	INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	    	text    TEXT NOT NULL UNIQUE
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
pub const PEOPLE: &'static str = "
    CREATE TABLE people (
	    id  	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
        use_id          INTEGER, /* use rew with id == this.use_id instead */
	    first_name 	    TEXT NOT NULL,
        middle_name	    TEXT NOT NULL,
        last_name	    TEXT NOT NULL,
        nickname	    TEXT NOT NULL
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

