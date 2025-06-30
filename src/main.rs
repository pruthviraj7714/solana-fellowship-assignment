use poem::{handler, http::StatusCode, listener::TcpListener, post, web::Json, Route, Server, Error, Result};
use serde::{Deserialize, Serialize};
use solana_sdk::{
    bs58,
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    system_instruction::create_account,
    transaction::Transaction,
};
use spl_token::{instruction::initialize_mint2, state::Mint, ID as TOKEN_PROGRAM_ID};
use base64::{engine::general_purpose, Engine as _};
use std::str::FromStr;
use std::convert::TryFrom;

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

#[derive(Deserialize)]
pub struct CreateTokenRequest {
    pub mint_authority: String,
    pub mint: String,
    pub decimals: u8,
}

#[derive(Serialize)]
pub struct AccountMetaInfo {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Serialize)]
pub struct CreateTokenResponse {
    pub program_id: String,
    pub secret: String,
}

#[derive(Serialize,Deserialize)]
pub struct SignMessageRequest {
    pub message: String,
    pub secret: String,
}

#[derive(Serialize)]
pub struct SignMessageResponse {
    pub signature: String,
    pub public_key: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct VerifyMessageRequest {
    pub message: String,
    pub signature: String, 
    pub pubkey: String,   
}

#[derive(Serialize)]
pub struct VerifyMessageResponse {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}

#[handler]
pub fn generate_keypair() -> Json<SuccessResponse<KeypairResponse>> {
    let keypair = Keypair::new();

    let base58_private_key = bs58::encode(keypair.to_bytes()).into_string();

    let response = SuccessResponse {
        success: true,
        data: KeypairResponse {
            pubkey: keypair.pubkey().to_string(),
            secret: base58_private_key,
        },
    };

    Json(response)
}

// #[handler]
// pub async fn create_token(Json(payload): Json<CreateTokenRequest>) -> Result<Json<SuccessResponse<CreateTokenResponse>>, ErrorResponse<>> {
//    let authority_pubkey = Pubkey::from_str(&payload.mint_authority)
//         .map_err(|_| poem::Error::from_status(StatusCode::BAD_REQUEST))?;

//     let mint_pubkey = Pubkey::from_str(&payload.mint)
//         .map_err(|_| poem::Error::from_status(StatusCode::BAD_REQUEST))?;

//     let instruction = initialize_mint2(
//         &TOKEN_PROGRAM_ID,
//         &mint_pubkey,
//         &authority_pubkey,
//         Some(&authority_pubkey),
//         payload.decimals,
//     ).map_err(|_| poem::Error::from_status(StatusCode::BAD_REQUEST))?;

//     let instruction_data = general_purpose::STANDARD.encode(&instruction.data);

//     let accounts = instruction
//         .accounts
//         .iter()
//         .map(|meta| AccountMetaInfo {
//             pubkey: meta.pubkey.to_string(),
//             is_signer: meta.is_signer,
//             is_writable: meta.is_writable,
//         })
//         .collect::<Vec<_>>();

//     let response = SuccessResponse {
//         success: true,
//         data: CreateTokenResponse {
//             program_id: instruction.program_id.to_string(),
//             instruction_data,
//             accounts,
//         },
//     };

//     Ok(Json(response))
// }

// #[handler]
// pub fn mint_token() {

// }

// #[handler]
// pub fn sign_message(Json(payload): Json<SignMessageRequest>) -> Json<SuccessResponse<SignMessageResponse>> {
//     let keypair_bytes = bs58::decode(payload.secret)
//         .into_vec()
//         .expect("Invalid base58 secret");

//     let keypair = Keypair::from_bytes(&keypair_bytes)
//         .expect("Invalid secret key format");

//     let signature = keypair.sign_message(payload.message.as_bytes());

//     let response = SuccessResponse {
//         success: true,
//         data: SignMessageResponse {
//             signature: base64::encode(signature.as_ref()),
//             public_key: keypair.pubkey().to_string(),
//             message: payload.message,
//         },
//     };

//     Json(response)
// }

// #[handler]
// pub fn verify_message(Json(payload): Json<VerifyMessageRequest>) -> Json<SuccessResponse<VerifyMessageResponse>> {
//     let pubkey = Pubkey::from_str(&payload.pubkey).expect("Invalid pubkey");
//     let signature_bytes = base64::decode(&payload.signature).expect("Invalid base64 signature");
//     let signature = Signature::try_from(&signature_bytes);

//     let is_valid = signature.verify(pubkey.as_ref(), payload.message.as_bytes());

//     let response = SuccessResponse {
//         success: true,
//         data: VerifyMessageResponse {
//             valid: is_valid,
//             message: payload.message,
//             pubkey: payload.pubkey,
//         },
//     };

//     Json(response)
// }
// #[handler]
// pub fn send_sol() {

// }


// #[handler]
// pub fn send_token() {

// }

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/keypair", post(generate_keypair));
        // .at("/token/create", post(create_token))
        // .at("/token/mint", post(mint_token))
        // .at("/message/sign", post(sign_message))
        // .at("/message/verify", post(verify_message))
        // .at("/send/sol", post(send_sol))
        // .at("/send/token", post(send_token));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
