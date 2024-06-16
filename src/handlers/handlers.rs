use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use actix_web::{web,  HttpResponse, Responder};
use reqwest::Client;
use diesel::dsl::select;
use rsa::pkcs1::ToRsaPublicKey;
use diesel::prelude::*;

use crate::genesis_block::{GenesisBlock, OptionInfo};
use crate::key_manager::{PublicKeyResponse, ServerState};
use crate::models::schema::users::dsl::{users, name as user_name, id as user_id};
use crate::models::schema::users::{id, name};
use crate::models::schema::vote_validators;
use diesel::Identifiable;
use serde::{Deserialize, Serialize};


use crate::models::schema::votes::dsl::{votes, vot_id, owner_id as vote_owner_id};
use crate::repository::database::DbPool;

use crate::models::user::{User,NewUser, NewUserRequest};
use crate::models::vote::{NewVote, NewVoteRequest, NewVoteValidator, Vote};

#[derive(Serialize, Deserialize)]
pub struct VoteResponse {
    pub vot_id: i32,
    pub name: String,
    pub owner_id: i32,
    pub is_finished: bool,
}

pub async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello worlddd!");
    println!("TEST PRINT");
    "Main page"
}

pub async fn get_public_key(data: web::Data<Mutex<Arc<ServerState>>>) -> impl Responder {
    println!("PUBLIC KEY!");

    let state = data.lock().unwrap();
    let public_key_pem = state.public_key. to_pkcs1_pem().unwrap();
    return HttpResponse::Ok().json(PublicKeyResponse { public_key: public_key_pem });

}

pub async fn add_user(pool: web::Data<DbPool>, user_request: web::Json<NewUserRequest>)->HttpResponse{
    println!("NEW USEr");
    let new_user = NewUser::from_request(user_request.into_inner());
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    match new_user.save(&mut conn) {
        Ok(_) => HttpResponse::Ok().body("{ response: true }"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Failed to create user: {}", err)),
    }
}

pub async fn test_get()-> impl Responder {
    println!("GET NEW USEr");
    HttpResponse::Ok().body("Hello worlddd!");
    println!("TEST PRINT");
    "Main page"
}

// pub fn init_routes(cfg:&mut web::ServiceConfig){
//     cfg.service(
//         web::scope("/")
//             .service(get_public_key)
//             .service(add_user)
//     );
// }

pub async fn startVoting(data: web::Data<Mutex<Arc<ServerState>>>) -> impl Responder {
    println!("PUBLIC KEY!");
    let state = data.lock().unwrap();
    let public_key_pem = state.public_key. to_pkcs1_pem().unwrap();
    return HttpResponse::Ok().json(PublicKeyResponse { public_key: public_key_pem });

}

pub async fn create_vote(pool: web::Data<DbPool>, vote_request: web::Json<NewVoteRequest>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    // Использование имени владельца для поиска id пользователя
    let owner_name = vote_request.ownerName.clone();

    let owner_id = match users.filter(user_name.eq(owner_name))
        .select(user_id)
        .first::<i32>(&mut conn) {
        Ok(new_id) => new_id ,
        Err(err) => return HttpResponse::InternalServerError().body(format!("Failed to find user ID: {}", err)),
    };


    // Создание новой записи голосования
    let new_vote = NewVote {
        name: vote_request.title.clone(),
        owner_id,
        is_finished: false, // Начальное значение
    };

    if let Err(err) = diesel::insert_into(votes)
        .values(&new_vote)
        .execute(&mut conn) {
        return HttpResponse::InternalServerError().body(format!("Failed to create vote: {}", err));
    }

    // Получение ID последнего вставленного голосования
    let vote_id: i32 = match votes
        .order(vot_id.desc())
        .select(vot_id)
        .first(&mut conn) {
        Ok(found_id) => found_id,
        Err(err) => return HttpResponse::InternalServerError().body(format!("Failed to retrieve vote ID: {}", err)),
    };

    // Создание записей валидаторов голосования
    for validator_name in vote_request.validatorNames.iter() {
        let new_vote_validator = NewVoteValidator {
            vote_id,
            val_endpoint: validator_name.clone(),
        };

        if let Err(err) = diesel::insert_into(vote_validators::table)
            .values(&new_vote_validator)
            .execute(&mut conn) {
            return HttpResponse::InternalServerError().body(format!("Failed to create vote validator: {}", err));
        }
    }

    // Создание GenesisBlock
    let genesis_block = GenesisBlock {
        index: 0,
        hash: String::new(), // Нужно добавить логику для расчета хеша
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
        election_name: vote_request.title.clone(),
        election_description: String::new(), // Заполнить по необходимости
        users: vote_request.userNames.clone(),
        options: vote_request.options.iter().enumerate().map(|(i, option)| OptionInfo { index: i as u64, value: option.clone() }).collect(),
        validators: vote_request.validatorNames.clone(),
        is_open: vote_request.isOpen,
        is_multi: vote_request.isMulti,
    };

    // Отправка GenesisBlock валидаторам
    let validator_urls = vec!["http://127.0.0.1:10098/genesis", "http://127.0.0.1:10037/genesis"];
    for url in validator_urls {
        if let Err(err) = send_genesis_block(&genesis_block, url).await {
            return HttpResponse::InternalServerError().body(format!("Failed to send genesis block to {}: {}", url, err));
        }
    }

    HttpResponse::Ok().body("Vote and validators created")
}
async fn send_genesis_block(genesis_block: &GenesisBlock, url: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();
    client.post(url)
        .json(genesis_block)
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}


pub async fn get_votes_by_user(pool: web::Data<DbPool>, query: web::Query<HashMap<String, String>>) -> impl Responder {
    let user_named = match query.get("user_name") {
        Some(otvet) => otvet.clone(),
        None => return HttpResponse::BadRequest().body("Missing user_name parameter"),
    };

    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    // Найти id пользователя по имени
    let owner = match users.filter(user_name.eq(user_named))
        .select(user_id)
        .first::<i32>(&mut conn) {
        Ok(something) => something ,
        Err(err) => return HttpResponse::InternalServerError().body(format!("Failed to find user ID: {}", err)),
    };

    // Найти все голосования, где пользователь является создателем
    let user_votes = match votes.filter(vote_owner_id.eq(owner))
        .load::<Vote>(&mut conn) {
        Ok(data) => data,
        Err(err) => return HttpResponse::InternalServerError().body(format!("Failed to load votes: {}", err)),
    };

    HttpResponse::Ok().json(user_votes)
}