use actix_web::{Responder, HttpResponse};

use crate::{web_json_translation::json_translation::TranslationClientState, registration::frontend_registration_service::ConnectedDevicesRequest};

#[actix_web::post("/frontend/get_connected_devices")]
pub async fn get_connected_devices(
    req_body: String,
    state: actix_web::web::Data<TranslationClientState>,
    ) -> impl Responder{
    let device_id: ConnectedDevicesRequest = match serde_json::from_str(&req_body) {
        Ok(r) => r,
        Err(_e) => return HttpResponse::BadRequest().body("malformed request")
    };
    return HttpResponse::Ok().body("ok");
}
