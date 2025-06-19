pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum AddrError {
    #[error(transparent)]
    Parse(#[from] std::net::AddrParseError),

    #[error(transparent)]
    Unexpected(#[from] std::string::FromUtf8Error),
}

#[derive(Debug, thiserror::Error)]
pub enum UriError {
    #[error(transparent)]
    HttpParse(#[from] httparse::Error),

    #[error("method {0}")]
    Method(String),

    #[error("path {0}")]
    Path(String),

    #[error("version {0}")]
    Version(u8),

    #[error("header {0}")]
    Header(String),

    #[error("address {0}")]
    Addr(AddrError),
}

impl<'a> From<&httparse::Header<'a>> for UriError {
    fn from(value: &httparse::Header<'a>) -> Self {
        Self::Header(format!(
            "{{ {}: {} }}",
            value.name,
            String::from_utf8_lossy(value.value)
        ))
    }
}

impl<T: Into<AddrError>> From<T> for UriError {
    fn from(value: T) -> Self {
        Self::Addr(value.into())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Invalid {
    #[error("Invalid {0}")]
    Uri(UriError),

    #[error("Invalid http format")]
    Format,
}

impl<T: Into<UriError>> From<T> for Invalid {
    fn from(value: T) -> Self {
        Self::Uri(value.into())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GifError {
    #[error(transparent)]
    Gif(#[from] gif::DecodingError),

    #[error(transparent)]
    Image(#[from] image::ImageError),

    #[error("GIF (missing frame rate)")]
    Delay,

    #[error("EOF")]
    Eof,
}

impl From<gif::DecodingError> for Error {
    fn from(value: gif::DecodingError) -> Self {
        Self::Gif(GifError::Gif(value))
    }
}

impl From<image::ImageError> for Error {
    fn from(value: image::ImageError) -> Self {
        Self::Gif(GifError::Image(value))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Parse(Invalid),

    #[error(transparent)]
    Gif(#[from] GifError),

    #[error(transparent)]
    Json(#[from] bincode::Error),

    #[error(transparent)]
    Cli(#[from] indicatif::style::TemplateError),

    #[error("The server is empty. Entering idle mode.")]
    Empty,

    #[error("An unexpected (poison or thread) error has occurred")]
    Sync,
}

impl<T> From<std::sync::PoisonError<std::sync::RwLockReadGuard<'_, T>>> for Error {
    fn from(_: std::sync::PoisonError<std::sync::RwLockReadGuard<'_, T>>) -> Self {
        Self::Sync
    }
}

impl<T> From<std::sync::PoisonError<std::sync::RwLockWriteGuard<'_, T>>> for Error {
    fn from(_: std::sync::PoisonError<std::sync::RwLockWriteGuard<'_, T>>) -> Self {
        Self::Sync
    }
}

impl<T> From<std::sync::PoisonError<std::sync::MutexGuard<'_, T>>> for Error {
    fn from(_: std::sync::PoisonError<std::sync::MutexGuard<'_, T>>) -> Self {
        Self::Sync
    }
}

impl<T: Into<Invalid>> From<T> for Error {
    fn from(value: T) -> Self {
        Self::Parse(value.into())
    }
}
