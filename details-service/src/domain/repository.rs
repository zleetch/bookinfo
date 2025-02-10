use crate::domain::detail::Detail;
use async_trait::async_trait;

#[async_trait]
pub trait DetailRepository: Send + Sync {
    async fn get_details(&self, id: &str) -> Option<Detail>;
}