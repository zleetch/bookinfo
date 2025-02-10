mod application;
mod domain;
mod infrastructure;

use std::env;
use infrastructure::grpc_server::serve_grpc;
use infrastructure::repository_impl::InMemoryDetailRepository;
use application::details_service::DetailsService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let version = env::var("APP_VERSION").unwrap_or_else(|_| "v1.0.0".into());
    let port = env::var("APP_PORT").unwrap_or_else(|_| "50002".into());
    let addr = format!("127.0.0.1:{}", port);

    let repo = InMemoryDetailRepository { version: version.clone() };
    let detail_service = DetailsService::new(repo);
    serve_grpc(detail_service, &addr).await?;
    Ok(())
}
