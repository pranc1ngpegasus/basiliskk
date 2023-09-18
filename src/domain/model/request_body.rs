use crate::domain::error::BasiliskkErr;
use encoding_rs::EUC_JP;

#[derive(Debug, Default, PartialEq)]
pub struct RequestBody(String);

impl TryFrom<Vec<u8>> for RequestBody {
    type Error = BasiliskkErr;

    fn try_from(input: Vec<u8>) -> Result<Self, Self::Error> {
        match EUC_JP.decode(&input) {
            (decoded, _, false) => Ok(RequestBody(decoded.to_string())),
            (_, _, true) => Err(BasiliskkErr {
                desc: String::from("failed to decode request body"),
            }),
        }
    }
}

impl std::fmt::Display for RequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![0xa4, 0xc6, 0xa4, 0xb9, 0xa4, 0xc8], String::from("てすと"))]
    #[test_case(vec![0xa4, 0xe2, 0xa4, 0xb8, 0xa4, 0xec, 0xa4, 0xc4], String::from("もじれつ"))]
    fn decode_req(input: Vec<u8>, expect: String) {
        let parsed = RequestBody::try_from(input);

        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap().to_string(), expect);
    }
}
