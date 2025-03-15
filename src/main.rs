use albhed_translator_service::{
    infrastracture::web,
    interface::adapter::JsonAlBhedTransferAdapter,
    usecase::{decode_usecase::DecodeInteractor, encode_usecase::EncodeInteractor},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let encode_port = EncodeInteractor::new();
    let decode_port = DecodeInteractor::new();
    let adapter = JsonAlBhedTransferAdapter::new(Box::new(encode_port), Box::new(decode_port));
    web::start_server(adapter).await
}
