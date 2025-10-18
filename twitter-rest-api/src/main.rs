use std::{io};
use actix_web::{middleware, App, HttpServer};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::{Pool, PooledConnection};

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env_logger::init();
    let database_url = "postgres://postgres:admin@localhost:5432/betta";
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool");
    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .wrap(middleware::Logger::default())
    }).bind("0.0.0.0:8080")?
    .run()
    .await
}
