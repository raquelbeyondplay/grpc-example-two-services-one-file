use tonic::{ Request, Response, Status};
use proto::admin_server::{Admin,AdminServer};
use proto::calculator_server::{Calculator,CalculatorServer};
use tonic::transport::Server;

mod proto {
    tonic::include_proto!("calculator");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("calculator_descriptor");
}

#[derive(Debug, Default)]
struct CalculatorService;

#[derive(Debug, Default)]
struct AdminService;

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn get_app_version(
        &self,
        _request: Request<proto::CalculatorEmptyRequest>,
    ) -> Result<Response<proto::CalculatorResponse>, Status> {
        Ok(Response::new(proto::CalculatorResponse {
            name: "Version 1.0".to_string(),
        }))
    }
}

#[tonic::async_trait]
impl Admin for AdminService {
    async fn get_admin_user(
        &self,
        _request: Request<proto::AdminEmptyRequest>,
    ) -> Result<Response<proto::AdminResponse>, Status> {
        Ok(Response::new(proto::AdminResponse {
            brand: Some(proto::Brand {
                name: "Admin User".to_string(),
            }),
        }))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let calculator_service = CalculatorServer::new(CalculatorService::default());
    let admin_service = AdminServer::new(AdminService::default());


    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(service)
        .add_service(calculator_service)
        .add_service(admin_service)
        .serve(addr)
        .await?;

    Ok(())
}
