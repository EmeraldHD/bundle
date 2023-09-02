use tranquil::{
    bot::Bot,
    utils::{debug_guilds_from_env, discord_token_from_env},
    AnyResult,
};

#[tokio::main]
async fn main() -> AnyResult<()> {
    dotenv::dotenv().ok();

    Bot::new()
        .application_command_update(debug_guilds_from_env()?)
        .register(ping_module::PingModule::new())?
        .run_until_ctrl_c(discord_token_from_env()?)
        .await
}
