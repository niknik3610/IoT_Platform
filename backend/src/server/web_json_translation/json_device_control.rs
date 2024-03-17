use actix_web::{HttpResponse, Responder};

use crate::web_json_translation::json_translation::TranslationClientState;

pub mod frontend_device_control {
    tonic::include_proto!("frontend.devicecontrol");
}

#[actix_web::options("/frontend/device_control")]
pub async fn json_device_control_options() -> impl Responder {
    return HttpResponse::Ok().body("");
}

#[actix_web::post("/frontend/device_control")]
pub async fn json_device_control(
    req_body: String,
    state: actix_web::web::Data<TranslationClientState>,
) -> impl Responder {
    let request: frontend_device_control::DeviceControlRequest =
        match serde_json::from_str(&req_body) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("The JSON proxy received a malformed request with error: \n{e}");
                return HttpResponse::BadRequest().body("malformed request");
            }
        };
    let response = {
        let mut client = state.device_control_client.lock().await;
        client.control_device(request).await
    };

    let response = match response {
        Ok(r) => r.into_inner(),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    let json_response = serde_json::to_string(&response);
    match json_response {
        Ok(r) => return HttpResponse::Ok().body(r),
        Err(e) => {
            eprintln!("{e}");
            return HttpResponse::InternalServerError()
                .body("Something went wrong while parsing the request");
        }
    }
}
