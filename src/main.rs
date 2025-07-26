use {
    clap::Parser,
    serde::Deserialize,
    serenity::{
        all::{Context, EventHandler, GuildId, Member, RoleId},
        async_trait,
        prelude::GatewayIntents,
        Client, Error,
    },
    std::{collections::HashMap, path::PathBuf},
};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Config {
    #[arg(short, long)]
    token: String,

    #[arg(short, long, default_value = "config.json")]
    config: PathBuf,
}

#[derive(Debug, Deserialize)]
struct Handler(HashMap<GuildId, Vec<RoleId>>);

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        if let Some(roles) = self.0.get(&new_member.guild_id) {
            if let Err(e) = new_member.add_roles(ctx.http, roles.as_slice()).await {
                eprintln!("{e:?}")
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    let cfg = Config::parse();

    let json_str = std::fs::read_to_string(cfg.config).map_err(Error::Io)?;
    let handler = serde_json::from_str::<Handler>(&json_str).map_err(Error::Json)?;

    let intents = GatewayIntents::GUILD_MEMBERS;

    let mut client = Client::builder(&cfg.token, intents)
        .event_handler(handler)
        .await?;

    Ok(client.start_autosharded().await?)
}
