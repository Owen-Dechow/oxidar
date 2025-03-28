use std::fmt::Display;

use crate::errors::OxidarError;

pub enum Version {
    Http0_9,
    Http1_0,
    Http1_1,
    Http2_0,
    Http3_0,
}

impl TryFrom<&str> for Version {
    type Error = OxidarError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "HTTP/0.9" => Ok(Self::Http0_9),
            "HTTP/1" | "HTTP/1.0" => Ok(Self::Http1_0),
            "HTTP/1.1" => Ok(Self::Http1_1),
            "HTTP/2" | "HTTP/2.0" => Ok(Self::Http2_0),
            "HTTP/3" | "HTTP/3.0" => Ok(Self::Http3_0),
            _ => Err(OxidarError::abort_std(format!(
                "Could not parse request version string \"{value}\"."
            ))),
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Version::Http0_9 => "HTTP/0.9",
                Version::Http1_0 => "HTTP/1.0",
                Version::Http1_1 => "HTTP/1.1",
                Version::Http2_0 => "HTTP/2",
                Version::Http3_0 => "HTTP/3",
            }
        )
    }
}

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    TRACE,
    CONNECT,
}

impl TryFrom<&str> for Method {
    type Error = OxidarError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "PATCH" => Ok(Self::PATCH),
            "HEAD" => Ok(Self::HEAD),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "CONNECT" => Ok(Self::CONNECT),
            _ => Err(OxidarError::abort_std(format!(
                "Could not parse request method string \"{value}\"."
            ))),
        }
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::GET => write!(f, "GET"),
            Method::POST => write!(f, "POST"),
            Method::PUT => write!(f, "PUT"),
            Method::DELETE => write!(f, "DELETE"),
            Method::PATCH => write!(f, "PATCH"),
            Method::HEAD => write!(f, "HEAD"),
            Method::OPTIONS => write!(f, "OPTIONS"),
            Method::TRACE => write!(f, "TRACE"),
            Method::CONNECT => write!(f, "CONNECT"),
        }
    }
}
