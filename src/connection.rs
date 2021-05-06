use std::env;
use std::ops::Deref;

use diesel::pg::PgConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;


pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    Pool::new(manager).expect("db pool");
}

fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub struct DBConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

imp<a',r'> FromRequest for DBConn {
    type Error = ();
    
    fn from_request(request: &'a Request<'r>) -> request::Outcome<DBConn, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DBConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DBConn {
    type Target = PgConnection;
    
    fn deref(&self) -> &Self::Target {
        &self.
    }
}