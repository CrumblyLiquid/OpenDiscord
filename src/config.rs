use std::collections::HashMap;
use std::fs::File;
use duplicate::duplicate;
use std::io::{self, prelude::*};
use std::path::Path;
use serde::Deserialize;

#[duplicate(name; [Guilds]; [Channels]; [Roles])]
#[derive(Deserialize, Debug)]
pub struct name {
    l: Vec<u64>,
}

#[duplicate(name; [Guilds]; [Channels]; [Roles])]
impl name {
    pub fn get(&self) -> &Vec<u64> {
        &self.l
    }

    pub fn check(&self, id: &u64) -> bool {
        self.l.contains(id)
    }

    pub fn empty(&self) -> bool {
        self.l.is_empty()
    }
}

// We use this PlaceholderConfig to deserialize json into it, then convert it into config
#[derive(Deserialize, Debug)]
struct PlaceholderConfig {
    token: String,
    guilds: Vec<u64>,
    channels: Vec<u64>,
    roles: Vec<u64>,
    duration: u64,
    commands: HashMap<String, String>,
}

impl PlaceholderConfig {
    fn new<P: AsRef<Path>>(path: P) -> Result<PlaceholderConfig, io::Error> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let conf: PlaceholderConfig = serde_json::from_str(&content[..])?;
        Ok(conf)
    }

    fn convert(&self) -> Config {
        Config {
            token: self.token.to_owned(),
            guilds: Guilds {l: self.guilds.to_owned()},
            channels: Channels {l: self.channels.to_owned()},
            roles: Roles {l: self.roles.to_owned()},
            duration: self.duration.to_owned(),
            commands: self.commands.to_owned(),
        }
    }
}

// Put config.json contents into this strut
#[derive(Debug)]
pub struct Config {
    token: String,
    pub guilds: Guilds,
    pub channels: Channels,
    pub roles: Roles,
    duration: u64,
    commands: HashMap<String, String>,
}

impl Config {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Config, io::Error> {
        let pconf = PlaceholderConfig::new(path)?;
        let conf = pconf.convert();
        Ok(conf)
    }

    pub fn get_token(&self) -> &String {
        &self.token
    }

    // Not used because we need the object, not reference
    // Might use later, if I do, remove the pub from guilds, channels and roles attributes
    // pub fn get_guilds(&self) -> &Guilds {
    //     &self.guilds
    // }

    // pub fn get_channels(&self) -> &Channels {
    //     &self.channels
    // }

    // pub fn get_roles(&self) -> &Roles {
    //     &self.roles
    // }

    // Duration of keypress in milliseconds
    pub fn get_duration(&self) -> &u64 {
        &self.duration
    }

    // Add some sort of command parser to support all the keys
    pub fn get_command(&self, msg: &String) -> Option<&String> {
        self.commands.get(msg)
    }
}