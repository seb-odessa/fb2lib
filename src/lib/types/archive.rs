#[derive(Debug)]
pub struct Archive {
    pub id: i64,
    pub name: String,
}
impl Archive {
    pub fn new(id: i64, name: String) -> Self {
        Self{id, name}
    }
}

/**************************************************************************************************/
#[derive(Debug)]
pub struct Sizes {
    pub id: i64,
    pub total_length: usize,
    pub piece_length: usize,
    pub pieces_count: usize,
}
impl Sizes {
    pub fn new(id: i64, total_length: i64, piece_length: i64, pieces_count: i64) -> Self {
        Self {
            id,
            total_length: total_length as usize,
            piece_length: piece_length as usize,
            pieces_count: pieces_count as usize,
        }
    }
}
/**************************************************************************************************/
