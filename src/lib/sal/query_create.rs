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
		INSERT INTO genre_map (code, name) VALUES ('sf_history', 	'Альтернативная история');
		INSERT INTO genre_map (code, name) VALUES ('sf_action', 	'Боевая фантастика');
		INSERT INTO genre_map (code, name) VALUES ('sf_epic', 		'Эпическая фантастика');
		INSERT INTO genre_map (code, name) VALUES ('sf_heroic', 	'Героическая фантастика');
		INSERT INTO genre_map (code, name) VALUES ('sf_detective', 	'Детективная фантастика');
		INSERT INTO genre_map (code, name) VALUES ('sf_cyberpunk', 	'Киберпанк');
		INSERT INTO genre_map (code, name) VALUES ('sf_space', 		'Космическая фантастика');
		INSERT INTO genre_map (code, name) VALUES ('sf_social', 	'Социально-психологическая фантастика');
		INSERT INTO genre_map (code, name) VALUES ('sf_horror', 	'Ужасы и Мистика');
		INSERT INTO genre_map (code, name) VALUES ('sf_humor', 		'Юмористическая фантастика');
		INSERT INTO genre_map (code, name) VALUES ('sf_fantasy', 	'Фэнтези');
		INSERT INTO genre_map (code, name) VALUES ('sf', 			'Научная Фантастика');

		INSERT INTO genre_map (code, name) VALUES ('det_classic', 	'Классический детектив');
		INSERT INTO genre_map (code, name) VALUES ('det_police', 	'Полицейский детектив');
		INSERT INTO genre_map (code, name) VALUES ('det_action', 	'Боевик');
		INSERT INTO genre_map (code, name) VALUES ('det_irony', 	'Иронический детектив');
		INSERT INTO genre_map (code, name) VALUES ('det_history', 	'Исторический детектив');
		INSERT INTO genre_map (code, name) VALUES ('det_espionage', 'Шпионский детектив');
		INSERT INTO genre_map (code, name) VALUES ('det_crime', 	'Криминальный детектив');
		INSERT INTO genre_map (code, name) VALUES ('det_political', 'Политический детектив');
		INSERT INTO genre_map (code, name) VALUES ('det_maniac', 	'Маньяки');
		INSERT INTO genre_map (code, name) VALUES ('det_hard', 		'Крутой детектив');
		INSERT INTO genre_map (code, name) VALUES ('thriller', 		'Триллер');
		INSERT INTO genre_map (code, name) VALUES ('detective', 	'Детектив');

		INSERT INTO genre_map (code, name) VALUES ('prose_classic', 		'Классическая проза');
		INSERT INTO genre_map (code, name) VALUES ('prose_history', 		'Историческая проза');
		INSERT INTO genre_map (code, name) VALUES ('prose_contemporary', 	'Современная проза');
		INSERT INTO genre_map (code, name) VALUES ('prose_counter', 		'Контркультура');
		INSERT INTO genre_map (code, name) VALUES ('prose_rus_classic', 	'Русская классическая проза');
		INSERT INTO genre_map (code, name) VALUES ('prose_su_classics', 	'Советская классическая проза');

		INSERT INTO genre_map (code, name) VALUES ('love_contemporary', 'Современные любовные романы');
		INSERT INTO genre_map (code, name) VALUES ('love_history', 		'Исторические любовные романы');
		INSERT INTO genre_map (code, name) VALUES ('love_detective', 	'Остросюжетные любовные романы');
		INSERT INTO genre_map (code, name) VALUES ('love_short', 		'Короткие любовные романы');
		INSERT INTO genre_map (code, name) VALUES ('love_erotica', 		'Эротика');

		INSERT INTO genre_map (code, name) VALUES ('adv_western', 	'Вестерн');
		INSERT INTO genre_map (code, name) VALUES ('adv_history', 	'Исторические приключения');
		INSERT INTO genre_map (code, name) VALUES ('adv_indian', 	'Приключения про индейцев');
		INSERT INTO genre_map (code, name) VALUES ('adv_maritime', 	'Морские приключения');
		INSERT INTO genre_map (code, name) VALUES ('adv_geo', 		'Путешествия и география');
		INSERT INTO genre_map (code, name) VALUES ('adv_animal', 	'Природа и животные');
		INSERT INTO genre_map (code, name) VALUES ('adventure', 	'Прочие приключения');

		INSERT INTO genre_map (code, name) VALUES ('child_tale', 		'Сказка');
		INSERT INTO genre_map (code, name) VALUES ('child_verse', 		'Детские стихи');
		INSERT INTO genre_map (code, name) VALUES ('child_prose', 		'Детскиая проза');
		INSERT INTO genre_map (code, name) VALUES ('child_sf', 			'Детская фантастика');
		INSERT INTO genre_map (code, name) VALUES ('child_det',			'Детские остросюжетные');
		INSERT INTO genre_map (code, name) VALUES ('child_adv', 		'Детские приключения');
		INSERT INTO genre_map (code, name) VALUES ('child_education', 	'Детская образовательная литература');
		INSERT INTO genre_map (code, name) VALUES ('children', 			'Прочая детская литература');

		INSERT INTO genre_map (code, name) VALUES ('poetry', 		'Поэзия');
		INSERT INTO genre_map (code, name) VALUES ('dramaturgy', 	'Драматургия');

		INSERT INTO genre_map (code, name) VALUES ('antique_ant', 		'Античная литература');
		INSERT INTO genre_map (code, name) VALUES ('antique_european', 	'Европейская старинная литература');
		INSERT INTO genre_map (code, name) VALUES ('antique_russian', 	'Древнерусская литература');
		INSERT INTO genre_map (code, name) VALUES ('antique_east', 		'Древневосточная литература');
		INSERT INTO genre_map (code, name) VALUES ('antique_myths', 	'Мифы. Легенды. Эпос');
		INSERT INTO genre_map (code, name) VALUES ('antique', 			'Прочая старинная литература');

		INSERT INTO genre_map (code, name) VALUES ('sci_history', 		'История');
		INSERT INTO genre_map (code, name) VALUES ('sci_psychology', 	'Психология');
		INSERT INTO genre_map (code, name) VALUES ('sci_culture', 		'Культурология');
		INSERT INTO genre_map (code, name) VALUES ('sci_religion', 		'Религиоведение');
		INSERT INTO genre_map (code, name) VALUES ('sci_philosophy',	'Философия');
		INSERT INTO genre_map (code, name) VALUES ('sci_politics', 		'Политика');
		INSERT INTO genre_map (code, name) VALUES ('sci_business', 		'Деловая литература');
		INSERT INTO genre_map (code, name) VALUES ('sci_juris', 		'Юриспруденция');
		INSERT INTO genre_map (code, name) VALUES ('sci_linguistic', 	'Языкознание');
		INSERT INTO genre_map (code, name) VALUES ('sci_medicine', 		'Медицина');
		INSERT INTO genre_map (code, name) VALUES ('sci_phys', 			'Физика');
		INSERT INTO genre_map (code, name) VALUES ('sci_math', 			'Математика');
		INSERT INTO genre_map (code, name) VALUES ('sci_chem', 			'Химия');
		INSERT INTO genre_map (code, name) VALUES ('sci_biology', 		'Биология');
		INSERT INTO genre_map (code, name) VALUES ('sci_tech', 			'Технические науки');
		INSERT INTO genre_map (code, name) VALUES ('science', 			'Прочая научная литература');
    
		INSERT INTO genre_map (code, name) VALUES ('comp_www', 			'Интернет');
		INSERT INTO genre_map (code, name) VALUES ('comp_programming', 	'Программирование');
		INSERT INTO genre_map (code, name) VALUES ('comp_hard', 		'Компьютерное железо (аппаратное обеспечение)');
		INSERT INTO genre_map (code, name) VALUES ('comp_soft', 		'Программы');
		INSERT INTO genre_map (code, name) VALUES ('comp_db', 			'Базы данных');
		INSERT INTO genre_map (code, name) VALUES ('comp_osnet', 		'ОС и Сети');
		INSERT INTO genre_map (code, name) VALUES ('computers', 		'Прочая околокомпьтерная литература');

		INSERT INTO genre_map (code, name) VALUES ('ref_encyc', 	'Энциклопедии');
		INSERT INTO genre_map (code, name) VALUES ('ref_dict', 		'Словари');
		INSERT INTO genre_map (code, name) VALUES ('ref_ref', 		'Справочники');
		INSERT INTO genre_map (code, name) VALUES ('ref_guide', 	'Руководства');
		INSERT INTO genre_map (code, name) VALUES ('reference', 	'Прочая справочная литература');

		INSERT INTO genre_map (code, name) VALUES ('nonf_biography', 	'Биографии и Мемуары');
		INSERT INTO genre_map (code, name) VALUES ('nonf_publicism',	'Публицистика');
		INSERT INTO genre_map (code, name) VALUES ('nonf_criticism', 	'Критика');
		INSERT INTO genre_map (code, name) VALUES ('design', 			'Искусство и Дизайн');
		INSERT INTO genre_map (code, name) VALUES ('nonfiction', 		'Прочая документальная литература');

		INSERT INTO genre_map (code, name) VALUES ('religion_rel', 			'Религия');
		INSERT INTO genre_map (code, name) VALUES ('religion_esoterics', 	'Эзотерика');
		INSERT INTO genre_map (code, name) VALUES ('religion_self', 		'Самосовершенствование');
		INSERT INTO genre_map (code, name) VALUES ('religion', 				'Прочая религионая литература');

		INSERT INTO genre_map (code, name) VALUES ('humor_anecdote', 	'Анекдоты');
		INSERT INTO genre_map (code, name) VALUES ('humor_prose', 		'Юмористическая проза');
		INSERT INTO genre_map (code, name) VALUES ('humor_verse', 		'Юмористические стихи');
		INSERT INTO genre_map (code, name) VALUES ('humor', 			'Прочий юмор');

		INSERT INTO genre_map (code, name) VALUES ('home_cooking', 		'Кулинария');
		INSERT INTO genre_map (code, name) VALUES ('home_pets', 		'Домашние животные');
		INSERT INTO genre_map (code, name) VALUES ('home_crafts', 		'Хобби и ремесла');
		INSERT INTO genre_map (code, name) VALUES ('home_entertain',	'Развлечения');
		INSERT INTO genre_map (code, name) VALUES ('home_health', 		'Здоровье');
		INSERT INTO genre_map (code, name) VALUES ('home_garden', 		'Сад и огород');
		INSERT INTO genre_map (code, name) VALUES ('home_diy', 			'Сделай сам');
		INSERT INTO genre_map (code, name) VALUES ('home_sport', 		'Спорт');
		INSERT INTO genre_map (code, name) VALUES ('home_sex', 			'Эротика, Секс');
		INSERT INTO genre_map (code, name) VALUES ('home', 				'Прочиее домоводство');

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

