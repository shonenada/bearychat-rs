use std::fmt::{Display, Result, Formatter};

use json::{JsonValue};
use chrono::{UTC, DateTime};
use error::Error;

#[allow(dead_code)]
pub struct Team {
    id: String,
    name: String,
    subdomain: String,
    inactive: bool,
    logo_url: Option<String>,
    email_domain: Option<String>,
    description: Option<String>,
//    created: DateTime<UTC>,
//    updated: DateTime<UTC>,
}

impl Team {
    pub fn parse_from_json(mut data: JsonValue) -> Team {
        Team {
            id: data["id"].take_string().unwrap(),
            name: data["name"].take_string().unwrap(),
            subdomain: data["subdomain"].take_string().unwrap(),
            inactive: data["inactive"].as_bool().unwrap(),
            email_domain: data["email_domain"].take_string(),
            description: data["description"].take_string(),
            logo_url: data["logo_url"].take_string(),
        }
    }
}

impl Display for Team {
    fn fmt(&self, f : &mut Formatter) -> Result {
        write!(f, "<Team id: \"{}\", name: \"{}\" >", self.id, self.name)
    }
}

#[allow(dead_code)]
pub struct User {
    id: String,
    name: String,
    email: String,
    team_id: String,
    avatar_url: String,
    full_name: Option<String>,
    presence: Option<String>,
}

impl User {
    pub fn parse_from_json(mut data: JsonValue) -> User {
        User {
            id: data["id"].take_string().unwrap(),
            name: data["name"].take_string().unwrap(),
            email: data["email"].take_string().unwrap(),
            team_id: data["team_id"].take_string().unwrap(),
            avatar_url: data["avatar_url"].take_string().unwrap(),
            full_name: data["full_name"].take_string(),
            presence: data["presence"].take_string(),
        }
    }
}

impl Display for User {
    fn fmt(&self, f : &mut Formatter) -> Result {
        write!(f, "<User id: \"{}\", name: \"{}\" >", self.id, self.name)
    }
}

#[allow(dead_code)]
pub struct Channel {
    id: String,
    uid: String,
    name: String,
    team_id: String,
    vchannel_id: String,
    is_member: bool,
    member_count: i32,
    mention_count: i32,
    general: bool,
    private: bool,
    inactive: bool,
    index_symbol: String,
    latest_ts: i64,
    topic: Option<String>,
    description: Option<String>,
}

impl Channel {
    pub fn parse_from_json(mut data: JsonValue) -> Channel {
        Channel {
            id: data["id"].take_string().unwrap(),
            uid: data["uid"].take_string().unwrap(),
            name: data["name"].take_string().unwrap(),
            team_id: data["team_id"].take_string().unwrap(),
            vchannel_id: data["vchannel_id"].take_string().unwrap(),
            is_member: data["is_member"].as_bool().unwrap(),
            member_count: data["member_count"].as_i32().unwrap(),
            mention_count: data["mention_count"].as_i32().unwrap(),
            general: data["general"].as_bool().unwrap(),
            private: data["private"].as_bool().unwrap(),
            inactive: data["inactive"].as_bool().unwrap(),
            index_symbol: data["index_symbol"].take_string().unwrap(),
            latest_ts: data["latest_ts"].as_i64().unwrap(),
            topic: data["topic"].take_string(),
            description: data["description"].take_string(),
        }
    }
}

impl Display for Channel {
    fn fmt(&self, f : &mut Formatter) -> Result {
        write!(f, "<Channel id: \"{}\", name: \"{}\" >", self.id, self.name)
    }
}
