extern crate time;
use time::Timespec;

pub type Id = u32;
pub type Text = String;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Status {
    REGISTRED,
    OPENNED,
    PROCESSED,
    UNAVAILABLE,
    UNREGISTRED,
    FAILED = 6
}

pub enum Registrable {
    Container(Container),
    File(File),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Container {
    pub id: Id,
    pub path: Text,
    pub name: Text,
    pub md5: Text,
    pub status: Status,
    pub changed: Timespec
}
impl Container {
    pub fn new<TEXT: Into<Text>>(path: TEXT, name: TEXT, md5: TEXT) -> Self {
        Container {
            id: 0,
            path: path.into(),
            name: name.into(),
            md5: md5.into(),
            status: Status::REGISTRED,
            changed: time::get_time(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct File {
    pub id: Id,
    pub container: Id,
    pub path: Text,
    pub name: Text,
    pub md5: Text,
    pub status: Status,
    pub changed: Timespec
}
impl File {
    pub fn new<TEXT: Into<Text>>(container: Id, path: TEXT, name: TEXT, md5: TEXT) -> Self {
        File {
            id: 0,
            container: container,
            path: path.into(),
            name: name.into(),
            md5: md5.into(),
            status: Status::REGISTRED,
            changed: time::get_time(),
        }
    }
}
