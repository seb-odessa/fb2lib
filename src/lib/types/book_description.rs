
use types::FileDescription;
use types::BlobDescription;

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
