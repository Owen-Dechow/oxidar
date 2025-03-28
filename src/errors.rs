use crate::server::{
    http::Version,
    response::{Response, ResponseContent},
};

#[derive(Debug)]
pub enum OxidarError {
    Fatal(Error),
    Abortion(Error),
    Normal(Error),
}

#[derive(Debug)]
pub enum Error {
    Untyped(String),
    Io(std::io::Error),
    Http404(Option<String>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Untyped(err) => write!(f, "(Untyped) {}", err),
            Error::Io(err) => write!(f, "(IO) {}", err),
            Error::Http404(err) => write!(
                f,
                "(404) {}",
                match err {
                    Some(msg) => msg,
                    None => "Resource Not Found",
                }
            ),
        }
    }
}

impl Error {
    pub fn status_str(&self) -> &'static str {
        match self {
            Error::Untyped(_) => "500 Server Error",
            Error::Io(_) => "500 Server Error",
            Error::Http404(_) => "404 Resource Not Found",
        }
    }
}

impl std::fmt::Display for OxidarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OxidarError::Normal(err) => write!(f, "Normal Error {}", err),
            OxidarError::Fatal(msg) => write!(f, "Fatal Error {}", msg),
            OxidarError::Abortion(msg) => write!(f, "Abortion Error {}", msg),
        }
    }
}

impl OxidarError {
    pub fn fio<T>(r: Result<T, std::io::Error>) -> Result<T, OxidarError> {
        match r {
            Ok(r) => Ok(r),
            Err(err) => Err(OxidarError::Fatal(Error::Io(err))),
        }
    }

    pub fn aio<T>(r: Result<T, std::io::Error>) -> Result<T, OxidarError> {
        match r {
            Ok(r) => Ok(r),
            Err(err) => Err(OxidarError::Fatal(Error::Io(err))),
        }
    }

    pub fn abort_std(msg: String) -> Self {
        OxidarError::Fatal(Error::Untyped(msg))
    }

    pub fn to_response(&self) -> Option<Response> {
        let err = match self {
            OxidarError::Fatal(_) => return None,
            OxidarError::Abortion(_) => return None,
            OxidarError::Normal(ref err) => err,
        };

        let status = err.status_str();
        let msg = err.to_string();

        return Some(Response {
            version: Version::Http1_1,
            status,
            content: ResponseContent::Html(format!("<h1 style='text-align: center'>{msg}</h1>")),
        });
    }

    pub fn http_404(msg: Option<String>) -> OxidarError {
        OxidarError::Normal(Error::Http404(msg))
    }
}

impl std::error::Error for OxidarError {}
