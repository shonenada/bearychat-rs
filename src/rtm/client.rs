use std::io::Read;

use ws::{connect, Handler, Sender, Handshake};
use rtm::utils::{http_get, parse_json, get_ws_url};
use rtm::data_structure::{Team, User, Channel};

static BASE_URL: &'static str = "https://rtm.bearychat.com/v1";

pub struct RTMClient {
    pub token: String,
    pub client: Option<Sender>,
}

impl RTMClient {
    pub fn team(&self) -> Option<Team> {
        let url = format!("{}{}?token={}", BASE_URL, "/current_team.info", self.token);
        let mut resp = http_get(url.as_str()).unwrap();
        let mut buffer = String::new();
        resp.read_to_string(&mut buffer);
        let team_resp = parse_json(buffer);
        match team_resp {
            Ok(team_resp) => {
                let code = team_resp["code"].as_i32().unwrap();
                if code == 0 {
                    let team_data = team_resp["result"].clone();
                    let team = Team::parse_from_json(team_data);
                    return Some(team)
                } else {
                    let error = team_resp["error"].clone();
                    panic!("Failed to get team. {}", error);
                    return None
                }
            },
            Err(e) => {
                panic!("Failed to get team. {}", e);
                return None
            }
        }
    }

    pub fn members(&self) -> Option<User> {
        let url = format!("{}{}", BASE_URL, "/current_team.members");
        let mut resp = http_get(url.as_str()).unwrap();
        let mut buffer = String::new();
        resp.read_to_string(&mut buffer);
        let members_resp = parse_json(buffer);
        match members_resp {
            Ok(members_resp) => {
                None
            },
            Err(e) => {
                println!("Failed to get team membres. {}", e);
                None
            }
        }
    }

    pub fn channels(&self) {
        let url = format!("{}{}", BASE_URL, "/current_team.channels");
        let mut resp = http_get(url.as_str());
    }

    fn get_user(&self, uid: i32) {
    }

    fn get_channel(&self, channel_id: i32) {
    }

    fn send_message(&self, message: String) {
    }

    fn start(&self) {
    }
}
