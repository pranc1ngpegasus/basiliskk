use super::error::BasiliskkErr;
use async_trait::async_trait;

#[async_trait]
pub trait Listener {
    async fn listen(&self) -> Result<(), BasiliskkErr>;
}
