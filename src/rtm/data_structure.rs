use std::fmt::{Display, Result, Formatter};

use json::{JsonValue};
use chrono::{UTC, DateTime};

pub struct Team {
    id: String,
    name: String,
    subdomain: String,
    email_domain: String,
    description: String,
    inactive: bool,
    logo_url: String,
//    created: DateTime<UTC>,
//    updated: DateTime<UTC>,
}

impl Display for Team {
    fn fmt(&self, f : &mut Formatter) -> Result {
        write!(f, "id: {:?}\nname: {:?}", self.id, self.name)
    }
}

impl Team {
    pub fn parse_from_json(mut data: JsonValue) -> Team {
        Team {
            id: data["id"].take_string().unwrap(),
            name: data["name"].take_string().unwrap(),
            subdomain: data["subdomain"].take_string().unwrap(),
            email_domain: data["email_domain"].take_string().unwrap(),
            description: data["description"].take_string().unwrap(),
            logo_url: data["logo_url"].take_string().unwrap(),
            inactive: data["inactive"].as_bool().unwrap(),
        }
    }
}

pub struct User {
    id: String,
    name: String,
    email: String,
    team_id: String,
    full_name: String,
    avatar_url: String,
    presence: String,
}

impl User {
    pub fn parse_from_json(mut data: JsonValue) -> User {
        User {
            id: data["id"].take_string().unwrap(),
            name: data["name"].take_string().unwrap(),
            email: data["email"].take_string().unwrap(),
            team_id: data["team_id"].take_string().unwrap(),
            full_name: data["full_name"].take_string().unwrap(),
            avatar_url: data["avatar_url"].take_string().unwrap(),
            presence: data["presence"].take_string().unwrap(),
        }
    }
}

pub struct Channel {
    id: String,
    uid: String,
    name: String,
    topic: String,
    team_id: String,
    vchannel_id: String,
    description: String,
    latest_ts: i64,
    index_symbol: String,
    is_member: bool,
    member_count: i32,
    mention_count: i32,
    general: bool,
    inactive: bool,
    private: bool,
}

impl Channel {
    pub fn parse_from_json(mut data: JsonValue) -> Channel {
        Channel {
            id: data["id"].take_string().unwrap(),
            uid: data["uid"].take_string().unwrap(),
            name: data["name"].take_string().unwrap(),
            topic: data["topic"].take_string().unwrap(),
            team_id: data["team_id"].take_string().unwrap(),
            vchannel_id: data["vchannel_id"].take_string().unwrap(),
            description: data["description"].take_string().unwrap(),
            latest_ts: data["latest_ts"].as_i64().unwrap(),
            index_symbol: data["index_symbol"].take_string().unwrap(),
            is_member: data["is_member"].as_bool().unwrap(),
            member_count: data["member_count"].as_i32().unwrap(),
            mention_count: data["mention_count"].as_i32().unwrap(),
            general: data["general"].as_bool().unwrap(),
            inactive: data["inactive"].as_bool().unwrap(),
            private: data["private"].as_bool().unwrap(),
        }
    }
}
