use crate::domain::review::ReviewList;
use async_trait::async_trait;

#[async_trait]
pub trait ReviewRepository: Send + Sync {
    async fn get_reviews(&self, book_id: &str) -> ReviewList;
}
