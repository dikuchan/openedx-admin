use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
};

/*
  Read migrations at compile time, without migration files being present on the file system.
 */
embed_migrations!();

/*
  Use PostgreSQL connection pool.
 */
pub type Connection = PgConnection;
pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

pub fn migrate(url: &str) -> Pool {
    let manager = ConnectionManager::<Connection>::new(url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create the connection pool.");
    embedded_migrations::run(&pool.get().expect("Failed to migrate."));

    pool
}
