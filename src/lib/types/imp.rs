use zip::ZipFile;
use std::convert::From;


#[derive(Debug, PartialEq, Eq)]
pub struct FileDesc
{
    pub file_name: String,
    pub compression_method: u16,
    pub compressed_size: i64,
    pub original_size: i64,
    pub src32: u32,
    pub offset: i64,
}

impl <'a,'b> From<&'a mut ZipFile<'b>> for FileDesc {
    fn from(zip: &mut ZipFile) -> Self {
        Self {
            file_name: zip.name().to_string(),
            compression_method: zip.compression().to_u16(),
            compressed_size: zip.compressed_size() as i64,
            original_size: zip.size() as i64,
            src32: zip.crc32(),
            offset: zip.offset() as i64,
        }
    }
}

#[derive(Debug)]
pub struct ArchiveSizes {
    pub id: i64,
    pub total_length: usize,
    pub piece_length: usize,
    pub pieces_count: usize,
}
impl ArchiveSizes {
    pub fn new(id: i64, total_length: i64, piece_length: i64, pieces_count: i64) -> Self {
        Self {
            id,
            total_length: total_length as usize,
            piece_length: piece_length as usize,
            pieces_count: pieces_count as usize,
        }
    }
}
