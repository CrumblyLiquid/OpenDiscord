use enigo::*;
use std::collections::HashMap;
use std::fs::File;
use duplicate::duplicate;
use std::io::{self, prelude::*};
use std::path::Path;
use serde::Deserialize;
use tokio::time::{sleep, Duration};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, id::RoleId},
    prelude::*,
};

struct Handler {
    config: Config,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Prevent handling messages from bots or prevent reading from the wrong channel
        if msg.author.bot || (!self.config.channels.empty() && !self.config.channels.check(&msg.channel_id.0)) {
            return
        }
        match msg.guild_id {
            Some(guild_id) => {
                if !self.config.guilds.empty() && !self.config.guilds.check(&guild_id.0) {
                    return
                }
                if !self.config.roles.empty() {
                    // Don't really know but maybe should cache the result if serenity doesn't do that already
                    let role_ids = self.config.roles.get();
                    for role_id in role_ids {
                        match msg.author.has_role(&ctx, guild_id, RoleId::from(role_id.to_owned())).await {
                            Ok(has) => {
                                // Return if he doesn't have the role
                                match has {
                                    true => (),
                                    false => return,
                                }
                            },
                            // Error? Maybe handle that idk
                            Err(_) => (),
                        }
                    }
                }
            },
            None => (),
        }
        // Get command for message
        let cmd = match self.config.get_command(&msg.content) {
            None => return,
            Some(s) => s,
        };
        // Get only the first character (will later implement support for all keys so this won't be necessary)
        let cmd = match cmd.to_lowercase().chars().next() {
            None => return,
            Some(s) => s,
        };
        let mut e = Enigo::new();
        println!("Triggered: {} -> {} ({}#{})", msg.content, cmd, msg.author.name, msg.author.discriminator);
        let key = Key::Layout(cmd);
        e.key_down(key);
        // Should add duration for each key separately
        sleep(Duration::from_millis(self.config.get_duration().to_owned())).await;
        e.key_up(key);
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    println!("OpenDiscord ({})\n  v{}\n  by {}", env!("CARGO_PKG_DESCRIPTION"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));
    let config: Config = Config::new("./config.json").unwrap();
    let mut client = Client::builder(config.get_token())
        .event_handler(Handler {config})
        .await
        .expect("Run into error while creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

trait Ids {
    fn get(&self) -> &Vec<u64>;
    fn check(&self, id: &u64) -> bool;
    fn empty(&self) -> bool;
}

#[duplicate(name; [Guilds]; [Channels]; [Roles])]
#[derive(Deserialize, Debug)]
struct name {
    l: Vec<u64>,
}

#[duplicate(name; [Guilds]; [Channels]; [Roles])]
impl Ids for name {
    fn get(&self) -> &Vec<u64> {
        &self.l
    }

    fn check(&self, id: &u64) -> bool {
        self.l.contains(id)
    }

    fn empty(&self) -> bool {
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
struct Config {
    token: String,
    guilds: Guilds,
    channels: Channels,
    roles: Roles,
    duration: u64,
    commands: HashMap<String, String>,
}

impl Config {
    fn new<P: AsRef<Path>>(path: P) -> Result<Config, io::Error> {
        let pconf = PlaceholderConfig::new(path)?;
        let conf = pconf.convert();
        Ok(conf)
    }

    fn get_token(&self) -> &String {
        &self.token
    }

    // Duration of keypress in milliseconds
    fn get_duration(&self) -> &u64 {
        &self.duration
    }

    // Add some sort of command parser to support all the keys
    fn get_command(&self, msg: &String) -> Option<&String> {
        self.commands.get(msg)
    }
}

