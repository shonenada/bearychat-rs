extern crate bearychat;

use bearychat::rtm::client::{RTMClient};

fn main() {
    let client = RTMClient {
        token: "hubot-token".to_string(),
        client: None
    };

    let members = client.members().unwrap();
    for each in members {
        println!("{}", each);
    }

}
