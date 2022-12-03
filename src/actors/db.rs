use actix::{Actor, SyncContext};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct DbActor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbActor {
    type Context = SyncContext<Self>;
}