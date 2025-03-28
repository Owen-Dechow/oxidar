pub mod app;
pub mod http;
pub mod request;
pub mod response;
mod thread_pool;

use crate::errors::OxidarError;
use app::AppReg;
use http::{Method, Version};
use request::Request;
use response::Response;
use std::fmt::Display;
use std::path::PathBuf;
use std::{
    collections::HashMap,
    io::{prelude::BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    str,
    sync::Arc,
};
use thread_pool::ThreadPool;

pub enum LogMethod {
    Info,
    Warning,
    Error,
}

pub enum LogStyle {
    Terminal,
    File(PathBuf),
    TerminalFile(PathBuf),
}

pub struct Oxidar {
    apps: Vec<AppReg>,
    socket_addr: &'static str,
    threads: usize,
    log_method: LogStyle,
    debug: bool,
}

impl Oxidar {
    pub fn new(
        apps: Vec<AppReg>,
        socket_addr: &'static str,
        threads: usize,
        log_method: LogStyle,
        debug: bool,
    ) -> Self {
        let oxidar = Self {
            apps,
            threads,
            socket_addr,
            log_method,
            debug,
        };

        oxidar.log(format!("Oxidar app created."));
        return oxidar;
    }
    pub fn run(self) -> Result<(), OxidarError> {
        let oxidar = Arc::new(self);
        let listener = OxidarError::fio(TcpListener::bind(oxidar.socket_addr))?;
        let pool = ThreadPool::new(oxidar.clone(), oxidar.threads);
        oxidar.log(format!(
            "Started server. Listening on {}",
            oxidar.socket_addr
        ));

        for stream in listener.incoming() {
            let stream = OxidarError::fio(stream)?;
            let oxidar = oxidar.clone();
            pool.execute(move || {
                if let Err(err) = oxidar.handel_connection(stream) {
                    match err {
                        OxidarError::Fatal(error) => {
                            todo!()
                        }
                        OxidarError::Abortion(error) => {
                            oxidar.loge(format!("Oxidar is aborting the request: {error}"))
                        }
                        OxidarError::Normal(error) => oxidar.loge(error),
                    };
                }
            })
        }

        return Ok(());
    }

    fn handel_connection(&self, stream: TcpStream) -> Result<(), OxidarError> {
        let buffer = BufReader::new(&stream);
        let mut lines = buffer.lines();

        let mut request: Option<Request> = None;

        while let Some(line) = lines.next() {
            let line = OxidarError::fio(line)?;
            match request {
                Some(ref mut request) => {
                    if line.is_empty() {
                        break;
                    }

                    let (key, val) = line.split_once(": ").ok_or(OxidarError::abort_std(
                        format!("Malformed Request: Header \"{line}\" could not be parsed."),
                    ))?;

                    request.headers.insert(key.to_owned(), val.to_owned());
                }
                None => {
                    let mut split = line.split(" ");

                    let method = split.next().ok_or(OxidarError::abort_std(format!(
                        "Malformed Request: Method could not be found."
                    )))?;

                    let uri = split.next().ok_or(OxidarError::abort_std(format!(
                        "Malformed Request: URI could not be found."
                    )))?;

                    let version = split.next().ok_or(OxidarError::abort_std(format!(
                        "Malformed Request: HTTP version could not be found."
                    )))?;

                    if let Some(_) = split.next() {
                        return Err(OxidarError::abort_std(format!(
                            "Malformed Request: Items found after version."
                        )));
                    }

                    request = Some(Request {
                        method: Method::try_from(method)?,
                        uri: uri.to_owned(),
                        version: Version::try_from(version)?,
                        headers: HashMap::new(),
                    });
                }
            }
        }

        let request =
            request.ok_or(OxidarError::abort_std(format!("No data found in request.")))?;

        return self.route_to_app(stream, request);
    }

    fn route_to_app(&self, mut stream: TcpStream, request: Request) -> Result<(), OxidarError> {
        self.log(format!("Processing: {} {}", request.method, request.uri));

        let uri = request.uri.clone() + "/";
        for app in &self.apps {
            if uri.starts_with(&app.0) {
                let app_response = app.1.respond(&self, request);
                match app_response {
                    Ok(response) => {
                        OxidarError::aio(stream.write_all(response.build_response().as_bytes()))?;
                        OxidarError::aio(stream.flush())?;
                        return Ok(());
                    }
                    Err(err) => {
                        OxidarError::aio(
                            stream.write_all(
                                match err.to_response() {
                                    Some(response) => response,
                                    None => return Err(err),
                                }
                                .build_response()
                                .as_bytes(),
                            ),
                        )?;
                        OxidarError::aio(stream.flush())?;
                        return Err(err);
                    }
                };
            }
        }

        let e404 = OxidarError::http_404(Some(format!("Could not tie to an app.")));
        OxidarError::aio(
            stream.write_all(
                match e404.to_response() {
                    Some(r) => r,
                    None => return Err(e404),
                }
                .build_response()
                .as_bytes(),
            ),
        )?;
        OxidarError::aio(stream.flush())?;

        return Ok(());
    }

    pub fn log<T>(&self, m: T)
    where
        T: std::fmt::Display,
    {
        let level = LogMethod::Info;
        self.create_log(m, level)
    }

    pub fn logw<T>(&self, m: T)
    where
        T: Display,
    {
        let level = LogMethod::Warning;
        self.create_log(m, level)
    }

    pub fn loge<T>(&self, m: T)
    where
        T: Display,
    {
        let level = LogMethod::Error;
        self.create_log(m, level)
    }

    pub fn create_log<T>(&self, message: T, level: LogMethod)
    where
        T: Display,
    {
        let level_token = match level {
            LogMethod::Info => "\x1b[34mINFO:",
            LogMethod::Warning => "\x1b[33mWARNING:",
            LogMethod::Error => "\x1b[31mERROR:",
        }
        .to_string()
            + "\x1b[0m";

        if let LogStyle::Terminal | LogStyle::TerminalFile(..) = self.log_method {
            println!("{level_token} {message}")
        }
    }
}
