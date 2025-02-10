use async_trait::async_trait;
use crate::domain::detail::Detail;
use crate::domain::repository::DetailRepository;

pub struct InMemoryDetailRepository {
    pub version: String,
}

#[async_trait]
impl DetailRepository for InMemoryDetailRepository {
    async fn get_details(&self, id: &str) -> Option<Detail> {
        let details = if self.version == "v2.0.0" {
            vec![
                Detail {
                    title: "Timun Emas".into(),
                    description: "Buku cerita anak (v2)".into(),
                },
                Detail {
                    title: "Habibie Ainun".into(),
                    description: "Buku inspirasi (v2)".into(),
                },
            ]
        } else {
            vec![
                Detail {
                    title: "Timun Emas".into(),
                    description: "Buku cerita anak".into(),
                },
                Detail {
                    title: "Habibie Ainun".into(),
                    description: "Buku inspirasi".into(),
                },
            ]
        };

        DetailList {
            details,
            version: self.version.clone(),
        }
    }
}