//! Error types that can be emitted from this library
extern crate std;

use std::convert;
use std::error;
use std::fmt;
use std::io;
use fb2parser::XmlParseError;
use zip::result::ZipError;

/// Generic result type with Fb2Error as its error variant
pub type Fb2Result<T> = Result<T, Fb2Error>;

/// Error type for Zip
#[derive(Debug)]
pub enum Fb2Error {
    /// An Error caused by I/O
    Io(io::Error),

    /// This file is probably not a zip archive
    InvalidArchive(&'static str),

    /// This archive is not supported
    UnsupportedArchive(&'static str),

    /// This file does not contains description tag
    UnableToLoadFb2Header,

    /// This file has unknown character symbols
    UnableToMakeUtf8,

    /// Unable to deserialize xml
    UnableDeserializeXML,

    /// The requested file could not be found in the archive
    FileNotFound,

    /// This SubCommand was not found
    UnsupportedSubCommand,
}

impl Fb2Error {
    fn detail(&self) -> ::std::borrow::Cow<str> {
        use std::error::Error;

        match *self {
            Fb2Error::Io(ref io_err) => {
                ("Io Error: ".to_string() + (io_err as &error::Error).description()).into()
            }
            Fb2Error::InvalidArchive(msg) |
            Fb2Error::UnsupportedArchive(msg) => {
                (self.description().to_string() + ": " + msg).into()
            }
            Fb2Error::FileNotFound |
            Fb2Error::UnableToMakeUtf8 |
            Fb2Error::UnableDeserializeXML |
            Fb2Error::UnableToLoadFb2Header |
            Fb2Error::UnsupportedSubCommand => self.description().into(),
        }
    }
}

impl convert::From<io::Error> for Fb2Error {
    fn from(err: io::Error) -> Fb2Error {
        Fb2Error::Io(err)
    }
}

impl convert::From<std::str::Utf8Error> for Fb2Error {
    fn from(_: std::str::Utf8Error) -> Fb2Error {
        Fb2Error::UnableToMakeUtf8
    }
}

impl convert::From<std::string::FromUtf8Error> for Fb2Error {
    fn from(_: std::string::FromUtf8Error) -> Fb2Error {
        Fb2Error::UnableToMakeUtf8
    }
}

impl convert::From<XmlParseError> for Fb2Error {
    fn from(_: XmlParseError) -> Fb2Error {
        Fb2Error::UnableDeserializeXML
    }
}

impl convert::From<ZipError> for Fb2Error {
    fn from(err: ZipError) -> Fb2Error {
        match err {
            ZipError::Io(io_err) => Fb2Error::Io(io_err),
            ZipError::InvalidArchive(msg) => Fb2Error::InvalidArchive(msg),
            ZipError::UnsupportedArchive(msg) => Fb2Error::UnsupportedArchive(msg),
            ZipError::FileNotFound => Fb2Error::FileNotFound,
        }
    }
}

impl convert::From<Fb2Error> for io::Error {
    fn from(err: Fb2Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err)
    }
}

impl fmt::Display for Fb2Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_str(&*self.detail())
    }
}

impl error::Error for Fb2Error {
    fn description(&self) -> &str {
        match *self {
            Fb2Error::Io(ref io_err) => (io_err as &error::Error).description(),
            Fb2Error::InvalidArchive(..) => "Invalid Zip archive",
            Fb2Error::UnsupportedArchive(..) => "Unsupported Zip archive",
            Fb2Error::UnableToMakeUtf8 => "Unable to convert content into UTF8",
            Fb2Error::UnableDeserializeXML => "Unable to deserialize from XML",
            Fb2Error::UnableToLoadFb2Header => "Unable to load FB2 description data",
            Fb2Error::UnsupportedSubCommand => "Unsupported sub command",
            Fb2Error::FileNotFound => "Specified file not found in archive",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Fb2Error::Io(ref io_err) => Some(io_err as &error::Error),
            _ => None,
        }
    }
}
