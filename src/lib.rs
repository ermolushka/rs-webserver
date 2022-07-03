mod server;
use serde::Deserialize;
use serde::Serialize;
use server::response::Response;
use server::threadpool::ThreadPool;
use server::worker::RequestFlow;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let mapping = create_mapping();

    let mut path = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    path.push_str("/templates/");

    let mut status = "";

    for val in mapping.iter() {
        if buffer.starts_with(val.request.clone().unwrap().as_bytes()) {
            status = &val.status;
            path.push_str(&val.filename);
        }
    }

    if status == "" {
        let unknown = unknown_request();
        status = &unknown.status;
        path.push_str(&unknown.filename);

        create_response(&mut stream, path, status);
    } else {
        create_response(&mut stream, path, status);
    }
}

pub fn start_server(host: String, port: String, num_of_threads: usize) {
    let address: String = format!("{}:{}", host, port);
    let listener = TcpListener::bind(address).unwrap();
    let pool = ThreadPool::new(num_of_threads);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

pub fn create_mapping() -> Vec<RequestFlow> {
    // parse a json metadata
    let mut path = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    path.push_str("/metadata.json");

    let json_data = fs::read_to_string(path).unwrap();
    let parsed_data: JsonSerialize =
        serde_json::from_str(&json_data).expect("JSON was not well-formatted");

    let mut mapping: Vec<RequestFlow> = Vec::new();

    for val in parsed_data.endpoints {
        let request = String::from(val.method);
        let path = val.endpoint.as_str();
        let filename = val.template;

        mapping.push(RequestFlow {
            request: Some(String::from(format!("{} {} HTTP/1.1\r\n", request, path))),
            filename: filename,
            status: String::from("HTTP/1.1 200 OK"),
        })
    }

    mapping
}

pub fn unknown_request() -> RequestFlow {
    RequestFlow {
        request: None,
        filename: String::from("404.html"),
        status: String::from("HTTP/1.1 404 NOT FOUND"),
    }
}

pub fn create_response(stream: &mut TcpStream, path: String, status: &str) {
    let contents = fs::read_to_string(path).unwrap();
    let response = Response::new(status, contents);
    stream.write(response.body.as_bytes()).unwrap();
    stream.flush().unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonItem {
    method: String,
    endpoint: String,
    template: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct JsonSerialize {
    endpoints: Vec<JsonItem>,
}
