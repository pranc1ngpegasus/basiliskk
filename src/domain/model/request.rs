use crate::domain::model::request_header::RequestHeader;

#[derive(Debug)]
pub struct Request {
    pub header: RequestHeader,
    pub body: Vec<u8>,
}

impl Request {
    pub fn new() -> Self {
        Self {
            header: RequestHeader::default(),
            body: Vec::new(),
        }
    }
}
