use anyhow::Result;
use async_trait::async_trait;
use tranquil::{
    context::command::CommandCtx,
    l10n::{L10n, L10nLoadError},
    macros::{command_provider, slash},
    module::Module,
};

pub struct PingModule;

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
        command
            .respond(|response| {
                response.interaction_response_data(|data| data.content("**Pong!**").ephemeral(true))
            })
            .await?;
        Ok(())
    }
}
