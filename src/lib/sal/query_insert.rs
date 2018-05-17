
pub const ARCHIVE: &'static str = "
	INSERT OR IGNORE
	INTO archives (name, created, hash, total_length, piece_length, pieces_count)
	VALUES (?, ?, ?, ?, ?, ?)
	";

pub const PIECE: &'static str = "
	INSERT INTO pieces (archive_id, piece_idx, hash) VALUES (?, ?, ?)";

pub const LANGUAGE: &'static str = "
    INSERT OR IGNORE INTO languages (name) VALUES (?);";

pub const DISABLE_LANGUAGE: &'static str = "
 	INSERT INTO filters_def (filter_id, filtered_id)
	SELECT filters.id, languages.id
	FROM filters, languages WHERE filters.name = 'lang' AND languages.name = ?;";

pub const ENABLE_LANGUAGE: &'static str = "
	DELETE from filters_def WHERE id = (
 	SELECT filters_def.id FROM filters_def
 		JOIN filters ON filters_def.filter_id = filters.id
 		JOIN languages ON filters_def.filtered_id = languages.id
 		WHERE filters.name = 'lang' AND languages.name = ?
 	)";

pub const DISABLE_GENRE: &'static str = "
 	INSERT INTO filters_def (filter_id, filtered_id)
	SELECT filters.id, genre_names.id
	FROM filters, genre_names WHERE filters.name = 'genre' AND genre_names.name = ?;";

pub const DISABLE_GENRE_GROUP: &'static str = "
 	INSERT INTO filters_def (filter_id, filtered_id)
	SELECT filters.id, genre_names.id FROM genre_groups, filters, genre_names
	WHERE genre_names.group_id = genre_groups.id AND filters.name = 'genre' AND genre_groups.name = ?;";

pub const ENABLE_GENRE: &'static str = "
	DELETE from filters_def WHERE id = (
 		SELECT filters_def.id FROM filters_def
 		JOIN filters ON filters_def.filter_id = filters.id
 		JOIN genre_names ON filters_def.filtered_id = genre_names.id
 		WHERE filters.name = 'genre' AND genre_names.name = ?
 	)";

pub const ENABLE_GENRE_GROUP: &'static str = "
	DELETE from filters_def WHERE id IN (
		SELECT filters_def.id FROM genre_groups, genre_names, filters_def, filters
		WHERE genre_names.group_id = genre_groups.id AND genre_names.id = filters_def.filtered_id
		AND filters.id = filters_def.filter_id AND filters.name = 'genre' AND genre_groups.name = ?
 	)";

pub const PEOPLE: &'static str = "
	INSERT INTO people (first_name, middle_name, last_name, nickname) VALUES (?, ?, ?, ?)";

pub const PROGRESS: &'static str = "
	INSERT INTO progress (archive_id, task_id, status_id) VALUES (?, ?, ?)";

pub const TITLES: &'static str = "INSERT INTO titles (title) VALUES (?)";

pub const SEQUENCES: &'static str = "INSERT INTO sequences (sequence) VALUES (?)";

pub const NAMES: &'static str = "INSERT INTO names (name) VALUES (?)";

pub const AUTHOR_LINK: &'static str = "INSERT INTO people_links VALUES (NULL, ?, ?, 0);";

pub const TITLE_LINK: &'static str = "INSERT INTO titles_links VALUES (NULL, ?, ?, 0);";

pub const SEQUENCES_LINK: &'static str = "INSERT INTO sequences_links VALUES (NULL, ?, ?, 0);";

pub const BOOK: &'static str = "
    INSERT INTO books VALUES (
        NULL,
        :archive_id,
        :file_name,
        :compression_method,
        :compressed_size,
        :original_size,
        :src32,
        :offset,
        :description
    );
";
