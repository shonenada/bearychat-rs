use std::io::Read;
use json::{Error as JError, JsonValue, parse};
use hyper::{Client, Error as HError};
use hyper::client::response::{Response};
use hyper::header::{ContentType};

static RTM_URL: &'static str = "https://rtm.bearychat.com/start";

pub fn http_get(url: &str) -> Result<Response, HError> {
    let client = Client::new();
    let resp = client.get(url).send();
    resp
}

pub fn parse_json(json_string: String) -> Result<JsonValue, JError> {
    parse(json_string.as_str())
}

pub fn get_ws_url(token: String) -> String {
    let client = Client::new();
    let body = format!("token={}", token);
    let mut res = client.post(RTM_URL)
                        .header(ContentType::form_url_encoded())
                        .body(&body)
                        .send()
                        .unwrap();

    let mut buffer = String::new();
    res.read_to_string(&mut buffer);

    let mut json_data = parse(&buffer).unwrap();
    json_data["result"]["ws_host"].take_string().unwrap()
}
