use tonic::{Request, Response, Status};
use tonic::transport::Server;
use tonic_reflection::server::Builder;

use crate::application::details_service::DetailsService;
use crate::domain::repository::DetailRepository;

pub mod details_proto {
    tonic::include_proto!("details");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("details_descriptor");
}

use details_proto::details_server::{Details, DetailsServer};
use details_proto::{BookRequest, Detail, DetailList};

pub struct GrpcDetailsService<R: DetailRepository> {
    pub details_service: DetailsService<R>,
}

#[tonic::async_trait]
impl<R: DetailRepository + 'static> Details for GrpcDetailsService<R> {
    async fn get_details(
        &self,
        request: Request<BookRequest>,
    ) -> Result<Response<DetailsList>, Status> {
        let req = request.into_inner();
        let domain_details = self.details_service.get_details(&req.book_id).await;

        let reply = DetailList {
            version: domain_details.version,
            details: domain_details
                .details
                .into_iter()
                .map(|r| Detail {
                    title: r.title,
                    description: r.description,
                })
                .collect(),
        };

        Ok(Response::new(reply))
    }
}

pub async fn serve_grpc<R: DetailRepository + 'static>(
    details_service: DetailsService<R>,
    add: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = add.parse()?;
    let grpc_service = GrpcDetailsService { details_service };

    println!("Details service listening on {}", &addr);

    let reflection_service = Builder::configure()
        .register_encoded_file_descriptor_set(details_proto::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    Server::builder()
        .add_service(DetailsServer::new(grpc_service))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}