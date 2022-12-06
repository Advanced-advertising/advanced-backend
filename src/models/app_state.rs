use actix::Addr;
use slog::Logger;
use crate::actors::db::DbActor;

pub struct AppState {
    pub db: Addr<DbActor>,
    pub logger: Logger,
}