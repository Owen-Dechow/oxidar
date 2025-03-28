use crate::errors::{Error, OxidarError};

use super::http::Version;

pub enum ResponseContent {
    Json(String),
    Html(String),
}

impl ResponseContent {
    pub(crate) fn get_body(&self) -> String {
        match self {
            ResponseContent::Json(json) => json.to_string(),
            ResponseContent::Html(html) => html.to_string(),
        }
    }
}

pub struct Response {
    pub version: Version,
    pub status: &'static str,
    pub content: ResponseContent,
}

impl Response {
    pub(crate) fn build_response(&self) -> String {
        let response = format!(
            "{} {}\r\n\r\n{}",
            self.version,
            self.status,
            self.content.get_body()
        );
        return response;
    }
}
