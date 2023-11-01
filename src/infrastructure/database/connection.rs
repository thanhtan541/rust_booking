// use diesel::prelude::*;
// use diesel::r2d2::ConnectionManager;
// use diesel::r2d2::Pool;
// use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
// use std::thread;

// pub type PgPool = Pool<ConnectionManager<PgConnection>>;

// pub fn establish_connection(database_url: &str) -> Pool<ConnectionManager<DbConnection>> {
//     let url = "some_string";
//     let manager = ConnectionManager::<DbConnection>::new(url);
//     // Refer to the `r2d2` documentation for more methods to use
//     // when building a connection pool
//     Pool::builder()
//         .max_size(1)
//         .test_on_check_out(true)
//         .connection_customizer(Box::new(SetupUserTableCustomizer))
//         .build(manager)
//         .expect("Could not build connection pool")
// }
