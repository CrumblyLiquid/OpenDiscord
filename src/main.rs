use enigo::*;
use tokio::time::{sleep, Duration};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, id::RoleId},
    prelude::*,
};

mod config;

struct Handler {
    config: config::Config,
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
    let config: config::Config = config::Config::new("./config.json").unwrap();
    let mut client = Client::builder(config.get_token())
        .event_handler(Handler {config})
        .await
        .expect("Run into error while creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
