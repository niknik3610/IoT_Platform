use actix_web::{HttpResponse, Responder};

use crate::types::types;
use crate::web_json_translation::json_translation::TranslationClientState;

pub mod json_registration_service {

    tonic::include_proto!("frontend.registration");
}

#[actix_web::options("/frotend/registration")]
pub async fn json_registration_options() -> impl Responder {
    return HttpResponse::Ok().body("");
}

#[actix_web::post("/frontend/registration")]
pub async fn json_registration(
    req_body: String,
    state: actix_web::web::Data<TranslationClientState>,
) -> impl Responder {
    let parsed_req: json_registration_service::RegistrationRequest =
        match serde_json::from_str(&req_body) {
            Ok(r) => r,
            Err(_e) => return HttpResponse::BadRequest().body("Unable to parse request"),
        };

    let response = {
        let mut registration_service_client = state.registration_client.lock().await;
        registration_service_client.register(parsed_req).await
    };

    let response = match response {
        Ok(r) => r.into_inner(),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    return HttpResponse::Ok().body(serde_json::to_string(&response).unwrap());
}
