use enigo::*;
use std::collections::HashMap;
use std::fs::File;
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
        let channel_id = self.config.get_channel_id();
        if msg.author.bot || (channel_id != msg.channel_id.0 && channel_id != 0) {
            return
        }
        let role_id = self.config.get_role_id();
        if role_id != 0 {
            // We want to limit potential requests to Discord (I think?)
            match msg.guild_id {
                // Don't do anything if it's in private chat
                None => (),
                // If it's in guild, check for role
                Some(guild_id) => {
                    match msg.author.has_role(&ctx, guild_id, RoleId::from(role_id)).await {
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
        sleep(Duration::from_millis(self.config.get_duration())).await;
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

// Put config.json contents into this strut
#[derive(Deserialize, Debug)]
struct Config {
    token: String,
    channel_id: u64,
    role_id: u64,
    duration: u64,
    commands: HashMap<String, String>,
}

impl Config {
    fn new<P: AsRef<Path>>(path: P) -> Result<Config, io::Error> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let conf: Config = serde_json::from_str(&content[..])?;
        Ok(conf)
    }

    fn get_token(&self) -> &String {
        &self.token
    }

    fn get_channel_id(&self) -> u64 {
        self.channel_id
    }

    // Implement check for roles
    fn get_role_id(&self) -> u64 {
        self.role_id
    }

    // Duration of keypress in milliseconds
    fn get_duration(&self) -> u64 {
        self.duration
    }

    // Add some sort of command parser to support all the keys
    fn get_command(&self, msg: &String) -> Option<&String> {
        self.commands.get(msg)
    }
}

