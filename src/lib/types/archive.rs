use zip::ZipFile;
use std::error::Error;
use std::convert::From;
use fb2parser::FictionBook;
use bincode::deserialize;

/**************************************************************************************************/
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct FileDescription {
    pub name: String,
    pub compression_method: u16,
    pub compressed_size: i64,
    pub original_size: i64,
    pub src32: u32,
    pub offset: i64,
}
impl <'a,'b> From<&'a mut ZipFile<'b>> for FileDescription {
    fn from(zip:  &mut ZipFile) -> Self {
        Self {
            name: String::from(zip.name()),
            compression_method: zip.compression().to_u16(),
            compressed_size: zip.compressed_size() as i64,
            original_size: zip.size() as i64,
            src32: zip.crc32(),
            offset: zip.offset() as i64,
        }
    }
}
/**************************************************************************************************/
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
/**************************************************************************************************/
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BookDescription
{
    pub arch: i64,
    pub file: FileDescription,
    pub blob: BlobDescription,
}
impl From<(i64, FileDescription, BlobDescription)> for BookDescription {
    fn from(arg: (i64, FileDescription, BlobDescription)) -> Self {
        Self {
            arch: arg.0,
            file: arg.1,
            blob: arg.2,
        }
    }
}

//impl From<(i64, String, u16, i64, i64, u32, i64, Option<Vec<u8>>)> for BookDescription {
//    fn from(arg: (i64, String, u16, i64, i64, u32, i64, Option<Vec<u8>>)) -> Self {
//        Self {
//            archive_id: arg.0,
//            file_name: arg.1,
//            compression_method: arg.2,
//            compressed_size: arg.3,
//            original_size: arg.4,
//            src32: arg.5,
//            offset: arg.6,
////            description: arg.7.and_then(|blob| deserialize(&blob).ok()),
//            blob_size: 0,
//            blob_data: None,
//            blob_sha1: String::new(),
//
//        }
//    }
//}

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
