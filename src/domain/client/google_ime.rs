use crate::domain::error::BasiliskkErr;
use async_trait::async_trait;

#[async_trait]
pub trait GoogleIme {
    async fn transliterate(
        &self,
        input: TransliterateInput,
    ) -> Result<TransliterateOutput, BasiliskkErr>;
}

pub struct TransliterateInput {
    pub kana: String,
}

#[derive(Debug, PartialEq)]
pub struct TransliterateCandidate {
    pub original: String,
    pub candidates: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct TransliterateOutput {
    pub candidates: Vec<TransliterateCandidate>,
}
