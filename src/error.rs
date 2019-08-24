use failure::Fail;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Invalid packet")]
    InvalidPacket,

    #[fail(display = "Tftp error: {:?}", _0)]
    Tftp(crate::TftpError),

    #[fail(display = "IO Error: {}", _0)]
    Io(std::io::Error),

    #[fail(display = "Failed to bind socket: {}", _0)]
    Bind(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error::Io(error)
    }
}

impl<'a> From<nom::Err<(&'a [u8], nom::error::ErrorKind)>> for Error {
    fn from(_error: nom::Err<(&'a [u8], nom::error::ErrorKind)>) -> Error {
        Error::InvalidPacket
    }
}

impl From<crate::TftpError> for Error {
    fn from(error: crate::TftpError) -> Error {
        Error::Tftp(error)
    }
}
