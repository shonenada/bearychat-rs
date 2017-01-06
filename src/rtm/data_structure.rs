use std::fmt::{Display, Result, Formatter};

use json::{JsonValue};
use chrono::{UTC, DateTime};

#[allow(dead_code)]
pub struct Team {
    id: Option<String>,
    name: Option<String>,
    subdomain: Option<String>,
    email_domain: Option<String>,
    description: Option<String>,
    inactive: Option<bool>,
    logo_url: Option<String>,
//    created: DateTime<UTC>,
//    updated: DateTime<UTC>,
}

impl Display for Team {
    fn fmt(&self, f : &mut Formatter) -> Result {
        write!(f, "<Team ");

        match self.id.clone() {
            Some(id) => { write!(f, "id: \"{}\"", id); },
            None => {}
        }

        match self.name.clone() {
            Some(name) => { write!(f, ", name: \"{}\"", name); },
            None => {}
        }

        write!(f, ">")
    }
}

impl Team {
    pub fn parse_from_json(mut data: JsonValue) -> Team {
        Team {
            id: data["id"].take_string(),
            name: data["name"].take_string(),
            subdomain: data["subdomain"].take_string(),
            email_domain: data["email_domain"].take_string(),
            description: data["description"].take_string(),
            logo_url: data["logo_url"].take_string(),
            inactive: data["inactive"].as_bool(),
        }
    }
}

#[allow(dead_code)]
pub struct User {
    id: Option<String>,
    name: Option<String>,
    email: Option<String>,
    team_id: Option<String>,
    full_name: Option<String>,
    avatar_url: Option<String>,
    presence: Option<String>,
}

impl Display for User {
    fn fmt(&self, f : &mut Formatter) -> Result {
        write!(f, "<User ");

        match self.id.clone() {
            Some(id) => { write!(f, "id: \"{}\"", id); },
            None => {}
        }

        match self.name.clone() {
            Some(name) => { write!(f, ", name: \"{}\"", name); }
            None => {}
        }

        write!(f, ">")
    }
}

impl User {
    pub fn parse_from_json(mut data: JsonValue) -> User {
        User {
            id: data["id"].take_string(),
            name: data["name"].take_string(),
            email: data["email"].take_string(),
            team_id: data["team_id"].take_string(),
            full_name: data["full_name"].take_string(),
            avatar_url: data["avatar_url"].take_string(),
            presence: data["presence"].take_string(),
        }
    }
}

#[allow(dead_code)]
pub struct Channel {
    id: Option<String>,
    uid: Option<String>,
    name: Option<String>,
    topic: Option<String>,
    team_id: Option<String>,
    vchannel_id: Option<String>,
    description: Option<String>,
    latest_ts: Option<i64>,
    index_symbol: Option<String>,
    is_member: Option<bool>,
    member_count: Option<i32>,
    mention_count: Option<i32>,
    general: Option<bool>,
    inactive: Option<bool>,
    private: Option<bool>,
}

impl Display for Channel {
    fn fmt(&self, f : &mut Formatter) -> Result {
        write!(f, "<Channel ");

        match self.id.clone() {
            Some(id) => { write!(f, "id: \"{}\"", id); }
            None => {}
        }

        match self.name.clone() {
            Some(name) => { write!(f, ", name: \"{}\"", name); }
            None => {}
        }

        write!(f, ">")
    }
}

impl Channel {
    pub fn parse_from_json(mut data: JsonValue) -> Channel {
        Channel {
            id: data["id"].take_string(),
            uid: data["uid"].take_string(),
            name: data["name"].take_string(),
            topic: data["topic"].take_string(),
            team_id: data["team_id"].take_string(),
            vchannel_id: data["vchannel_id"].take_string(),
            description: data["description"].take_string(),
            latest_ts: data["latest_ts"].as_i64(),
            index_symbol: data["index_symbol"].take_string(),
            is_member: data["is_member"].as_bool(),
            member_count: data["member_count"].as_i32(),
            mention_count: data["mention_count"].as_i32(),
            general: data["general"].as_bool(),
            inactive: data["inactive"].as_bool(),
            private: data["private"].as_bool(),
        }
    }
}
