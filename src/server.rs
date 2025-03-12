pub enum Response {
    Json(String),
}

pub enum Method {
    GET,
    POST,
}

pub struct Request {
    pub method: Method,
}

pub struct Path(pub String, pub fn(&App, &Request) -> Response);
impl Path {
    pub fn p(path: &str, view: fn(&App, &Request) -> Response) -> Path {
        Path(path.to_string(), view)
    }
}

pub struct App {
    urls: Vec<Path>,
}

impl App {
    pub fn new(urls: Vec<Path>) -> App {
        App { urls }
    }
}

pub struct Oxidar {}
impl Oxidar {}
