extern crate bearychat;

use bearychat::rtm::client::{RTMClient};

fn main() {
    let client = RTMClient {
        token: "hubot-token".to_string(),
    };

    let channels = client.channels().unwrap();
    for each in channels {
        println!("{}", each);
    }

}
