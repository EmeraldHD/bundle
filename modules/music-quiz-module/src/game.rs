use std::{num::NonZeroUsize, time::Duration};

pub(crate) struct GameSettings {
    pub(crate) rounds: NonZeroUsize,
    pub(crate) guess_time: Duration,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            rounds: NonZeroUsize::new(20).unwrap(),
            guess_time: Duration::from_secs(30),
        }
    }
}
