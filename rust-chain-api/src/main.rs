use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use rust_chain::block::Block;
use rust_chain::blockchain::Blockchain;
use rust_chain::hash::{Hash, HashValue};
use serde::Serialize;
use sha2::{Digest, Sha256};

#[get("/blocks")]
async fn blocks_request(bc: web::Data<Blockchain<Data>>) -> impl Responder {
    // HttpResponse::Ok().body(format!("Blocks {}", bc.chain.len()))

    web::Json(bc.chain.clone())
}

///Data type under test.
#[derive(Debug, Default, Eq, PartialEq, Clone, Serialize)]
pub struct Data {
    pub field: i32,
}

impl Hash for Data {
    fn hash(&self) -> HashValue {
        Sha256::digest(self.field.to_string()).into()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bc: Blockchain<Data> = Blockchain::new();
    let app_state = web::Data::new(bc);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(blocks_request)
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}
