use actix_web::{HttpResponse, Responder};

use crate::{
    registration::frontend_registration_service::ConnectedDevicesRequest,
    web_json_translation::json_translation::TranslationClientState,
};

#[actix_web::post("/frontend/get_connected_devices")]
pub async fn get_connected_devices(
    req_body: String,
    _state: actix_web::web::Data<TranslationClientState>,
) -> impl Responder {
    let _device_id: ConnectedDevicesRequest = match serde_json::from_str(&req_body) {
        Ok(r) => r,
        Err(_e) => return HttpResponse::BadRequest().body("malformed request"),
    };
    return HttpResponse::Ok().body("ok");
}
