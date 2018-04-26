use zip::ZipFile;
use std::convert::From;

/**************************************************************************************************/
#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub struct FileDesc
{
    pub archive_id: i64,
    pub file_name: String,
    pub compression_method: u16,
    pub compressed_size: i64,
    pub original_size: i64,
    pub src32: u32,
    pub offset: i64,
}

impl <'a,'b> From<(i64, &'a mut ZipFile<'b>)> for FileDesc {
    fn from(arg: (i64, &mut ZipFile)) -> Self {
        Self {
            archive_id: arg.0,
            file_name: arg.1.name().to_string(),
            compression_method: arg.1.compression().to_u16(),
            compressed_size: arg.1.compressed_size() as i64,
            original_size: arg.1.size() as i64,
            src32: arg.1.crc32(),
            offset: arg.1.offset() as i64,
        }
    }
}

impl From<(i64, String, u16, i64, i64, u32, i64)> for FileDesc {
    fn from(arg: (i64, String, u16, i64, i64, u32, i64)) -> Self {
        Self {
            archive_id: arg.0,
            file_name: arg.1,
            compression_method: arg.2,
            compressed_size: arg.3,
            original_size: arg.4,
            src32: arg.5,
            offset: arg.6,
        }
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
