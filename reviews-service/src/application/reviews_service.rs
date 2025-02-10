use crate::domain::repository::ReviewRepository;
use crate::domain::review::ReviewList;

pub struct ReviewsService<R: ReviewRepository> {
    repository: R,
}

impl<R: ReviewRepository> ReviewsService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_reviews(&self, book_id: &str) -> ReviewList {
        self.repository.get_reviews(book_id).await
    }
}
