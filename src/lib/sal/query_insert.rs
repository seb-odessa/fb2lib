
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

