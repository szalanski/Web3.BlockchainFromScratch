mod blocks;
mod models;
use crate::blocks::block_controller;

use actix_web::{web, App, HttpServer};
use models::application_state::ApplicationState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(ApplicationState::new());

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(block_controller::blocks_request)
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}
