use tonic::{transport::Server, Request, Response, Status};
use crate::api::switch_machine;
use crate::api::make_product;
use crate::api::product_definitions::{ProductParameters, ProductType, ProductStrength};
use jurassip::coffee_maker_server::{CoffeeMaker, CoffeeMakerServer};
use jurassip::{MakeProductRequest, MachineResponse};
use std::env;

pub mod jurassip {
    tonic::include_proto!("jurassip");
}

#[derive(Default)]
pub struct CoffeeMakerService;

#[tonic::async_trait]
impl CoffeeMaker for CoffeeMakerService {
    async fn make_product(
        &self,
        request: Request<MakeProductRequest>,
    ) -> Result<Response<MachineResponse>, Status> {
        let req: MakeProductRequest = request.into_inner();

        let product_type: ProductType = match req.product_type.as_str() {
            "SingleCoffee" => ProductType::SingleCoffee,
            "DoubleCoffee" => ProductType::DoubleCoffee,
            "SingleEspresso" => ProductType::SingleEspresso,
            "DoubleEspresso" => ProductType::DoubleEspresso,
            _ => return Err(Status::invalid_argument("Invalid coffee type")),
        };

        let product_strength: ProductStrength = match req.product_strength.as_str() {
            "Mild" => ProductStrength::Mild,
            "Normal" => ProductStrength::Normal,
            "Strong" => ProductStrength::Strong,
            "Extra" => ProductStrength::Extra,
            _ => return Err(Status::invalid_argument("Invalid strength")),
        };

        let params: ProductParameters = ProductParameters {
            product_type,
            product_strength,
        };

        if let Err(e) = make_product::make_product(params) {
            return Err(Status::internal(format!("Failed to make product: {}", e)));
        }

        let response: MachineResponse = MachineResponse {
            message: "Your product is ready!".to_string(),
        };

        Ok(Response::new(response))
    }

    async fn machine_on(
        &self,
        _request: Request<jurassip::Empty>,
    ) -> Result<Response<MachineResponse>, Status> {
        if let Err(e) = switch_machine::machine_on() {
            return Err(Status::internal(format!("Failed to turn on machine: {}", e)));
        }
        let response: MachineResponse = MachineResponse {
            message: "Machine turned on!".to_string(),
        };
        Ok(Response::new(response))
    }

    async fn machine_off(
        &self,
        _request: Request<jurassip::Empty>,
    ) -> Result<Response<MachineResponse>, Status> {
        if let Err(e) = switch_machine::machine_off() {
            return Err(Status::internal(format!("Failed to turn off machine: {}", e)));
        }
        let response: MachineResponse = MachineResponse {
            message: "Machine turned off!".to_string(),
        };
        Ok(Response::new(response))
    }
}

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let host: String = env::var("GRPC_SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: String = env::var("GRPC_SERVER_PORT").unwrap_or_else(|_| "50051".to_string());
    let addr: std::net::SocketAddr = format!("{}:{}", host, port).parse()?;
    let coffee_maker = CoffeeMakerService::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(CoffeeMakerServer::new(coffee_maker))
        .serve(addr)
        .await?;

    Ok(())
}
