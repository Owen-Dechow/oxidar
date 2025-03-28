use crate::errors::OxidarError;

use super::{
    http::Version,
    request::Request,
    response::{Response, ResponseContent},
    Oxidar,
};

pub struct AppReg(pub String, pub App);
impl AppReg {
    pub fn p(path: &str, app: App) -> AppReg {
        let mut path = path.to_string();

        if !path.starts_with('/') {
            path.insert(0, '/');
        }

        if !path.ends_with("/") {
            path.push('/');
        }

        AppReg(path.to_string(), app)
    }
}

pub struct ViewReg(pub String, pub fn(&App, &Request) -> ResponseContent);
impl ViewReg {
    pub fn p(path: &str, view: fn(&App, &Request) -> ResponseContent) -> ViewReg {
        ViewReg(path.to_string(), view)
    }
}

pub struct App {
    urls: Vec<ViewReg>,
}

impl App {
    pub fn new(urls: Vec<ViewReg>) -> App {
        App { urls }
    }

    pub(crate) fn respond(
        &self,
        oxidar: &Oxidar,
        request: Request,
    ) -> Result<Response, OxidarError> {
        Ok(Response {
            version: Version::Http1_1,
            status: "200 OK",
            content: ResponseContent::Html(format!("WE HAVE SOME SUCCESS")),
        })
    }
}
