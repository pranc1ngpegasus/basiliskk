use crate::domain::client::google_ime::{
    GoogleIme, TransliterateCandidate, TransliterateInput, TransliterateOutput,
};
use crate::domain::error::BasiliskkErr;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;
use tracing::debug;

pub struct GoogleImeImpl {
    client: Client,
}

impl GoogleImeImpl {
    pub fn new() -> Self {
        let client = Client::new();

        Self { client }
    }
}

const BASE_URL: &str = "https://www.google.com/transliterate?langpair=ja-Hira|ja&text=";

#[async_trait]
impl GoogleIme for GoogleImeImpl {
    async fn transliterate(
        &self,
        input: TransliterateInput,
    ) -> Result<TransliterateOutput, BasiliskkErr> {
        let url = format!("{}{}", BASE_URL, input.kana);

        let res = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| BasiliskkErr {
                desc: format!("Failed to send request to Google IME: {}", e),
            })?;

        let body = res.text().await.map_err(|e| BasiliskkErr {
            desc: format!("failed to read response from Google IME: {}", e),
        })?;

        debug!("response from Google IME: {:?}", body);

        let json: Value = serde_json::from_str(&body).map_err(|e| BasiliskkErr {
            desc: format!("failed to parse response from Google IME: {}", e),
        })?;

        let candidate_array = json.as_array().ok_or(BasiliskkErr {
            desc: format!("failed to parse response from Google IME: {}", json),
        })?;

        let candidates: Vec<TransliterateCandidate> = candidate_array
            .iter()
            .map(|candidate| {
                let kanjis = candidate[1]
                    .as_array()
                    .ok_or(BasiliskkErr {
                        desc: format!("failed to parse response from Google IME: {}", candidate),
                    })
                    .map(|kanjis| {
                        kanjis
                            .iter()
                            .map(|kanji| kanji.to_string().replace('\"', ""))
                            .collect::<Vec<String>>()
                    })
                    .expect("failed to parse response from Google IME");

                TransliterateCandidate {
                    original: candidate[0].to_string().replace('\"', ""),
                    candidates: kanjis,
                }
            })
            .collect();

        Ok(TransliterateOutput { candidates })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn request() {
        let client = GoogleImeImpl::new();
        let out = client
            .transliterate(TransliterateInput {
                kana: "ひらがな".to_string(),
            })
            .await;

        assert!(out.is_ok());
        assert_eq!(
            out.unwrap().candidates[0],
            TransliterateCandidate {
                original: "ひらがな".to_string(),
                candidates: vec![
                    "ひらがな".to_string(),
                    "平仮名".to_string(),
                    "平がな".to_string(),
                    "ヒラガナ".to_string(),
                    "平假名".to_string(),
                ]
            }
        );
    }
}
