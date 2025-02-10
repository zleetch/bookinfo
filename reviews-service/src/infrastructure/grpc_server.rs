use tonic::{Request, Response, Status};
use tonic::transport::Server;
use tonic_reflection::server::Builder;

use crate::application::reviews_service::ReviewsService;
use crate::domain::repository::ReviewRepository;

pub mod reviews_proto {
    tonic::include_proto!("reviews");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("reviews_descriptor");
}

use reviews_proto::reviews_server::{Reviews, ReviewsServer};
use reviews_proto::{BookRequest, Review, ReviewList};

pub struct GrpcReviewsService<R: ReviewRepository> {
    pub reviews_service: ReviewsService<R>,
}

#[tonic::async_trait]
impl<R: ReviewRepository + 'static> Reviews for GrpcReviewsService<R> {
    async fn get_reviews(
        &self,
        request: Request<BookRequest>,
    ) -> Result<Response<ReviewList>, Status> {
        let req = request.into_inner();
        let domain_reviews = self.reviews_service.get_reviews(&req.book_id).await;

        let reply = ReviewList {
            version: domain_reviews.version,
            reviews: domain_reviews
                .reviews
                .into_iter()
                .map(|r| Review {
                    reviewer: r.reviewer,
                    comment: r.comment,
                })
                .collect(),
        };

        Ok(Response::new(reply))
    }
}

pub async fn serve_grpc<R: ReviewRepository + 'static>(
    reviews_service: ReviewsService<R>,
    add: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = add.parse()?;
    let grpc_service = GrpcReviewsService { reviews_service };

    println!("Reviews service listening on {}", &addr);

    let reflection_service = Builder::configure()
        .register_encoded_file_descriptor_set(reviews_proto::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    Server::builder()
        .add_service(ReviewsServer::new(grpc_service))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}