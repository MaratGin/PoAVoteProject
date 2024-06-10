use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;


// use crate::models::user::{User, NewUser};
// use crate::models::schema::users::dsl::*;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv().ok();
    let database_url = "sqlite://poa_vote.db";
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}

// impl Database { 
    // pub fn new() -> Self {
    //     dotenv().ok();
    //     let database_url = std::env::var("sqlite://poa_vote.db").expect("DATABASE_URL must be set");
        
    //     let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    //     let result = r2d2::Pool::builder()
    //         .build(manager)
    //         .expect("Failed to create pool.");
        
    //     Database { pool: result }
    // }

    // pub fn get_events(&self) -> Vec<Event> {
        
    //     events
    //         .load::<Event>(&mut self.pool.get().unwrap())
    //         .expect("Failed to get events.")
    // }

    // pub fn get_event(&self, find_id:i32) -> Option<Event> {
    //     events
    //         .find(find_id)
    //         .first::<Event>(&mut self.pool.get().unwrap())
    //         .ok()
    // }

    // pub fn create_user(&self,user:User)->Result<User,diesel::result::Error>{
    //     diesel::insert_into(users).values(&user.into_inner()).get_result(&mut self.pool.get().unwrap())
    // }

    // pub fn delete_event(&self,find_id:i32)->Result<usize,diesel::result::Error>{
    //     diesel::delete(events.filter(id.eq(find_id))).execute(&mut self.pool.get().unwrap())
    // }

    // pub fn update_event(&self,event:Event)->Result<Event,diesel::result::Error>{
    //     diesel::update(events.filter(id.eq(event.id))).set(&event).get_result(&mut self.pool.get().unwrap())
    // }
// }