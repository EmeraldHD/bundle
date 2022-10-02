use std::cmp::Ordering;

pub struct GuessingGame {
    _player: String,
    secret: i32,
}

#[derive(PartialEq, Eq, Debug)]
pub enum GuessResult {
    TooSmall,
    TooBig,
    Correct,
}

impl GuessingGame {
    pub fn new(player: String) -> Self {
        Self {
            _player: player,
            secret: 42,
        }
    }
    pub fn guess(&self, number: i32) -> GuessResult {
        match number.cmp(&self.secret) {
            Ordering::Less => GuessResult::TooSmall,
            Ordering::Equal => GuessResult::Correct,
            Ordering::Greater => GuessResult::TooBig,
        }
    }
}

#[test]
fn test() {
    let game = GuessingGame::new("Neko".to_owned());
    let result = game.guess(40);
    assert_eq!(result, GuessResult::TooSmall);
}
