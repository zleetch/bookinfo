mod application;
mod domain;
mod infrastructure;

use std::env;
use infrastructure::grpc_server::serve_grpc;
use infrastructure::repository_impl::InMemoryReviewRepository;
use application::reviews_service::ReviewsService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let version = env::var("APP_VERSION").unwrap_or_else(|_| "v1.0.0".into());
    let port = env::var("APP_PORT").unwrap_or_else(|_| "50001".into());
    let addr = format!("127.0.0.1:{}", port);

    let repo = InMemoryReviewRepository { version: version.clone() };
    let review_service = ReviewsService::new(repo);
    serve_grpc(review_service, &addr).await?;
    Ok(())
}
