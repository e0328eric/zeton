use thiserror::Error;

#[derive(Debug, Error)]
pub enum ZetonError {
    #[error("{0}")]
    IOErr(#[from] std::io::Error),
    #[error("{0}")]
    NulErr(#[from] std::ffi::NulError),
    #[error("{0}")]
    FromBytesWithNulErr(#[from] std::ffi::FromBytesWithNulError),
    #[error("{0}")]
    TomlParseErr(#[from] toml::de::Error),
    #[error("")]
    CannotLoadFontFromName,
    #[error("")]
    CannotLoadFontFromPattern,
    #[error("no font specified")]
    FontNotSpecified,
    #[error("cannot get the font pattern")]
    CannotGetFontPattern,
}

pub type Result<T> = std::result::Result<T, ZetonError>;
