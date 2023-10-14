use std::{num::NonZeroUsize, time::Instant};

use thiserror::Error;
use tranquil::serenity::model::id::UserId;

use crate::game::GameSettings;

pub(crate) struct Lobby {
    created_at: Instant,
    last_interaction: Instant,
    players: Vec<UserId>,
    player_limit: PlayerLimit,
    pub(crate) accessibility: LobbyAccessibility,
    pub(crate) game_settings: GameSettings,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum PlayerLimit {
    #[default]
    None,
    AtMost(NonZeroUsize),
}

pub(crate) enum LobbyAccessibility {
    Public,
    Private { invitations: Vec<UserId> },
}

#[derive(Debug, Error)]
pub(crate) enum LobbyJoinError {
    #[error("{0} is not invited to this private lobby")]
    Private(UserId),
    #[error("{0} already joined this lobby")]
    UserAlreadyJoined(UserId),
    #[error("{0} cannot join a full lobby")]
    LobbyFull(UserId),
}

#[derive(Debug, Error)]
pub(crate) enum LobbyKickError {
    #[error("{0} is not part of this lobby")]
    UserNotJoined(UserId),
}

#[derive(Debug, Error)]
pub(crate) enum LobbyInviteError {
    #[error("cannot invite users to a public lobby")]
    PublicLobby,
    #[error("{0} is already invited to this lobby")]
    UserAlreadyInvited(UserId),
}

#[derive(Debug, Error)]
pub(crate) enum LobbyUninviteError {
    #[error("cannot invite users to a public lobby")]
    PublicLobby,
    #[error("{0} is not invited to this lobby")]
    UserNotInvited(UserId),
}

pub(crate) struct LobbyKick {
    /// Indicates that the lobby is now empty and should be deleted.
    pub(crate) lobby_empty: bool,
}

impl Lobby {
    pub(crate) fn new(host: UserId) -> Self {
        let now = Instant::now();
        Self {
            created_at: now,
            last_interaction: now,
            players: vec![host],
            player_limit: PlayerLimit::None,
            accessibility: LobbyAccessibility::Private {
                invitations: vec![],
            },
            game_settings: GameSettings::default(),
        }
    }

    pub(crate) fn created_at(&self) -> Instant {
        self.created_at
    }

    pub(crate) fn last_interaction(&self) -> Instant {
        self.last_interaction
    }

    pub(crate) fn host(&self) -> UserId {
        *self.players.first().expect("players should not be empty")
    }

    pub(crate) fn promote_host(&mut self, user: UserId) {
        todo!("move the player at the beginning of the players array")
    }

    pub(crate) fn players(&self) -> &[UserId] {
        &self.players
    }

    pub(crate) fn player_limit(&self) -> PlayerLimit {
        self.player_limit
    }

    pub(crate) fn join(&mut self, user: UserId) -> Result<(), LobbyJoinError> {
        if !self.accessibility.can_join(user) {
            Err(LobbyJoinError::Private(user))?
        }

        if let PlayerLimit::AtMost(max_players) = self.player_limit {
            if self.players.len() == max_players.get() {
                Err(LobbyJoinError::LobbyFull(user))?
            }
        }

        if self.players.contains(&user) {
            Err(LobbyJoinError::UserAlreadyJoined(user))?
        }

        self.players.push(user);
        Ok(())
    }

    pub(crate) fn kick(&mut self, user: UserId) -> Result<LobbyKick, LobbyKickError> {
        if let Some(index) = self
            .players
            .iter()
            .copied()
            .enumerate()
            .find_map(|(index, joined_user)| (user == joined_user).then_some(index))
        {
            self.players.remove(index);
            Ok(LobbyKick {
                lobby_empty: self.players.is_empty(),
            })
        } else {
            Err(LobbyKickError::UserNotJoined(user))
        }
    }

    pub(crate) fn notify_interaction(&mut self) {
        self.last_interaction = Instant::now();
    }

    pub(crate) fn update_embed(&self) {}
}

impl LobbyAccessibility {
    pub(crate) fn can_join(&self, user: UserId) -> bool {
        match self {
            LobbyAccessibility::Public => true,
            LobbyAccessibility::Private { invitations } => invitations.contains(&user),
        }
    }

    pub(crate) fn invitations(&self) -> Option<&[UserId]> {
        match self {
            LobbyAccessibility::Public => None,
            LobbyAccessibility::Private { invitations } => Some(invitations),
        }
    }

    pub(crate) fn invite(&mut self, user: UserId) -> Result<(), LobbyInviteError> {
        match self {
            LobbyAccessibility::Public => Err(LobbyInviteError::PublicLobby),
            LobbyAccessibility::Private { invitations } => {
                if invitations.contains(&user) {
                    Err(LobbyInviteError::UserAlreadyInvited(user))?
                }

                invitations.push(user);
                Ok(())
            }
        }
    }

    pub(crate) fn uninvite(&mut self, user: UserId) -> Result<(), LobbyUninviteError> {
        match self {
            LobbyAccessibility::Public => Err(LobbyUninviteError::PublicLobby),
            LobbyAccessibility::Private { invitations } => {
                if let Some(index) = invitations
                    .iter()
                    .copied()
                    .enumerate()
                    .find_map(|(index, invited_user)| (user == invited_user).then_some(index))
                {
                    invitations.remove(index);
                    Ok(())
                } else {
                    Err(LobbyUninviteError::UserNotInvited(user))
                }
            }
        }
    }
}
