extern crate futures;
extern crate hyper;
#[macro_use]
extern crate serde_json;
extern crate quikdecision;

use futures::future;
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, Server, StatusCode};

use quikdecision::{coin,dice,oracle,percent,pick,select};
use quikdecision::{Command,Decision,Decider};

use std::collections::HashMap;

/// We need to return different futures depending on the route matched,
/// and we can do that with an enum, such as `futures::Either`, or with
/// trait objects.
///
/// A boxed Future (trait object) is used as it is easier to understand
/// and extend with more types. Advanced users could switch to `Either`.
type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn process_command(cmdres: Result<Command,String>, response: &mut Response<Body>)
{
    match cmdres
    {
        Ok(cmd) => {
            let json = match cmd.decide()
            {
                Decision::Text(value) => { json!({ "value": value }) },
                Decision::LabelledText{label, value} => { json!({ "label": label, "value": value }) },
                Decision::Num(value) => { json!({ "value": value }) },
                Decision::AnnotatedNum{value, extra} => { json!({ "value": value, "extra": extra }) },
                Decision::Bool(value) => { json!({ "value": value }) },
            };
            *response.body_mut() = Body::from(json.to_string());
        },
        Err(msg) => {
            *response.body_mut() = Body::from(json!({ "error": msg }).to_string());
        },
    }
}

fn query_params(opt_query: Option<&str>) -> HashMap<&str,&str>
{
    let mut params = HashMap::new();
    if let Some(query) = opt_query
    {
        for p in query.split("&")
        {
            let kv = p.split("=").take(2).collect::<Vec<&str>>();
            if kv.len() == 2 { params.insert(kv[0], kv[1]); }
        }
    }
    params
}

fn report_error(response: &mut Response<Body>, msg: &str)
{
    *response.body_mut() = Body::from(json!({ "error": msg }).to_string())
}

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
fn echo(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        }

        // Flip a coin
        (&Method::GET, "/flip") => {
            process_command(coin::command(), &mut response);
        }

        // Roll dice
        (&Method::GET, "/roll") => {
            let params = query_params(req.uri().query());
            match params.get("expr")
            {
                Some(expr) => process_command(dice::command(expr.to_string()), &mut response),
                None => report_error(&mut response, "Missing required 'expr'."),
            }
        }

        // Ask the Oracle
        (&Method::GET, "/oracle") => {
            process_command(oracle::command(), &mut response);
        }

        // Percent likely
        (&Method::GET, "/likely") => {
            match percent_params(req.uri().query())
            {
                Ok(percent) => process_command(percent::command(percent), &mut response),
                Err(msg) => report_error(&mut response, msg),
            }
        }

        // Pick Number
        (&Method::GET, "/pick") => {
            match pick_params(req.uri().query())
            {
                Ok((low, high)) => process_command(pick::command(low, high), &mut response),
                Err(msg) => report_error(&mut response, msg),
            }
        }

        // Select string
        (&Method::GET, "/select") => {
            match select_params(req.uri().query())
            {
                Ok(strvec) => process_command(select::command(strvec), &mut response),
                Err(msg) => report_error(&mut response, msg),
            }
        }

        // The 404 Not Found route...
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Box::new(future::ok(response))
}

fn percent_params(opt_query: Option<&str>) -> Result<u32, &str>
{
    let params = query_params(opt_query);
    match params.get("percent")
    {
        Some(percent) => match percent.parse::<u32>()
        {
            Ok(p) => Ok(p),
            Err(_) => Err("'percent' value must be an integer"),
        },
        None => Err("Missing required 'percent'."),
    }
}

fn pick_params(opt_query: Option<&str>) -> Result<(i32, i32), &str>
{
    let params = query_params(opt_query);
    match (params.get("low"), params.get("high"))
    {
        (Some(low), Some(high)) => {
            match (low.parse::<i32>(), high.parse::<i32>())
            {
                (Ok(l), Ok(h)) => Ok((l, h)),
                (Err(_), _) => Err("'low' value must be an integer"),
                (_, Err(_)) => Err("'high' value must be an integer"),
            }
        },
        (None, None) => Err("Missing required 'low' and 'high'."),
        (None, _) => Err("Missing required 'low'."),
        (_, None) => Err("Missing required 'high'."),
    }
}

fn select_params(opt_query: Option<&str>) -> Result<Vec<String>, &str>
{
    let params = query_params(opt_query);
    match params.get("strings")
    {
        Some(strings) => Ok(strings.split(",").map(|s| s.to_string()).collect::<Vec<String>>()),
        None => Err("Missing required 'strings'."),
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
