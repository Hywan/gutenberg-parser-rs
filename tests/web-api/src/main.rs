extern crate gutenberg_post_parser;
extern crate actix_web;

use actix_web::{
    App,
    Responder,
    http::Method,
    server,
};
use std::env;

fn serve_parse(body: String) -> impl Responder {
    match gutenberg_post_parser::root(body.as_bytes()) {
        Ok((_remaining, nodes)) => {
            format!("Parsed {} nodes.", nodes.len())
        },

        _ => {
            format!("Failed to parse.")
        }
    }
}

fn main() {
    let server_address = match env::var("GUTENBERG_TEST_SERVER_ADDRESS") {
        Ok(value) => value,
        _ => panic!("The `GUTENBERG_TEST_SERVER_ADDRESS` environment variable is not defined. The server cannot run.")
    };

    println!("Listening {}.", &server_address);

    server
        ::new(
            || {
                App::new()
                    .resource(
                        "/",
                        |resource| {
                            resource.method(Method::POST).with(serve_parse)
                        }
                    )
            }
        )
        .bind(&server_address)
        .expect(&format!("Cannot bind the server to {}.", &server_address))
        .shutdown_timeout(30)
        .run();
}
