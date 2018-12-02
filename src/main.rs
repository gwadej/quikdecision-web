extern crate futures;
extern crate hyper;
extern crate quikdecision;
extern crate qdweb;

use futures::future;
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::fs::File;
use std::io::prelude::*;

use quikdecision::{coin,dice,oracle,percent,pick,select};
//use quikdecision::{Command,Decision,Decider};

use qdweb::*;

/// We need to return different futures depending on the route matched,
/// and we can do that with an enum, such as `futures::Either`, or with
/// trait objects.
///
/// A boxed Future (trait object) is used as it is easier to understand
/// and extend with more types. Advanced users could switch to `Either`.
type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
fn echo(req: Request<Body>) -> BoxFut {
    let response =
    match (req.method(), req.uri().path())
    {
        // Serve some instructions at /
        (&Method::GET, "/") => {
            match load_file("static/quikdecision.html")
            {
                Ok(content) => {
                    Response::builder()
                        .header("Content-Type", "text/html")
                        .body(Body::from(content))
                        .unwrap()
                },
                Err(msg) => report_error(&msg),
            }
        }

        // Flip a coin
        (&Method::GET, "/flip") => {
            process_command(coin::command())
        }

        // Roll dice
        (&Method::GET, "/roll") => {
            let params = query_params(req.uri().query());
            match params.get("expr")
            {
                Some(expr) => process_command(dice::command(expr.to_string())),
                None => report_error("Missing required 'expr'."),
            }
        }

        // Ask the Oracle
        (&Method::GET, "/oracle") => {
            process_command(oracle::command())
        }

        // Percent likely
        (&Method::GET, "/likely") => {
            match percent_params(req.uri().query())
            {
                Ok(percent) => process_command(percent::command(percent)),
                Err(msg) => report_error(msg),
            }
        }

        // Pick Number
        (&Method::GET, "/pick") => {
            match pick_params(req.uri().query())
            {
                Ok((low, high)) => process_command(pick::command(low, high)),
                Err(msg) => report_error(msg),
            }
        }

        // Select string
        (&Method::GET, "/select") => {
            match select_params(req.uri().query())
            {
                Ok(strvec) => process_command(select::command(strvec)),
                Err(msg) => report_error(msg),
            }
        }

        // The 404 Not Found route...
        _ => {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Unknown decision command"))
                .unwrap()
        }
    };

    Box::new(future::ok(response))
}

fn load_file(name: &str) -> Result<String,String>
{
    let mut file = match File::open(name)
    {
        Ok(file) => file,
        Err(_) => return Err(format!("Cannot open file: '{}'", name)),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents)
    {
        Ok(_) => Ok(contents),
        Err(_) => Err(format!("Failure reading file: '{}'", name)),

    }
}

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn(echo))
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);
    hyper::rt::run(server);
}
