#![feature(proc_macro)]
extern crate iron;
extern crate router;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::string::String;

use iron::prelude::*;
use iron::status;
use router::Router;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    age: Option<u8>
}

fn list_users() -> String {
    let u1 = User { name: "user 1".to_string(), age: Some(34u8) };
    let u2 = User { name: "user 2".to_string(), age: Some(12u8) };
    let u3 = User { name: "user 3".to_string(), age: None };
    let users = vec![u1, u2, u3];
    serde_json::to_string(&users).unwrap_or("[]".to_string())
}


fn get_user(user: String) -> String {
    let u = User { name: user, age: Some(34u8) };
    serde_json::to_string(&u).unwrap_or("null".to_string())
}

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "index");
    router.get("/:query", handler, "query");

    Iron::new(router).http("localhost:3000").unwrap();

    fn handler(req: &mut Request) -> IronResult<Response> {
        match req.extensions.get::<Router>().unwrap().find("query") {
            Some(query) => Ok(Response::with((status::Ok, get_user(query.to_string())))),
            None => Ok(Response::with((status::Ok, list_users()))) 
        }        
    }
}
