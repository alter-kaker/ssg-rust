use std::error::Error;

use std::fmt::Display;

#[derive(Debug)]
pub enum GeneratorError {
    CollectionAlreadyRegistered(String),
    DataObjectNotInitialized,
    DataObjectAlreadyInitialized,
    HeadAlreadySet,
    LibraryError(LibraryErrorKind, Box<dyn Error>),
}

#[derive(Debug)]
pub enum LibraryErrorKind {
    TemplateError,
    ApiError,
    IOError,
}

impl Display for GeneratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeneratorError::CollectionAlreadyRegistered(key) => {
                write!(f, "{:?}:, Collection {} Already Registered", self, key)
            }
            GeneratorError::LibraryError(kind, source) => {
                write!(f, "{:?}: {:?}: {}", self, kind, source)
            }
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Error for GeneratorError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GeneratorError::LibraryError(_kind, source) => Some(source.as_ref()),
            _ => None,
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl From<minijinja::Error> for GeneratorError {
    fn from(value: minijinja::Error) -> Self {
        GeneratorError::LibraryError(LibraryErrorKind::TemplateError, Box::new(value))
    }
}

impl From<ureq::Error> for GeneratorError {
    fn from(value: ureq::Error) -> Self {
        GeneratorError::LibraryError(LibraryErrorKind::ApiError, Box::new(value))
    }
}

impl From<std::io::Error> for GeneratorError {
    fn from(value: std::io::Error) -> Self {
        GeneratorError::LibraryError(LibraryErrorKind::TemplateError, Box::new(value))
    }
}
