use crate::domain::model::request_body::RequestBody;
use crate::domain::model::request_header::RequestHeader;

#[derive(Debug)]
pub struct Request {
    pub header: RequestHeader,
    pub body: RequestBody,
}

impl Request {
    pub fn new() -> Self {
        Self {
            header: RequestHeader::default(),
            body: RequestBody::default(),
        }
    }
}

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "header: {}, body: {}", self.header, self.body)
    }
}
