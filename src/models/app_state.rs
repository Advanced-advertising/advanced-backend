use crate::actors::db::DbActor;
use actix::Addr;
use slog::Logger;

pub struct AppState {
    pub db: Addr<DbActor>,
    pub logger: Logger,
}
