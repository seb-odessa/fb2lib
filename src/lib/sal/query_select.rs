pub const ID_BY_HASH: &'static str = "SELECT id FROM archives WHERE hash = ?1";

pub const INDEX_AND_HASH_BY_ARCH_ID: &'static str = "SELECT piece_idx, hash FROM pieces WHERE archive_id = ?1";

pub const ARCH_SIZES_BY_NAME: &'static str = "SELECT id, total_length, piece_length, pieces_count FROM archives WHERE name = ?1";

pub const HASH_BY_ARCH_ID_AND_INDEX: &'static str = "SELECT hash FROM pieces WHERE archive_id = ?1 AND piece_idx = ?2";

pub const LANGUAGES_DISABLED: &'static str = "SELECT name FROM languages_disabled ORDER BY name";

pub const LANGUAGES_ENABLED: &'static str = "SELECT name FROM languages_enabled ORDER BY name";

pub const GENRE_NAME: &'static str = "SELECT id, name FROM genres WHERE code = :1";

pub const GENRES_DISABLED: &'static str = "SELECT group_name, genre_name FROM genres_disabled ORDER BY group_name, genre_name";

pub const GENRES_ENABLED: &'static str = "SELECT group_name, genre_name FROM genres_enabled ORDER BY group_name, genre_name";

pub const GENRES_GROUPS_DISABLED: &'static str = "SELECT DISTINCT group_name FROM genres_disabled ORDER BY group_name, genre_name";

pub const GENRES_GROUPS_ENABLED: &'static str = "SELECT DISTINCT group_name FROM genres_enabled ORDER BY group_name, genre_name";

pub const GENRE_CODES_DISABLED: &'static str = "
    SELECT genres.code FROM genres, filters_def WHERE genres.id = filtered_id AND filter_id = (SELECT id FROM filters WHERE name = 'genre')";

pub const GENRES_CODES_AND_GROUPS: &'static str = "SELECT code, type FROM genres;";

