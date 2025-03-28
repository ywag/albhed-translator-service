use std::env;

use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer};

use crate::interface::adapter::JsonAlBhedTransferAdapter;

pub async fn start_server(adapter: JsonAlBhedTransferAdapter) -> std::io::Result<()> {
    let port = env::var("BACKEND_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    let frontend_origin =
        env::var("FRONTEND_ORIGIN").unwrap_or("http://localhost:5173".to_string());

    let actix_adapter = web::Data::new(adapter);
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(&frontend_origin)
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE]),
            )
            .app_data(actix_adapter.clone())
            .service(encode_handler)
            .service(decode_handler)
            .service(health_check)
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

#[get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
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

    #[actix_web::test]
    async fn test_health_check() {
        let encode_port = EncodeInteractor::new();
        let decode_port = DecodeInteractor::new();
        let adapter = JsonAlBhedTransferAdapter::new(Box::new(encode_port), Box::new(decode_port));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(adapter))
                .service(encode_handler)
                .service(decode_handler)
                .service(health_check),
        )
        .await;

        let request = test::TestRequest::get().uri("/health").to_request();
        let response = test::call_service(&app, request).await;
        assert!(response.status().is_success());
    }
}
