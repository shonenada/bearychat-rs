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

impl Incoming {
    fn send(&self, url: &str) {
        let client = Client::new();
        let encoded = json::encode(self).unwrap();
        let res = client.post(url)
                        .header(ContentType::json())
                        .body(&encoded)
                        .send();
        match res {
            Ok(mut res) => {
                let mut buffer = String::new();
                res.read_to_string(&mut buffer);
                println!("Res {}: {:?}", res.status, buffer);
            },

            Err(e) => {
                println!("Err: {:?}", e);
            }
        }
    }
}
