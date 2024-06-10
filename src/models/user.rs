use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset, SqliteConnection, RunQueryDsl, Identifiable};
use crate::models::schema::users;


#[derive(Queryable, Serialize, Deserialize,Debug,Clone,AsChangeset,Insertable, Identifiable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub mail: String,
    pub password: String,
    pub timestamp: std::time::SystemTime,
}

#[derive(Deserialize)]
pub struct NewUserRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize, Serialize,Debug,Clone,Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub mail: String,
    pub password: String,
}

impl NewUser {
    pub fn from_request(req: NewUserRequest) -> Self {
        NewUser {
            name: req.username,
            mail: req.email,
            password: req.password,
        }
    }

    pub fn save(&self, conn: &mut SqliteConnection) -> Result<usize, diesel::result::Error> {
        diesel::insert_into(users::table)
            .values(self)
            .execute(conn)
    }
}