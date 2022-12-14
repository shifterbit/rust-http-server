use crate::mimetypes::HTTPContentType;

pub enum HTTPResponseCode {
    OK,
    NotFound,
}

impl ToString for HTTPResponseCode {
    fn to_string(&self) -> String {
        match self {
            HTTPResponseCode::OK => "200 OK".to_string(),
            HTTPResponseCode::NotFound => "404 Not Found".to_string(),
        }
    }
}

pub struct HTTPResponse {
    code: HTTPResponseCode,
    content_type: HTTPContentType,
    content_length: usize,
    pub body: Vec<u8>,
}

impl HTTPResponse {
    pub fn new<'a>(
        code: HTTPResponseCode,
        content_type: HTTPContentType,
        body: Vec<u8>,
    ) -> HTTPResponse {
        HTTPResponse {
            code,
            content_type,
            content_length: body.len(),
            body,
        }
    }
    pub fn headers(&self) -> Vec<String> {
        let base = format!("HTTP/1.1 {}\n", self.code.to_string());
        let content_type = format!("Content-Type: {}\n", self.content_type.to_string());
        let content_length = format!("Content-Length: {}", self.content_length);
        let header_end = "\r\n\r\n";
        vec![
            base.to_owned(),
            content_type.to_owned(),
            content_length.to_owned(),
            header_end.to_owned(),
        ]
    }
}

pub fn log_response(response: HTTPResponse) {
    let HTTPResponse {
        code, content_type, ..
    }: HTTPResponse = response;
    println!(
        "[RESPONSE] {} content-type: {}",
        code.to_string(),
        content_type.to_string()
    )
}
