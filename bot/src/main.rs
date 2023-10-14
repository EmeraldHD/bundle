use anyhow::Result;
use tranquil::{
    bot::Bot,
    utils::{debug_guilds_from_env, discord_token_from_env},
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    Bot::new()
        .application_command_update(debug_guilds_from_env()?)
        .register(ping_module::PingModule)
        .run_until_ctrl_c(discord_token_from_env()?)
        .await
}
