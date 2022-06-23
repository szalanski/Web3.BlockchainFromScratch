use crate::models::application_state::ApplicationState;
use actix_web::{get, web, Responder, Result};

#[get("/blocks")]
async fn blocks_request(app: web::Data<ApplicationState>) -> Result<impl Responder> {
    Ok(web::Json(app.bc.chain.clone()))
}
