use std::env;

use actix_web::{post, web, App, HttpResponse, HttpServer};

use crate::interface::adapter::JsonAlBhedTransferAdapter;

pub async fn start_server(adapter: JsonAlBhedTransferAdapter) -> std::io::Result<()> {
    let port = env::var("BACKEND_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    let actix_adapter = web::Data::new(adapter);
    HttpServer::new(move || {
        App::new()
            .app_data(actix_adapter.clone())
            .service(encode_handler)
            .service(decode_handler)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

#[post("/encode")]
async fn encode_handler(
    body: String,
    adapter: web::Data<JsonAlBhedTransferAdapter>,
) -> HttpResponse {
    match adapter.encode(&body) {
        Ok(response) => HttpResponse::Ok()
            .content_type("application/json")
            .body(response),
        Err(error) => HttpResponse::BadRequest().body(error),
    }
}

#[post("/decode")]
async fn decode_handler(
    body: String,
    adapter: web::Data<JsonAlBhedTransferAdapter>,
) -> HttpResponse {
    match adapter.decode(&body) {
        Ok(response) => HttpResponse::Ok()
            .content_type("application/json")
            .body(response),
        Err(error) => HttpResponse::BadRequest().body(error),
    }
}

#[cfg(test)]
mod tests {
    use actix_web::test;

    use super::*;
    use crate::usecase::{decode_usecase::DecodeInteractor, encode_usecase::EncodeInteractor};

    #[actix_web::test]
    async fn test_encode_endpoint_valid() {
        let encode_port = EncodeInteractor::new();
        let decode_port = DecodeInteractor::new();
        let adapter = JsonAlBhedTransferAdapter::new(Box::new(encode_port), Box::new(decode_port));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(adapter))
                .service(encode_handler)
                .service(decode_handler),
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/encode")
            .set_json(serde_json::json!({"text": "ごめん"}))
            .to_request();

        let response = test::call_service(&app, request).await;
        assert!(response.status().is_success());

        let body: serde_json::Value = test::read_body_json(response).await;
        assert_eq!(body, serde_json::json!({"result": "ゾレン"}))
    }

    #[actix_web::test]
    async fn test_encode_endpoint_invalid() {
        let encode_port = EncodeInteractor::new();
        let decode_port = DecodeInteractor::new();
        let adapter = JsonAlBhedTransferAdapter::new(Box::new(encode_port), Box::new(decode_port));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(adapter))
                .service(encode_handler)
                .service(decode_handler),
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/encode")
            .set_json(serde_json::json!({"str":"str"}))
            .to_request();

        let response = test::call_service(&app, request).await;
        assert!(response.status().is_client_error());
    }

    #[actix_web::test]
    async fn test_decode_endpoint_valid() {
        let encode_port = EncodeInteractor::new();
        let decode_port = DecodeInteractor::new();
        let adapter = JsonAlBhedTransferAdapter::new(Box::new(encode_port), Box::new(decode_port));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(adapter))
                .service(encode_handler)
                .service(decode_handler),
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/decode")
            .set_json(serde_json::json!({"text": "フエキミ、モ！"}))
            .to_request();

        let response = test::call_service(&app, request).await;
        assert!(response.status().is_success());

        let body: serde_json::Value = test::read_body_json(response).await;
        assert_eq!(body, serde_json::json!({"result": "うれしい、よ！"}))
    }

    #[actix_web::test]
    async fn test_decode_endpoint_invalid() {
        let encode_port = EncodeInteractor::new();
        let decode_port = DecodeInteractor::new();
        let adapter = JsonAlBhedTransferAdapter::new(Box::new(encode_port), Box::new(decode_port));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(adapter))
                .service(encode_handler)
                .service(decode_handler),
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/decode")
            .set_json(serde_json::json!({"str":"str"}))
            .to_request();

        let response = test::call_service(&app, request).await;
        assert!(response.status().is_client_error());
    }
}
