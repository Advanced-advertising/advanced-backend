use actix::{Actor, Handler, Message};
use argonautica::Hasher;
use diesel::prelude::*;
use uuid::Uuid;
use crate::actors::db::DbActor;
use crate::models::user::User;
use crate::schema::users::dsl::{users, user_id};

#[derive(Message)]
#[rtype(result="QueryResult<User>")]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Message)]
#[rtype(result="QueryResult<User>")]
pub struct UpdateUser {
    pub uuid: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl Handler<CreateUser> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Unable to get a connection");

        let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
        let mut hasher = Hasher::default();
        let password_hash = hasher
            .with_password(msg.password)
            .with_secret_key(hash_secret)
            .hash()
            .unwrap();

        let new_user = User {
            user_id: Uuid::new_v4(),
            user_name: msg.name,
            email: msg.email,
            password: password_hash,
        };

        diesel::insert_into(users)
            .values(new_user)
            .get_result::<User>(&mut conn)
    }
}