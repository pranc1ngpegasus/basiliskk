use crate::domain::error::BasiliskkErr;

#[derive(Debug, Default, PartialEq)]
pub enum RequestHeader {
    #[default]
    Disconnect,
    Conversion,
    Version,
    HostInfo,
    Completion,
}

impl std::fmt::Display for RequestHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Disconnect => write!(f, "Disconnect"),
            Self::Conversion => write!(f, "Conversion"),
            Self::Version => write!(f, "Version"),
            Self::HostInfo => write!(f, "HostInfo"),
            Self::Completion => write!(f, "Completion"),
        }
    }
}

impl TryFrom<Vec<u8>> for RequestHeader {
    type Error = BasiliskkErr;

    fn try_from(input: Vec<u8>) -> Result<Self, Self::Error> {
        if input.len() != 1 {
            return Err(BasiliskkErr {
                desc: format!("invalid header length: {}", input.len()),
            });
        }

        match input[0] {
            b'0' => Ok(RequestHeader::Disconnect),
            b'1' => Ok(RequestHeader::Conversion),
            b'2' => Ok(RequestHeader::Version),
            b'3' => Ok(RequestHeader::HostInfo),
            b'4' => Ok(RequestHeader::Completion),
            _ => Err(BasiliskkErr {
                desc: format!("failed to parse request header: {:?}", input),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![b'0'], RequestHeader::Disconnect)]
    #[test_case(vec![b'1'], RequestHeader::Conversion)]
    #[test_case(vec![b'2'], RequestHeader::Version)]
    #[test_case(vec![b'3'], RequestHeader::HostInfo)]
    #[test_case(vec![b'4'], RequestHeader::Completion)]
    fn parse_succeed(input: Vec<u8>, expect: RequestHeader) {
        let header = RequestHeader::try_from(input);

        assert!(header.is_ok());
        assert_eq!(header.unwrap(), expect)
    }
}
