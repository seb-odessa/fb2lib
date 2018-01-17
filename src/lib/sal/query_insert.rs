
pub const ARCHIVE: &'static str = "
	INSERT OR IGNORE
	INTO archives (name, created, hash, total_length, piece_length, pieces_count)
	VALUES (?, ?, ?, ?, ?, ?)
	";

pub const PIECE: &'static str = "
	INSERT INTO pieces (archive_id, piece_idx, hash) VALUES (?, ?, ?)";

pub const LANGUAGES: &'static str = "
    INSERT OR IGNORE INTO languages (id, text) VALUES (0, ?);";