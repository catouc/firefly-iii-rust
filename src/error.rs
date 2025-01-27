pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    HTTPErr{
        status: u16,
        response_msg: String,
    },
    IOErr(String),
}

