pub struct Response {
    pub body: String,
}

impl Response {
    pub fn new(status: &str, contents: String) -> Response {
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status,
            contents.len(),
            contents
        );

        Response { body: response }
    }
}
