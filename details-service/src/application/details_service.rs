use crate::domain::detail::Detail;
use crate::domain::repository::DetailRepository;

pub struct DetailsService<R: DetailRepository> {
    repository: R,
}

impl<R: DetailRepository> DetailsService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_details(&self, book_id: &str) -> Option<Detail> {
        self.repository.get_details(book_id).await
    }
}