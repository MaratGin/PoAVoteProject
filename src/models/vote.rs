use diesel::associations::HasTable;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::models::schema::vote_validators;
use super::schema::votes;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "votes"]
pub struct Vote {
    pub vot_id: i32,
    pub name: String,
    pub owner_id: i32,
    pub is_finished: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "votes"]
pub struct NewVote {
    pub name: String,
    pub owner_id: i32,
    pub is_finished: bool,
}
//
impl NewVote {
    pub fn save(&self, conn: &mut SqliteConnection) -> Result<usize, diesel::result::Error> {
        diesel::insert_into(votes::table)
            .values(self)
            .execute(conn)
    }
}

#[derive(Deserialize)]
pub struct NewVoteRequest {
    pub ownerName: String,
    pub title: String,
    pub options: Vec<String>,
    pub userNames: Vec<String>,
    pub validatorNames: Vec<String>,
    pub isOpen: bool,
    pub isMulti: bool,
}

#[derive(Serialize, Deserialize)]
pub struct VoteValidator {
    pub vote_id: i32,
    pub val_endpoint: String,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "vote_validators"]
pub struct NewVoteValidator {
    pub vote_id: i32,
    pub val_endpoint: String,
}

impl NewVoteValidator {
    pub fn save(&self, conn: &mut SqliteConnection) -> Result<usize, diesel::result::Error> {
        diesel::insert_into(vote_validators::table)
            .values(self)
            .execute(conn)
    }
}
// с фронтенда в качестве post запроса на /votes приходит json такого формата:
// {
// isMulti: boolean,
// isOpen: boolean,
// options: {
// }
// }

