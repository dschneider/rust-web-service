extern crate iron;
extern crate router;

use std::net::SocketAddr;
use std::str::FromStr;
use iron::{Iron, Request, Response, IronResult};
use iron::status;
use iron::mime::{Mime};
use iron::headers::{ContentType};
use iron::modifiers::{Header};
use router::{Router};
use std::env;

fn json_mime() -> Mime {
  "application/json".parse().unwrap()
}

fn main() {
  let mut router = Router::new();

  let health_handler = |_request: &mut Request| -> IronResult<Response> {
    Ok(Response::with((status::Ok, "{\"up\":true}", Header(ContentType(json_mime())))))
  };

  router.get("/api/health", health_handler);

  match env::var("PORT") {
    Ok(port_num) => {
      match Iron::new(router).http(&*format!("0.0.0.0:{}", port_num)) {
        Ok(server_listener) => println!("Started! Yay!"),
        Err(_) => println!("Shit's on fire, again, yo.")
      }
    },
    Err(_) => println!("Shit's on fire, yo.")
  }
}
