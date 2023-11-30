use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    FailToCreatePool(String),
    Surreal(#[serde_as(as = "DisplayFromStr")] surrealdb::Error),
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
impl std::convert::From<surrealdb::Error> for Error {
    fn from(value: surrealdb::Error) -> Self {
        todo!()
    }
}
// endregion: --- Error Boilerplate
