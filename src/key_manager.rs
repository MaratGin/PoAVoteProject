// use pem::{Pem, encode};
// use actix_web::{web, App, HttpServer, Responder};
use rsa::{RsaPrivateKey, RsaPublicKey};
use rand::rngs::OsRng;
use serde::Serialize;
// use std::sync::Arc;

#[derive(Serialize)]
pub struct PublicKeyResponse {
    pub public_key: String,
}

pub struct ServerState {
   pub private_key: RsaPrivateKey,
   pub public_key: RsaPublicKey,
}

impl ServerState {
    pub fn new() -> Self {
        let mut rng = OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);
        ServerState { private_key, public_key }
    }
}




// fn generate_keys() -> (String, String) {
//     // Генерация приватного ключа
//     let mut rng = OsRng;
//     let bits = 2048;
//     let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
//     let public_key = RsaPublicKey::from(&private_key);

//     // Сериализация ключей в формат PEM
//     let private_key_pem = Pem {
//         tag: String::from("RSA PRIVATE KEY"),
//         contents: private_key.to_pkcs1_der().unwrap().as_ref().to_vec(),
//     };
//     let public_key_pem = Pem {
//         tag: String::from("RSA PUBLIC KEY"),
//         contents: public_key.to_pkcs1_der().unwrap().as_ref().to_vec(),
//     };

//     (encode(&private_key_pem), encode(&public_key_pem))
// }


