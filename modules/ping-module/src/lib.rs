use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::RwLock;
use tranquil::{
    context::CommandCtx,
    l10n::{L10n, L10nLoadError},
    macros::{command_provider, slash},
    module::Module,
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

#[async_trait]
impl Module for PingModule {
    async fn l10n(&self) -> Result<L10n, L10nLoadError> {
        L10n::from_yaml_file("l10n/ping-module.yaml").await
    }
}

#[command_provider]
impl PingModule {
    #[slash]
    async fn ping(&self, command: CommandCtx) -> Result<()> {
        let mut ping_count = self.ping_count.write().await;
        *ping_count += 1;
        let ping_count = ping_count.downgrade();
        command
            .interaction
            .create_interaction_response(&command.bot.http, |response| {
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
