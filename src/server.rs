use tonic::{transport::Server, Request, Response, Status};
use crate::api::make_product;
use crate::api::product_definitions::{CoffeeParameters, ProductType, ProductStrength};
use jurassip::coffee_maker_server::{CoffeeMaker, CoffeeMakerServer};
use jurassip::{MakeProductRequest, MakeProductResponse};

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
    ) -> Result<Response<MakeProductResponse>, Status> {
        let req = request.into_inner();

        let coffee_type = match req.coffee_type.as_str() {
            "CoffeeSingle" => ProductType::CoffeeSingle,
            "CoffeeDouble" => ProductType::CoffeeDouble,
            "EspressoSingle" => ProductType::EspressoSingle,
            "EspressoDouble" => ProductType::EspressoDouble,
            _ => return Err(Status::invalid_argument("Invalid coffee type")),
        };

        let strength = match req.strength.as_str() {
            "Mild" => ProductStrength::Mild,
            "Normal" => ProductStrength::Normal,
            "Strong" => ProductStrength::Strong,
            "Extra" => ProductStrength::Extra,
            _ => return Err(Status::invalid_argument("Invalid strength")),
        };

        let params = CoffeeParameters {
            coffee_type,
            strength,
        };

        if let Err(e) = make_product::make_coffee(params) {
            return Err(Status::internal(format!("Failed to make product: {}", e)));
        }

        let response = MakeProductResponse {
            message: "Your product is ready!".to_string(),
        };

        Ok(Response::new(response))
    }
}

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let coffee_maker = CoffeeMakerService::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(CoffeeMakerServer::new(coffee_maker))
        .serve(addr)
        .await?;

    Ok(())
}
