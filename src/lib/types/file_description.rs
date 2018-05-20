use zip::ZipFile;

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