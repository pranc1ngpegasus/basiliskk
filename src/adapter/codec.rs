use crate::domain::error::BasiliskkErr;
use crate::domain::model::request::Request;
use crate::domain::model::request_body::RequestBody;
use crate::domain::model::request_header::RequestHeader;
use bytes::BytesMut;
use tokio_util::codec::Decoder;

enum DecodeState {
    Head,
    Data,
}

pub struct SkkCodecImpl {
    state: DecodeState,
}

const MESSAGE_HEADER_LENGTH: usize = 1;

impl SkkCodecImpl {
    pub fn new() -> Self {
        Self {
            state: DecodeState::Head,
        }
    }

    fn decode_header(&self, src: &mut BytesMut) -> Option<BytesMut> {
        if src.len() < MESSAGE_HEADER_LENGTH {
            return None;
        }

        src.reserve(MESSAGE_HEADER_LENGTH);

        Some(src.split_to(MESSAGE_HEADER_LENGTH))
    }

    fn decode_body(&self, src: &mut BytesMut) -> Option<BytesMut> {
        src.reserve(src.len());

        Some(src.split_to(src.len()))
    }
}

impl Decoder for SkkCodecImpl {
    type Item = Request;
    type Error = BasiliskkErr;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut parsed = Request::new();

        loop {
            match self.state {
                DecodeState::Head => match self.decode_header(src) {
                    Some(data) => {
                        self.state = DecodeState::Data;
                        parsed.header = RequestHeader::try_from(data.to_vec())?;
                    }
                    None => break,
                },
                DecodeState::Data => match self.decode_body(src) {
                    Some(data) => {
                        self.state = DecodeState::Head;
                        parsed.body = RequestBody::try_from(data.to_vec())?;

                        return Ok(Some(parsed));
                    }
                    None => break,
                },
            }
        }

        self.state = DecodeState::Head;
        src.clear();

        Ok(None)
    }
}
