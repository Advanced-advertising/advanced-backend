use actix::{Addr, SyncArbiter};
use serde::Deserialize;
use slog::{o, Drain, Logger};
use slog_async;
use slog_envlogger;
use slog_term;
use crate::actors::db::DbActor;
use crate::config;
use crate::db_utils::get_pool;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

pub struct Config {
    pub server: ServerConfig,
    pub db: Addr<DbActor>,
}

impl Config {
    pub fn from_env() -> Self {
        let db_url = dotenv::var("DATABASE_URL").expect("Error retrieving the database url");
        // run_migrations(&db_url);
        let pool = get_pool(&db_url);
        let db_addr = SyncArbiter::start(8, move || DbActor(pool.clone()));

        Self {
            server: ServerConfig { host: "0.0.0.0".parse().unwrap(), port: 4000 },
            db: db_addr,
        }
    }

    pub fn configure_log() -> Logger {
        let decorator = slog_term::TermDecorator::new().build();
        let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
        let console_drain = slog_envlogger::new(console_drain);
        let console_drain = slog_async::Async::new(console_drain).build().fuse();
        Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
    }
}
