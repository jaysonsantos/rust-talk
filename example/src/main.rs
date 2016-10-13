#![feature(proc_macro)]
extern crate iron;
extern crate router;

#[macro_use] extern crate lazy_static;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate dotenv;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel;

extern crate bmemcached;

mod schema;
mod models;

use std::env;
use std::string::String;

use iron::prelude::*;
use iron::status;
use router::Router;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;

use bmemcached::MemcachedClient;

use models::User;

lazy_static! {
    static ref CACHE: MemcachedClient = {
        dotenv().ok();
        MemcachedClient::new(vec![&*env::var("MEMCACHED").expect("Memcached configuration not found")], 10).expect("Could not connect to memcached.")
    };
}

fn get_db_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn list_users() -> String {
    use schema::users::dsl::*;
    let cached_users: Result<String, _> = CACHE.get("users");
    match cached_users {
        Ok(json) => return json,
        Err(_) => {}
    }
    let conn = get_db_connection();
    let db_users = users.limit(5).load::<User>(&conn).expect("Error loading users");
    let json = serde_json::to_string(&db_users).unwrap_or("[]".to_string());
    CACHE.set("users", &json, 50000).unwrap();
    json
}

fn get_user(user: String) -> String {
    use schema::users::dsl::*;
    let cache_key = format!("get_user_{}", user);
    let cached_user: Result<String, _> = CACHE.get(&cache_key);
    match cached_user {
        Ok(json) => return json,
        Err(_) => {}
    }
    let conn = get_db_connection();
    let user_list = users.filter(name.like(&format!("%{}%", user))).limit(1).load::<User>(&conn).expect("Error loading user");
    let db_user = user_list.iter().nth(0);
    let json = serde_json::to_string(&db_user).unwrap_or("null".to_string());
    CACHE.set(&cache_key, &json, 50000).unwrap();
    json
}

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "index");
    router.get("/:query", handler, "query");

    Iron::new(router).http("0.0.0.0:3000").unwrap();

    fn handler(req: &mut Request) -> IronResult<Response> {
        match req.extensions.get::<Router>().unwrap().find("query") {
            Some(query) => Ok(Response::with((status::Ok, get_user(query.to_string())))),
            None => Ok(Response::with((status::Ok, list_users()))) 
        }        
    }
}
