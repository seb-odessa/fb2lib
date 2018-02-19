pub const ID_BY_HASH: &'static str = "SELECT id FROM archives WHERE hash = ?1";

pub const INDEX_AND_HASH_BY_ARCH_ID: &'static str = "SELECT piece_idx, hash FROM pieces WHERE archive_id = ?1";

pub const ARCH_SIZES_BY_NAME: &'static str = "SELECT id, total_length, piece_length, pieces_count FROM archives WHERE name = ?1";

pub const HASH_BY_ARCH_ID_AND_INDEX: &'static str = "SELECT hash FROM pieces WHERE archive_id = ?1 AND piece_idx = ?2";

pub const LANGUAGES_DISABLED: &'static str = "SELECT name FROM languages_disabled ORDER BY name";

pub const LANGUAGES_ENABLED: &'static str = "SELECT name FROM languages_enabled ORDER BY name";

pub const LANGUAGES: &'static str = "SELECT name FROM languages";

pub const GENRE_CODES: &'static str = "SELECT code FROM genres";

pub const GENRES_DISABLED: &'static str = "SELECT group_name, genre_name FROM genres_disabled ORDER BY group_name, genre_name";

pub const GENRES_ENABLED: &'static str = "SELECT group_name, genre_name FROM genres_enabled ORDER BY group_name, genre_name";

pub const GENRES_GROUPS_DISABLED: &'static str = "SELECT DISTINCT group_name FROM genres_disabled ORDER BY group_name, genre_name";

pub const GENRES_GROUPS_ENABLED: &'static str = "SELECT DISTINCT group_name FROM genres_enabled ORDER BY group_name, genre_name";

pub const GENRE_CODES_DISABLED: &'static str = "
    SELECT genres.code FROM genres, filters_def WHERE genres.id = filtered_id AND filter_id = (SELECT id FROM filters WHERE name = 'genre')";

pub const GENRES_CODES_AND_GROUPS: &'static str = "SELECT code, type FROM genres;";

pub const PROGRESS_STATUS: &'static str = "
	SELECT status_id FROM progress LEFT JOIN archives ON progress.archive_id = archives.id WHERE archives.name = ? AND progress.task_id = ?;";

pub const ARCHIVE_ID_BY_NAME: &'static str = "SELECT id FROM archives WHERE name = ?1";

pub const PEOPLE: &'static str = "SELECT first_name, middle_name, last_name, nickname FROM people";

pub const TITLES: &'static str = "SELECT title FROM titles";

pub const SEQUENCES: &'static str = "SELECT sequence FROM sequences";


