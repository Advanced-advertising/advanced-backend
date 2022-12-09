use actix::{Actor, SyncContext};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use r2d2::PooledConnection;
use slog::{crit, Logger, o};
use crate::errors::AppError;

pub struct DbActor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbActor {
    type Context = SyncContext<Self>;
}

pub fn get_pooled_connection(
    pool: &Pool<ConnectionManager<PgConnection>>,
    logger: Logger
) -> Result<PooledConnection<ConnectionManager<PgConnection>>, AppError> {
    pool.get().map_err(|err: PoolError| {
        let sub_log = logger.new(o!("cause" => err.to_string()));
        crit!(sub_log, "Error getting pooled connection");
        AppError::from(err)
    })
}
