pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    HTTPErr{
        status: u32,
        response_msg: String,
    },
    IOErr,
}

impl From<ureq::Error> for Error {
    fn from(err: ureq::Error) -> Self {
        err.into()
    }
}

impl From<&ureq::Error> for Error {
    fn from(err: &ureq::Error) -> Self {
        err.into()
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        err.into()
    }
}

impl From<u16> for Error {
    fn from(err: u16) -> Self {
        err.into()
    }
}
