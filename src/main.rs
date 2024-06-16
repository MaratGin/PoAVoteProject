#[warn(non_snake_case)]


mod handlers;
mod repository;

mod block;
mod key_manager;
mod genesis_block;
mod models;
mod transaction;

use key_manager::ServerState;
use std::sync::{Arc, Mutex};
use rsa::{pkcs1::ToRsaPublicKey};


use diesel::prelude::*;
extern crate dotenv;


extern crate diesel_migrations;
use diesel_migrations::{MigrationHarness};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use handlers::handlers::{add_user, get_public_key, greet};
use crate::handlers::handlers::{create_vote, get_votes_by_user};
use crate::repository::database::establish_connection;


#[derive(Serialize)]
pub struct Response {
    status: String,
    message: String,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    dotenv::dotenv().ok();let pool = establish_connection();


    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(greet))
            .route("/test", web::get().to(test))
            .route("/user", web::post().to(add_user))
            .route("/user", web::get().to(add_user))
            .route("/public_key", web::get().to(get_public_key))
            .route("/vote", web::post().to(create_vote))
            .route("/getVotes", web::get().to(get_votes_by_user))

    })
    .bind("127.0.0.1:9090")?
    .run()
    .await
}

async fn test() -> impl Responder {

    HttpResponse::Ok().body("Hello world!");
    println!("TEST /test");
    "test"
}

// async fn create_user(pool: web::Data<database::DbPool>, new_user: web::Json<NewUser>) -> impl Responder {
//
//     let conn = pool.get().expect("Couldn't get db connection from pool");
//     let new_user = new_user.into_inner();
//
//     match new_user.save(&conn) {
//         Ok(_) => HttpResponse::Ok().body("User created"),
//         Err(err) => HttpResponse::InternalServerError().body(format!("Failed to create user: {}", err)),
//     }
//
//     HttpResponse::Ok().body("User created")
// }

// async fn get_public_key(data: web::Data<Mutex<Arc<ServerState>>>) -> impl Responder {
//     println!("PUBLIC KEY!");
//
//     let state = data.lock().unwrap();
//     let public_key_pem = state.public_key. to_pkcs1_pem().unwrap();
//     return HttpResponse::Ok().json(PublicKeyResponse { public_key: public_key_pem });
//
// }



// fn main() {
    // println!("START");
    // let tx1 = Transaction {
    //     sender: "Alice".to_string(),
    //     recipient: "Bob".to_string(),
    //     value: 1,
    // };

    // let tx2 = Transaction {
    //     sender: "Charlie".to_string(),
    //     recipient: "Dave".to_string(),
    //     value: 2,
    // };

    // let previous_hash = "abc123".to_string(); // Пример предыдущего хеша
    // let validator_signature = "validator_signature_here".to_string(); // Пример подписи валидатора
    // let block = Block::new(vec![tx1, tx2], previous_hash, validator_signature);

    // println!("{:#?}", block);
// }


// fn initVote() {
//     println!("INIT VOTE")
// }




    /*
    println!("Guess the number!");
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=1000);

    io::stdin()
        .read_line(&mut guess)
        .expect("fail");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!, {secret_number}"),
        Ordering::Greater => println!("Too big!, {secret_number}"),
        Ordering::Equal => println!("You win!"),
    }
    */