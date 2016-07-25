
#[derive(Debug)]
pub enum Error {
    UnknownError,
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Error::UnknownError => write!(f, "Unbekannter Fehler"),
        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnknownError => "Unbekannter Fehler",
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::UnknownError => None,
        }
    }
}
