use super::http::{Method, Version};
use std::collections::HashMap;

pub struct Request {
    pub method: Method,
    pub uri: String,
    pub version: Version,
    pub headers: HashMap<String, String>,
}
