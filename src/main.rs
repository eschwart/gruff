use {
    clap::Parser,
    serenity::{
        all::{Context, EventHandler, Member, RoleId},
        async_trait,
        prelude::GatewayIntents,
        Client,
    },
};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Config {
    #[arg(short, long)]
    token: String,

    #[arg(required = true, num_args = 1.., last = true)]
    roles: Vec<RoleId>,
}

struct Handler(Vec<RoleId>);

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        if let Err(e) = new_member.add_roles(ctx.http, self.0.as_slice()).await {
            eprintln!("{:?}", e)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), serenity::Error> {
    let cfg = Config::parse();

    let intents = GatewayIntents::GUILD_MEMBERS;

    let mut client = Client::builder(&cfg.token, intents)
        .event_handler(Handler(cfg.roles))
        .await?;

    client.start_autosharded().await
}
