use std::error::Error;

use diesel::prelude::*;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::r2d2::PooledConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::time::Duration;

use crate::context::GraphQLContext;

pub type ConnectionMgr = ConnectionManager<MysqlConnection>;
pub type SqlitePool = Pool<ConnectionManager<MysqlConnection>>;

#[derive(Debug)]
pub struct ConnectionOptions {
    pub busy_timeout: Option<Duration>,
}

// impl diesel::r2d2::CustomizeConnection<MysqlConnection, diesel::r2d2::Error> for ConnectionOptions {
// fn on_acquire(&self, conn: &mut MysqlConnection) -> Result<(), diesel::r2d2::Error> {
// (|| {
//     if let Some(d) = self.busy_timeout {
//         sql_query(format!("PRAGMA busy_timeout = {};", d.as_millis())).execute(conn)?;
//     }
//     Ok(())
// })()
// .map_err(diesel::r2d2::Error::QueryError)
// }
// }

pub fn get_pool() -> SqlitePool {
    use dotenvy::dotenv;
    use std::env;
    dotenv().ok();
    let url = env::var("DATABASE_URL").unwrap();
    let mgr = ConnectionManager::<MysqlConnection>::new(url);
    r2d2::Pool::builder()
        .min_idle(Some(3))
        .max_size(20)
        // .connection_customizer(Box::new(ConnectionOptions {
        //     busy_timeout: Some(Duration::from_secs(2)),
        // }))
        .build(mgr)
        .expect("could not build connection pool")
}

pub(crate) fn get_conn(context: &GraphQLContext) -> PooledConnection<ConnectionMgr> {
    context.pool.get().expect("Could not get db connection")
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
pub fn run_migrations(
    connection: &mut impl MigrationHarness<diesel::mysql::Mysql>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}
