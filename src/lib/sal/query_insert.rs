
pub const ARCHIVE: &'static str = "
	INSERT OR IGNORE
	INTO archives (name, created, hash, total_length, piece_length, pieces_count)
	VALUES (?, ?, ?, ?, ?, ?)
	";

pub const PIECE: &'static str = "
	INSERT INTO pieces (archive_id, piece_idx, hash) VALUES (?, ?, ?)";

pub const LANGUAGES: &'static str = "
    INSERT OR IGNORE INTO languages (id, name) VALUES (0, ?);";

pub const DISABLE_LANGUAGE: &'static str = "
 	INSERT INTO filters_def (filter_id, filtered_id)
	SELECT filters.id, languages.id
	FROM filters, languages WHERE filters.name = \"lang\" AND languages.name = ?;";

pub const ENABLE_LANGUAGE: &'static str = "
	DELETE from filters_def WHERE id = (
 	SELECT filters_def.id FROM filters_def
 		JOIN filters ON filters_def.filter_id = filters.id
 		JOIN languages ON filters_def.filtered_id = languages.id
 		WHERE filters.name = \"lang\" AND languages.name = ?
 	)";

pub const INSERT_GENRES: &'static str = "
	BEGIN;
	INSERT INTO genre_groups (id, name) VALUES (1, 'приключения');
	INSERT INTO genre_groups (id, name) VALUES (2, 'древняя литература');
	INSERT INTO genre_groups (id, name) VALUES (3, 'детская литература');
	INSERT INTO genre_groups (id, name) VALUES (4, 'компьютерная литература');
	INSERT INTO genre_groups (id, name) VALUES (5, 'детектив');
	INSERT INTO genre_groups (id, name) VALUES (6, 'поэзия и драматургия');
	INSERT INTO genre_groups (id, name) VALUES (7, 'домоводство');
	INSERT INTO genre_groups (id, name) VALUES (8, 'дамские романы');
	INSERT INTO genre_groups (id, name) VALUES (9, 'прочее');
	INSERT INTO genre_groups (id, name) VALUES (10, 'фантастика и фэнтези');
	INSERT INTO genre_groups (id, name) VALUES (11, 'проза');
	INSERT INTO genre_groups (id, name) VALUES (12, 'справочная литература');
	INSERT INTO genre_groups (id, name) VALUES (13, 'религия');
	INSERT INTO genre_groups (id, name) VALUES (14, 'научная и научно-популярная литература');
	INSERT INTO genre_groups (id, name) VALUES (15, 'юмористическая литература');

	INSERT INTO genre_names (group_id, code, name) VALUES (1, 'adv_animal',   'природа и животные');
	INSERT INTO genre_names (group_id, code, name) VALUES (1, 'adv_geo',      'путешествия и география');
	INSERT INTO genre_names (group_id, code, name) VALUES (1, 'adv_history',  'исторические приключения');
	INSERT INTO genre_names (group_id, code, name) VALUES (1, 'adv_indian',   'приключения про индейцев');
	INSERT INTO genre_names (group_id, code, name) VALUES (1, 'adv_maritime', 'морские приключения');
	INSERT INTO genre_names (group_id, code, name) VALUES (1, 'adv_western',  'вестерн');
	INSERT INTO genre_names (group_id, code, name) VALUES (1, 'adventure',    'прочие приключения');

	INSERT INTO genre_names (group_id, code, name) VALUES (2, 'antique_ant', 'античная литература');
	INSERT INTO genre_names (group_id, code, name) VALUES (2, 'antique_east', 'древневосточная литература');
	INSERT INTO genre_names (group_id, code, name) VALUES (2, 'antique_european', 'европейская старинная литература');
	INSERT INTO genre_names (group_id, code, name) VALUES (2, 'antique_myths', 'мифы, легенды, эпос');
	INSERT INTO genre_names (group_id, code, name) VALUES (2, 'antique_russian', 'древнерусская литература');
	INSERT INTO genre_names (group_id, code, name) VALUES (2, 'antique', 'прочая старинная литература');

	INSERT INTO genre_names (group_id, code, name) VALUES (3, 'child_adv', 'детские приключения');
	INSERT INTO genre_names (group_id, code, name) VALUES (3, 'child_det', 'детские остросюжетные');
	INSERT INTO genre_names (group_id, code, name) VALUES (3, 'child_education', 'детская образовательная литература');
	INSERT INTO genre_names (group_id, code, name) VALUES (3, 'child_prose', 'детская проза');
	INSERT INTO genre_names (group_id, code, name) VALUES (3, 'child_sf', 'детская фантастика');
	INSERT INTO genre_names (group_id, code, name) VALUES (3, 'child_tale', 'сказка');
	INSERT INTO genre_names (group_id, code, name) VALUES (3, 'child_verse', 'детские стихи');
	INSERT INTO genre_names (group_id, code, name) VALUES (3, 'children', 'прочая детская литература');

	INSERT INTO genre_names (group_id, code, name) VALUES (4, 'comp_db', 'базы данных');
	INSERT INTO genre_names (group_id, code, name) VALUES (4, 'comp_hard', 'компьютерное железо (аппаратное обеспечение)');
	INSERT INTO genre_names (group_id, code, name) VALUES (4, 'comp_osnet', 'ос и сети');
	INSERT INTO genre_names (group_id, code, name) VALUES (4, 'comp_programming', 'программирование');
	INSERT INTO genre_names (group_id, code, name) VALUES (4, 'comp_soft', 'программы');
	INSERT INTO genre_names (group_id, code, name) VALUES (4, 'comp_www', 'интернет');
	INSERT INTO genre_names (group_id, code, name) VALUES (4, 'design', 'искусство и дизайн');
	INSERT INTO genre_names (group_id, code, name) VALUES (4, 'computers', 'прочая околокомпьтерная литература');

	INSERT INTO genre_names (group_id, code, name) VALUES (5, 'det_action', 'боевик');
	INSERT INTO genre_names (group_id, code, name) VALUES (5, 'det_classic', 'классический детектив');
	INSERT INTO genre_names (group_id, code, name) VALUES (5, 'det_crime', 'криминальный детектив');
	INSERT INTO genre_names (group_id, code, name) VALUES (5, 'det_espionage', 'шпионский детектив');
	INSERT INTO genre_names (group_id, code, name) VALUES (5, 'det_hard', 'крутой детектив');
	INSERT INTO genre_names (group_id, code, name) VALUES (5, 'det_history', 'исторический детектив');
	INSERT INTO genre_names (group_id, code, name) VALUES (5, 'det_irony', 'иронический детектив');
	INSERT INTO genre_names (group_id, code, name) VALUES (5, 'det_maniac', 'маньяки');
	INSERT INTO genre_names (group_id, code, name) VALUES (5, 'det_police', 'полицейский детектив');
	INSERT INTO genre_names (group_id, code, name) VALUES (5, 'det_political', 'политический детектив');
	INSERT INTO genre_names (group_id, code, name) VALUES (5, 'thriller', 'триллер');
	INSERT INTO genre_names (group_id, code, name) VALUES (5, 'detective', 'детектив');

	INSERT INTO genre_names (group_id, code, name) VALUES (6, 'dramaturgy', 'драматургия');
	INSERT INTO genre_names (group_id, code, name) VALUES (6, 'poetry', 'поэзия');
	INSERT INTO genre_names (group_id, code, name) VALUES (6, 'lyrics', 'лирика');
	
	INSERT INTO genre_names (group_id, code, name) VALUES (7, 'home_cooking', 'кулинария');
	INSERT INTO genre_names (group_id, code, name) VALUES (7, 'home_crafts', 'хобби и ремесла');
	INSERT INTO genre_names (group_id, code, name) VALUES (7, 'home_diy', 'сделай сам');
	INSERT INTO genre_names (group_id, code, name) VALUES (7, 'home_entertain', 'развлечения');
	INSERT INTO genre_names (group_id, code, name) VALUES (7, 'home_garden', 'сад и огород');
	INSERT INTO genre_names (group_id, code, name) VALUES (7, 'home_health', 'здоровье');
	INSERT INTO genre_names (group_id, code, name) VALUES (7, 'home_pets', 'домашние животные');
	INSERT INTO genre_names (group_id, code, name) VALUES (7, 'home_sex', 'эротика, секс');
	INSERT INTO genre_names (group_id, code, name) VALUES (7, 'home_sport', 'спорт');
	INSERT INTO genre_names (group_id, code, name) VALUES (7, 'home', 'прочиее домоводство');

	INSERT INTO genre_names (group_id, code, name) VALUES (8, 'love', 'любовные романы');
	INSERT INTO genre_names (group_id, code, name) VALUES (8, 'love_contemporary', 'современные любовные романы');
	INSERT INTO genre_names (group_id, code, name) VALUES (8, 'love_detective', 'остросюжетные любовные романы');
	INSERT INTO genre_names (group_id, code, name) VALUES (8, 'love_erotica', 'любовно-эротические романы');
	INSERT INTO genre_names (group_id, code, name) VALUES (8, 'love_history', 'исторические любовные романы');
	INSERT INTO genre_names (group_id, code, name) VALUES (8, 'love_sf', 'романтическая фантастика');
	INSERT INTO genre_names (group_id, code, name) VALUES (8, 'love_short', 'короткие любовные романы');
	INSERT INTO genre_names (group_id, code, name) VALUES (8, 'foreign_love', 'неразделённая любовь');
	INSERT INTO genre_names (group_id, code, name) VALUES (8, 'erotica', 'эротика');
		
	INSERT INTO genre_names (group_id, code, name) VALUES (9, 'nonf_biography', 'биографии и Мемуары');
	INSERT INTO genre_names (group_id, code, name) VALUES (9, 'nonf_criticism', 'критика');
	INSERT INTO genre_names (group_id, code, name) VALUES (9, 'nonf_publicism', 'публицистика');	
	INSERT INTO genre_names (group_id, code, name) VALUES (9, 'periodic',	'периодика');
	INSERT INTO genre_names (group_id, code, name) VALUES (9, 'other', 'прочая литиратура');
	INSERT INTO genre_names (group_id, code, name) VALUES (9, 'nonfiction', 'прочая документальная литература');
	INSERT INTO genre_names (group_id, code, name) VALUES (9, 'scenarios', 'сценарии');
	INSERT INTO genre_names (group_id, code, name) VALUES (9, 'cinema_theatre', 'кино сценарии и театральные программы');
	
	INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf', 'научная фантастика');
	INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf_action', 'боевая фантастика');
	INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf_cyberpunk', 'киберпанк');
	INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf_detective', 'детективная фантастика');
	INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf_epic', 'эпическая фантастика');
	INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf_fantasy', 'фэнтези');
	INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf_heroic', 'героическая фантастика');
	INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf_history', 'альтернативная история');
	INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf_horror', 'ужасы и мистика');
	INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf_litrpg', 'литературное RPG');	
	INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf_social', 'социально-психологическая фантастика');
	INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf_space', 'космическая фантастика');
	INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf_postapocalyptic', 'постапокалиптическая фантастика');
	INSERT INTO genre_names (group_id, code, name) VALUES (10, 'popadanec', 'попаданцы');
	INSERT INTO genre_names (group_id, code, name) VALUES (10, 'fanfiction', 'фанфики');
	INSERT INTO genre_names (group_id, code, name) VALUES (10, 'city_fantasy', 'городское фэнтези');

	INSERT INTO genre_names (group_id, code, name) VALUES (11, 'short_story', 'рассказы');
	INSERT INTO genre_names (group_id, code, name) VALUES (11, 'prose_classic', 'классическая проза');
	INSERT INTO genre_names (group_id, code, name) VALUES (11, 'prose_contemporary', 'современная проза');
	INSERT INTO genre_names (group_id, code, name) VALUES (11, 'prose_counter', 'контркультура');
	INSERT INTO genre_names (group_id, code, name) VALUES (11, 'prose_history', 'историческая проза');
	INSERT INTO genre_names (group_id, code, name) VALUES (11, 'prose_military', 'военная проза');
	INSERT INTO genre_names (group_id, code, name) VALUES (11, 'prose_rus_classic', 'русская классическая проза');
	INSERT INTO genre_names (group_id, code, name) VALUES (11, 'prose_su_classics', 'советская классическая проза');
	INSERT INTO genre_names (group_id, code, name) VALUES (11, 'russian_contemporary', 'российская современная проза');	
	INSERT INTO genre_names (group_id, code, name) VALUES (11, 'prose', 'прочая проза');

	INSERT INTO genre_names (group_id, code, name) VALUES (12, 'ref_dict', 'словари');
	INSERT INTO genre_names (group_id, code, name) VALUES (12, 'ref_encyc', 'энциклопедии');	
	INSERT INTO genre_names (group_id, code, name) VALUES (12, 'ref_guide', 'руководства');
	INSERT INTO genre_names (group_id, code, name) VALUES (12, 'ref_ref', 'справочники');
	INSERT INTO genre_names (group_id, code, name) VALUES (12, 'reference', 'прочая справочная литература');
	
	INSERT INTO genre_names (group_id, code, name) VALUES (13, 'religion_budda', 'буддизм');
	INSERT INTO genre_names (group_id, code, name) VALUES (13, 'religion_christianity', 'христианство');
	INSERT INTO genre_names (group_id, code, name) VALUES (13, 'religion_esoterics', 'эзотерика');
	INSERT INTO genre_names (group_id, code, name) VALUES (13, 'religion_islam', 'ислам');
	INSERT INTO genre_names (group_id, code, name) VALUES (13, 'religion_orthodoxy', 'православие');
	INSERT INTO genre_names (group_id, code, name) VALUES (13, 'religion_paganism', 'язычество');
	INSERT INTO genre_names (group_id, code, name) VALUES (13, 'religion_rel', 'религия');
	INSERT INTO genre_names (group_id, code, name) VALUES (13, 'psy_personal', 'самопознание');
	INSERT INTO genre_names (group_id, code, name) VALUES (13, 'religion_self', 'самосовершенствование');
	INSERT INTO genre_names (group_id, code, name) VALUES (13, 'religion', 'прочая религионая литература');

	INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_biology', 'биология');
	INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_business', 'деловая литература');
	INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_chem', 'химия');
	INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_cosmos', 'космология');
	INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_culture', 'культурология');
	INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_economic', 'экономика');
	INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_history',  'история');
	INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_juris', 'юриспруденция');
	INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_linguistic', 'языкознание');
	INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_math', 'математика');
	INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_medicine', 'медицина');
	INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_pedagogy', 'педагогика');
	INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_philology', 'филология');
	INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_philosophy', 'философия');
	INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_phys', 'физика');
	INSERT INTO genre_names (group_id, code, name) VALUES (14, 'nonf_military', 'военное дело');
	INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_politics', 'политика');
	INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_popular', 'научно-популярная литература');
	INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_psychology', 'психология');
	INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_religion', 'религиоведение');
	INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_sociology', 'социология');	
	INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_tech', 'технические науки');
	INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_transport', 'транспортные науки');
	INSERT INTO genre_names (group_id, code, name) VALUES (14, 'science', 'прочая научная литература');

	INSERT INTO genre_names (group_id, code, name) VALUES (15, 'humor', 'прочий юмор');
	INSERT INTO genre_names (group_id, code, name) VALUES (15, 'humor_anecdote', 'анекдоты');
	INSERT INTO genre_names (group_id, code, name) VALUES (15, 'humor_fantasy', 'юмористическая фэнтези');
	INSERT INTO genre_names (group_id, code, name) VALUES (15, 'sf_humor', 'юмористическая фантастика');
	INSERT INTO genre_names (group_id, code, name) VALUES (15, 'humor_prose', 'юмористическая проза');	
	INSERT INTO genre_names (group_id, code, name) VALUES (15, 'humor_verse', 'юмористические стихи');	

	INSERT INTO genre_synonyms (code, synonym) VALUES ('litrpg', 'sf_litrpg');
	INSERT INTO genre_synonyms (code, synonym) VALUES ('literature_short', 'short_story');
	INSERT INTO genre_synonyms (code, synonym) VALUES ('sociology_book', 'sci_sociology');
	INSERT INTO genre_synonyms (code, synonym) VALUES ('sci_social_studies', 'sci_sociology');
	INSERT INTO genre_synonyms (code, synonym) VALUES ('popular_business', 'sci_popular');
	INSERT INTO genre_synonyms (code, synonym) VALUES ('sf_etc', 'sf');
	INSERT INTO genre_synonyms (code, synonym) VALUES ('fantastic', 'sf');
	INSERT INTO genre_synonyms (code, synonym) VALUES ('sf_mystic', 'sf_horror');
	INSERT INTO genre_synonyms (code, synonym) VALUES ('vampire_book', 'sf_horror');
	INSERT INTO genre_synonyms (code, synonym) VALUES ('sf_irony', 'sf_humor');
	INSERT INTO genre_synonyms (code, synonym) VALUES ('story', 'short_story');
	INSERT INTO genre_synonyms (code, synonym) VALUES ('russian_fantasy', 'sf_fantasy');
	INSERT INTO genre_synonyms (code, synonym) VALUES ('fantasy', 'sf_fantasy');
	INSERT INTO genre_synonyms (code, synonym) VALUES ('fantasy_fight', 'sf_fantasy');
	INSERT INTO genre_synonyms (code, synonym) VALUES ('historical_fantasy', 'sf_fantasy');
	INSERT INTO genre_synonyms (code, synonym) VALUES ('pedagogy_book', 'sci_pedagogy');
	INSERT INTO genre_synonyms (code, synonym) VALUES ('military_history', 'nonf_military');
	INSERT INTO genre_synonyms (code, synonym) VALUES ('military_special', 'nonf_military');
	INSERT INTO genre_synonyms (code, synonym) VALUES ('military_weapon', 'nonf_military');
	INSERT INTO genre_synonyms (code, synonym) VALUES ('roman', 'prose');

	COMMIT;
	";
