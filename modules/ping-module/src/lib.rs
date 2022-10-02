use tokio::sync::RwLock;
use tranquil::{
    command::CommandContext, command_provider, l10n::CommandL10nProvider, module::Module, slash,
    AnyResult,
};

#[derive(Default)]
pub struct PingModule {
    ping_count: RwLock<u64>,
}

impl PingModule {
    pub fn new() -> PingModule {
        Default::default()
    }
}

impl CommandL10nProvider for PingModule {
    fn translations_filepath(&self) -> &'static str {
        "l10n/ping-module.toml"
    }
}

impl Module for PingModule {}

#[command_provider]
impl PingModule {
    #[slash]
    async fn ping(&self, command: CommandContext, _dummy: Option<bool>) -> AnyResult<()> {
        // TODO: _dummy parameter, because slash macro is bugged and requires at least one parameter...
        let mut ping_count = self.ping_count.write().await;
        *ping_count += 1;
        let ping_count = ping_count.downgrade();
        command
            .interaction
            .create_interaction_response(&command.ctx.http, |response| {
                response.interaction_response_data(|data| {
                    data.embed(|embed| {
                        embed.title("Pong").field(
                            "Ping-Count",
                            format!("```rust\n{ping_count}```"),
                            true,
                        )
                    })
                })
            })
            .await?;
        Ok(())
    }
}
