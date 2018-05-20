
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BlobDescription {
    pub size: i64,
    pub sha1: String,
    pub data: Option<Vec<u8>>,
}
impl BlobDescription {
    pub fn from(blob: Vec<u8>, sha1: String) -> Self {
        Self {
            size: blob.len() as i64,
            sha1: sha1,
            data: Some(blob),
        }
    }
}