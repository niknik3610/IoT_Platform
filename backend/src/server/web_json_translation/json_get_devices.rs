use actix_web::{HttpResponse, Responder};

use crate::{
    web_json_translation::json_registration::json_registration_service::ConnectedDevicesRequest,
    web_json_translation::json_translation::TranslationClientState,
};

#[actix_web::options("/frontend/get_connected_devices")]
pub async fn json_registration_options() -> impl Responder {
    return HttpResponse::Ok().body("");
}

#[actix_web::post("/frontend/get_connected_devices")]
pub async fn get_connected_devices(
    req_body: String,
    state: actix_web::web::Data<TranslationClientState>,
) -> impl Responder {
    let request: ConnectedDevicesRequest = match serde_json::from_str(&req_body) {
        Ok(r) => r,
        Err(_e) => {
            eprintln!("{req_body}");
            return HttpResponse::BadRequest().body("malformed request");
        }
    };

    let response = {
        let mut registration_client = state.registration_client.lock().await;
        registration_client.get_connected_devices(request).await
    };

    let response = match response {
        Ok(r) => {
            r.into_inner()
        },
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string())
    };

    let json_response = serde_json::to_string(&response);
    match json_response {
        Ok(r) => return HttpResponse::Ok().body(r),
        Err(e) => {
            eprintln!("{e}");
            return HttpResponse::InternalServerError().body("Something went wrong while parsing the request");
        }
    }
}
