extern crate hyper;
extern crate rustc_serialize;

use rustc_serialize::json;
use hyper::Client;
use hyper::header::{Headers, ContentType};

#[derive(RustcEncodable, RustcDecodable)]
struct Incoming {
    text : String,
    markdown : String,
    attachments : Vec<IncomingAttachment>,
}

#[derive(RustcEncodable, RustcDecodable)]
struct IncomingAttachment {
    text : String,
    color : String,
    images : Vec<AttachmentImage>,
}

#[derive(RustcEncodable, RustcDecodable)]
struct AttachmentImage {
    url : String,
}

fn main() {
    let url = "https://hook.bearychat.com/";
    let client = Client::new();
    let incoming = Incoming {
        text : "Hello **rust**".to_string(),
        markdown : "true".to_string(),
        attachments : vec![
            IncomingAttachment {
                text : "**BearyChat**".to_string(),
                color : "#86b4de".to_string(),
                images : vec![
                    AttachmentImage {
                        url : "https://cdn.bearychat.com/94030a9693952e9f7e769a5c61d2dcfb.png".to_string(),
                    }
                ]
            }
        ]
    };

    let encoded = json::encode(&incoming).unwrap();
    println!("{}", encoded);

    let res = client.post(url)
                    .header(ContentType::json())
                    .body(&encoded)
                    .send().unwrap();
}
