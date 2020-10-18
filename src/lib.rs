extern crate hyper;
extern crate quikdecision;
#[macro_use]
extern crate serde_json;
extern crate percent_encoding;

use eyre::eyre;
use std::collections::HashMap;
use hyper::{Body,  Response, StatusCode};
use percent_encoding::percent_decode;

use quikdecision::{Command, Decision, Decider, Result};

pub fn process_command(cmdres: Result<Command>) -> Response<Body>
{
    let mut builder = Response::builder();
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
                Decision::List(strings) => { json!({ "list": strings }) }
                Decision::Card(card) => {
                    json!({
                        "value":  card.to_string(),
                        "suit":   card.suit(),
                        "number": card.value(),
                        "glyph":  card.glyph(),
                    })
                },
            }.to_string())
        },
        Err(msg) => {
            builder.status(StatusCode::BAD_REQUEST);
            Body::from(json!({ "error": msg.to_string() }).to_string())
        },
    };
    builder
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap()
}

fn url_decode(input: &str) -> String
{
    percent_decode(&(input.bytes().collect::<Vec<u8>>()))
        .decode_utf8()
        .unwrap()
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
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from(json!({ "error": msg }).to_string()))
        .unwrap()
}

pub fn percent_params(opt_query: Option<&str>) -> eyre::Result<u32>
{
    extract_int::<u32>(&query_params(opt_query), "percent")
}

fn extract_int<T>(params: &HashMap<&str,String>, key: &str) -> eyre::Result<T>
    where T: std::str::FromStr
{
    params.get(key)
        .ok_or_else(|| eyre!("Missing required '{}'.", key))
        .and_then(|l| l.parse::<T>()
                  .map_err(|_| eyre!("'{}' value must be an integer", key)))
}

pub fn pick_params(opt_query: Option<&str>) -> eyre::Result<(i32, i32)>
{
    let params = query_params(opt_query);
    match (extract_int::<i32>(&params, "low"),
            extract_int::<i32>(&params, "high"))
    {
        (Ok(l), Ok(h)) => Ok((l, h)),
        (Err(e), _) | (_, Err(e)) => Err(e),
    }
}

fn split_strings(strings: &str) -> Vec<String>
{
    let sep = if strings.contains("\n") { "\n" } else { "," };
    strings.split(sep)
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

pub fn select_params(opt_query: Option<&str>) -> eyre::Result<Vec<String>>
{
    query_params(opt_query).get("strings")
        .map(|s| split_strings(s))
        .ok_or_else(|| eyre!("Missing required 'strings'."))
}
