use serde_derive::Deserialize;
use serde_json::Value;

/// An error encountered when communicating with the Kuda API.
#[derive(Debug)]
pub enum Error {
    /// An error reported by Kuda in the response body.
    Kuda(RequestError),
    /// An http or networking error communicating with the Kuda server.
    Http(HttpError),
    /// An error reading the response body.
    Io(std::io::Error),
    /// An error serializing a request before it is sent to kuda.
    Serialize(Box<dyn std::error::Error + Send>),
    /// An error deserializing a response received from kuda.
    Deserialize(Box<dyn std::error::Error + Send>),
    /// Indicates an operation not supported (yet?) by this library.
    Unsupported(&'static str),
    /// An invariant has been violated. Either a bug in this library or Kuda
    Unexpected(&'static str),
}

impl Error {
    #[allow(dead_code)]
    pub(crate) fn timeout() -> Error {
        Error::Http(HttpError::Timeout)
    }

    pub(crate) fn serialize<T>(err: T) -> Error
    where
        T: std::error::Error + Send + 'static,
    {
        Error::Serialize(Box::new(err))
    }

    pub(crate) fn deserialize<T>(err: T) -> Error
    where
        T: std::error::Error + Send + 'static,
    {
        Error::Deserialize(Box::new(err))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[allow(deprecated)]
        f.write_str(std::error::Error::description(self))?;
        match *self {
            Error::Kuda(ref err) => write!(f, ": {}", err),
            Error::Http(ref err) => write!(f, ": {}", err),
            Error::Io(ref err) => write!(f, ": {}", err),
            Error::Serialize(ref err) => write!(f, ": {}", err),
            Error::Deserialize(ref err) => write!(f, ": {}", err),
            Error::Unsupported(msg) => write!(f, "{}", msg),
            Error::Unexpected(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Kuda(_) => "error reported by kuda",
            Error::Http(_) => "error communicating with kuda",
            Error::Io(_) => "error reading response from kuda",
            Error::Serialize(_) => "error serializing a request",
            Error::Deserialize(_) => "error deserializing a response",
            Error::Unsupported(_) => "an unsupported operation was attempted",
            Error::Unexpected(_) => "an unexpected error has occurred",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            Error::Kuda(ref err) => Some(err),
            Error::Http(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
            Error::Serialize(ref err) => Some(&**err),
            Error::Deserialize(ref err) => Some(&**err),
            Error::Unsupported(_) => None,
            Error::Unexpected(_) => None,
        }
    }
}

impl From<RequestError> for Error {
    fn from(err: RequestError) -> Error {
        Error::Kuda(err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Http(HttpError::Stream(err))
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

#[derive(Debug)]
pub enum HttpError {
    /// An error handling HTTP streams.
    Stream(hyper::Error),
    /// The request timed out.
    Timeout,
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[allow(deprecated)]
        match *self {
            HttpError::Stream(ref err) => err.fmt(f),
            HttpError::Timeout => f.write_str(std::error::Error::description(self)),
        }
    }
}

impl std::error::Error for HttpError {
    fn description(&self) -> &str {
        #[allow(deprecated)]
        match *self {
            HttpError::Stream(ref err) => err.description(),
            HttpError::Timeout => "request timed out",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            HttpError::Stream(ref err) => Some(err),
            HttpError::Timeout => None,
        }
    }
}

/// An error reported by kuda in a request's response.
///
/// For more details see https://developers.kuda.com/#block7Head
#[derive(Debug, Default, Deserialize)]
pub struct RequestError {
    /// The HTTP status in the response.
    #[serde(skip_deserializing)]
    pub http_status: u16,

    /// A human-readable message providing more details about the error.
    #[serde(default)]
    pub message: String,

    /// Some types of errors also include a details array:
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.http_status)?;
        if let Some(ref data) = self.data {
            write!(f, "{:?}", data)?;
        }
        Ok(())
    }
}

impl std::error::Error for RequestError {
    fn description(&self) -> &str {
        self.message.as_ref()
    }
}
