extern crate hyper;
extern crate quikdecision;
#[macro_use]
extern crate serde_json;
extern crate percent_encoding;

use std::collections::HashMap;
use hyper::{Body,  Response};
use percent_encoding::percent_decode;

use quikdecision::{Command, Decision, Decider};

pub fn process_command(cmdres: Result<Command,String>) -> Response<Body>
{
    let body = match cmdres
    {
        Ok(cmd) => {
            Body::from(match cmd.decide()
            {
                Decision::Text(value) => { json!({ "value": value }) },
                Decision::LabelledText{label, value} => { json!({ "label": label, "value": value }) },
                Decision::Num(value) => { json!({ "value": value }) },
                Decision::AnnotatedNum{value, extra} => { json!({ "value": value, "extra": extra }) },
                Decision::Bool(value) => { json!({ "value": value }) },
            }.to_string())
        },
        Err(msg) => Body::from(json!({ "error": msg }).to_string()),
    };
    Response::builder()
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap()
}

fn url_decode(input: &str) -> String
{
    percent_decode(&(input.bytes().collect::<Vec<u8>>()))
        .decode_utf8()
        .unwrap()
        .to_owned()
        .to_string()
}

pub fn query_params(opt_query: Option<&str>) -> HashMap<&str,String>
{
    let mut params = HashMap::new();
    if let Some(query) = opt_query
    {
        for p in query.split("&")
        {
            let kv = p.split("=").take(2).collect::<Vec<&str>>();
            if kv.len() == 2 { params.insert(kv[0], url_decode(kv[1])); }
        }
    }
    params
}

pub fn report_error(msg: &str) -> Response<Body>
{
    Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::from(json!({ "error": msg }).to_string()))
        .unwrap()
}

pub fn percent_params(opt_query: Option<&str>) -> Result<u32, &str>
{
    query_params(opt_query)
        .get("percent")
        .ok_or("Missing required 'percent'.")
        .and_then(|percent|
            percent.parse::<u32>()
                   .map_err(|_| "'percent' value must be an integer"))
}

pub fn pick_params(opt_query: Option<&str>) -> Result<(i32, i32), &str>
{
    let params = query_params(opt_query);
    match (params.get("low"), params.get("high"))
    {
        (Some(low), Some(high)) => {
            match (low.parse::<i32>().map_err(|_| "'low' value must be an integer"),
                   high.parse::<i32>().map_err(|_| "'high' value must be an integer"))
            {
                (Ok(l), Ok(h)) => Ok((l, h)),
                (Err(e), _) | (_, Err(e))=> Err(e),
            }
        },
        (None, None) => Err("Missing required 'low' and 'high'."),
        (None, _) => Err("Missing required 'low'."),
        (_, None) => Err("Missing required 'high'."),
    }
}

fn split_strings(strings: &str) -> Vec<String>
{
    let sep = if strings.contains("\n") { "\n" } else { "," };
    strings.split(sep).map(|s| s.to_string()).collect::<Vec<String>>()
}

pub fn select_params(opt_query: Option<&str>) -> Result<Vec<String>, &str>
{
    query_params(opt_query).get("strings")
        .map(|s| split_strings(s))
        .ok_or("Missing required 'strings'.")
}
