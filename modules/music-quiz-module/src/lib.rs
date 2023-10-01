use std::collections::{btree_map::Entry, BTreeMap};

use anyhow::Result;
use async_trait::async_trait;
use lobby::Lobby;
use tokio::sync::RwLock;
use tranquil::{
    autocomplete::Autocomplete,
    context::{AutocompleteCtx, CommandCtx},
    l10n::{L10n, L10nLoadError},
    macros::{autocompleter, command_provider, slash},
    module::Module,
    serenity::model::{id::GuildId, user::User},
};

mod game;
mod lobby;

#[derive(Default)]
pub struct MusicQuizModule {
    lobbies: RwLock<BTreeMap<GuildId, Lobby>>,
}

impl MusicQuizModule {
    pub fn new() -> MusicQuizModule {
        Default::default()
    }
}

#[async_trait]
impl Module for MusicQuizModule {
    async fn l10n(&self) -> Result<L10n, L10nLoadError> {
        L10n::from_yaml_file("l10n/music-quiz-module.yaml").await
    }
}

#[command_provider]
impl MusicQuizModule {
    #[slash(autocomplete)]
    async fn guess(&self, command: CommandCtx, _guess: Autocomplete<String>) -> Result<()> {
        command
            .create_response(|response| {
                response.interaction_response_data(|data| data.content("Hallo Welt!"))
            })
            .await?;
        Ok(())
    }

    #[slash(rename = "music-quiz create")]
    async fn music_quiz_create(&self, command: CommandCtx) -> Result<()> {
        let guild_id = command.interaction.guild_id.unwrap();
        let mut lobbies = self.lobbies.write().await;

        match lobbies.entry(guild_id) {
            Entry::Occupied(_) => {
                drop(lobbies);
                command
                    .create_response(|response| {
                        response.interaction_response_data(|data| {
                            data.content("Only one lobby can be created per server!")
                                .ephemeral(true)
                        })
                    })
                    .await?;
            }
            Entry::Vacant(entry) => {
                entry.insert(Lobby::new(command.interaction.user.id));
                drop(lobbies);
                command
                    .create_response(|response| {
                        response.interaction_response_data(|data| data.content("Lobby created!"))
                    })
                    .await?;
            }
        }

        Ok(())
    }

    #[slash(rename = "music-quiz invite")]
    async fn music_quiz_invite(&self, command: CommandCtx, user: User) -> Result<()> {
        let guild_id = command.interaction.guild_id.unwrap();
        let lobbies = self.lobbies.write().await;
        if let Some(lobby) = lobbies.get(&guild_id) {
            if lobby.host() == command.interaction.user.id {}
        } else {
            command
                .create_response(|response| {
                    response.interaction_response_data(|data| {
                        data.content(":x: No music-quiz lobby on this server!")
                    })
                })
                .await?;
        }

        Ok(())
    }

    #[slash(rename = "music-quiz uninvite")]
    async fn music_quiz_uninvite(&self, command: CommandCtx, user: User) -> Result<()> {
        command
            .create_response(|response| {
                response.interaction_response_data(|data| data.content(format!("Invited {user}")))
            })
            .await?;
        Ok(())
    }

    #[slash(rename = "music-quiz kick")]
    async fn music_quiz_kick(&self, command: CommandCtx, user: User) -> Result<()> {
        command
            .create_response(|response| {
                response.interaction_response_data(|data| data.content(format!("Kicked {user}")))
            })
            .await?;
        Ok(())
    }

    #[slash(rename = "music-quiz join")]
    async fn music_quiz_join(&self, command: CommandCtx) -> Result<()> {
        command
            .create_response(|response| {
                response.interaction_response_data(|data| data.content("Joined!"))
            })
            .await?;
        Ok(())
    }

    #[slash(rename = "music-quiz leave")]
    async fn music_quiz_leave(&self, command: CommandCtx) -> Result<()> {
        command
            .create_response(|response| {
                response.interaction_response_data(|data| data.content("Joined!"))
            })
            .await?;
        Ok(())
    }
}

impl MusicQuizModule {
    #[autocompleter]
    async fn autocomplete_guess(&self, _ctx: AutocompleteCtx) -> Result<()> {
        Ok(())
    }
}
