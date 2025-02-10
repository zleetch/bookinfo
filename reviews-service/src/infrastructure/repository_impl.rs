use async_trait::async_trait;
use crate::domain::repository::ReviewRepository;
use crate::domain::review::{Review, ReviewList};

pub struct InMemoryReviewRepository {
    pub version: String,
}

#[async_trait]
impl ReviewRepository for InMemoryReviewRepository {
    async fn get_reviews(&self, _book_id: &str) -> ReviewList {
        let reviews = if self.version == "v2.0.0" {
            vec![
                Review {
                    reviewer: "Udin".into(),
                    comment: "Buku yang bagus! (v2)".into(),
                },
                Review {
                    reviewer: "Ucok".into(),
                    comment: "Rekomendasi banget sih. (v2)".into(),
                },
            ]
        } else {
            vec![
                Review {
                    reviewer: "Udin".into(),
                    comment: "Buku yang bagus!".into(),
                },
                Review {
                    reviewer: "Ucok".into(),
                    comment: "Rekomendasi banget sih.".into(),
                },
            ]
        };

        ReviewList {
            reviews,
            version: self.version.clone(),
        }
    }
}