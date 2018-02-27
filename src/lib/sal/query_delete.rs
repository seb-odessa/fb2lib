
pub const AUTHOR_LINK: &'static str = "DELETE FROM people_links WHERE src_id = ? AND dst_id = ?;";

pub const TITLE_LINK: &'static str = "DELETE FROM titles_links WHERE src_id = ? AND dst_id = ?;";

pub const SEQUENCES_LINK: &'static str = "DELETE FROM sequences_links WHERE src_id = ? AND dst_id = ?;";
