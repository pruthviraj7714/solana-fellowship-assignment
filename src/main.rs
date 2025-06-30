use poem::{Route, Server, handler, listener::TcpListener, post, web::Json};
use serde::Serialize;
use solana_sdk::{signature::Keypair, signer::Signer};

#[derive(Serialize)]
pub struct SuccessResponse<T> {
    success: bool,
    data: T,
}

#[derive(Serialize)]
pub struct ErrorResponse<T> {
    success: bool,
    error: T,
}

#[derive(Serialize)]
pub struct KeypairResponse {
    pub pubkey: String,
    pub secret: String,
}

#[handler]
fn generate_keypair() -> Json<SuccessResponse<KeypairResponse>> {
    let keypair = Keypair::new();
    let address = keypair.pubkey();

    let response = SuccessResponse {
        success: true,
        data: KeypairResponse {
            pubkey: keypair.pubkey().to_string(),
            secret: keypair.secret().to_string(),
        },
    };

    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/keypair", post(generate_keypair));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
