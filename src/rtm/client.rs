use std::io::Read;

use rtm::utils::{http_get, parse_json};
use rtm::data_structure::{Team, User, Channel};

static BASE_URL: &'static str = "https://rtm.bearychat.com/v1";

pub struct RTMClient {
    pub token: String,
}

impl RTMClient {
    pub fn team(&self) -> Option<Team> {
        let url = format!("{}{}?token={}", BASE_URL, "/current_team.info", self.token);
        let mut resp = http_get(url.as_str()).unwrap();
        let mut buffer = String::new();
        resp.read_to_string(&mut buffer);
        let team_data= parse_json(buffer);
        match team_data {
            Ok(data) => {
                let code = data["code"].as_i32().unwrap();
                if code == 0 {
                    let team_data = data["result"].clone();
                    let team = Team::parse_from_json(team_data);
                    return Some(team)
                } else {
                    let error = data["error"].clone();
                    panic!("Failed to get team. {}", error);
                }
            },
            Err(e) => {
                panic!("Failed to get team. {}", e);
            }
        }
    }

    pub fn members(&self) -> Option<Vec<User>> {
        let url = format!("{}{}?token={}", BASE_URL, "/current_team.members", self.token);
        let mut resp = http_get(url.as_str()).unwrap();
        let mut buffer = String::new();
        resp.read_to_string(&mut buffer);
        let members_data = parse_json(buffer);
        match members_data {
            Ok(data) => {
                let code = data["code"].as_i32().unwrap();
                if code == 0 {
                    let mut members = Vec::new();
                    let mut rv = data["result"].clone();
                    let size = rv.len();
                    for i in 0..size {
                        let obj = User::parse_from_json(rv[i].take());
                        members.push(obj);
                    }
                    return Some(members)
                } else {
                    let error = data["error"].clone();
                    panic!("Failed to get members. {}", error);
                }
            },
            Err(e) => {
                panic!("Failed to get team membres. {}", e);
            }
        }
    }

    pub fn channels(&self) -> Option<Vec<Channel>> {
        let url = format!("{}{}?token={}", BASE_URL, "/current_team.channels", self.token);
        let mut resp = http_get(url.as_str()).unwrap();
        let mut buffer = String::new();
        resp.read_to_string(&mut buffer);
        let channels_data = parse_json(buffer);
        match channels_data {
            Ok(data) => {
                let code = data["code"].as_i32().unwrap();
                if code == 0 {
                    let mut channels = Vec::new();
                    let mut rv = data["result"].clone();
                    let size = rv.len();
                    for i in 0..size {
                        let obj = Channel::parse_from_json(rv[i].take());
                        channels.push(obj);
                    }
                    return Some(channels)
                } else {
                    let error = data["error"].clone();
                    panic!("Failed to get channels. {}", error);
                }
            },
            Err(e) => {
                panic!("Failed to get team channels. {}", e);
            }
        }
    }
}
