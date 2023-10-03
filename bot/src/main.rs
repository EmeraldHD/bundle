use anyhow::Result;
use db::Database;
use tranquil::{
    bot::Bot,
    utils::{debug_guilds_from_env, discord_token_from_env},
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let mut db = Database::connect()?;
    db.migrate_verbose()?;

    return Ok(());

    Bot::new()
        .application_command_update(debug_guilds_from_env()?)
        .register(ping_module::PingModule::new())
        .run_until_ctrl_c(discord_token_from_env()?)
        .await
}
