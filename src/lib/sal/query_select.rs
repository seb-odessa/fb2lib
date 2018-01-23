pub const ID_BY_HASH: &'static str = "SELECT id FROM archives WHERE hash = ?1";

pub const INDEX_AND_HASH_BY_ARCH_ID: &'static str = "SELECT piece_idx, hash FROM pieces WHERE archive_id = ?1";

pub const ARCH_SIZES_BY_NAME: &'static str = "SELECT id, total_length, piece_length, pieces_count FROM archives WHERE name = ?1";

pub const HASH_BY_ARCH_ID_AND_INDEX: &'static str = "SELECT hash FROM pieces WHERE archive_id = ?1 AND piece_idx = ?2";

pub const LANGUAGES_DISABLED: &'static str = "SELECT name FROM languages_disabled ORDER BY name";

pub const LANGUAGES_ENABLED: &'static str = "SELECT name FROM languages_enabled ORDER BY name";





