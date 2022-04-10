use crate::game::{AI_ID, GameType, PLAYER_ID};
use crate::game::Letter::*;

const NO_PLAYER: u32 = 0;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Letter {
    T,
    O,
}

pub const PLAYER_WINNING_SEQ: [Letter; 4] = [T, O, O, T];
pub const AI_WINNING_SEQ: [Letter; 4] = [O, T, T, O];

#[derive(Clone, Copy, Debug)]
pub struct Slot {
    occupied: bool,
    player: u32,
    letter: Letter,
}

impl Slot {
    pub fn new() -> Self {
        Self {
            occupied: false,
            player: NO_PLAYER,
            letter: T,
        }
    }

    pub fn place(&mut self, player: u32, letter: Option<Letter>) {
        self.occupied = true;
        self.player = player;

        match letter {
            None => {}
            Some(letter) => {
                self.letter = letter;
            }
        };
    }

    pub fn clear(&mut self) {
        self.occupied = false;
        self.player = NO_PLAYER;
    }

    pub fn to_string(&self, mode: GameType) -> String {
        return if self.player == 0 {
            return String::from("-");
        } else {
            if mode == GameType::Connect4 {
                if self.player == PLAYER_ID {
                    String::from("1")
                } else if self.player == AI_ID {
                    String::from("2")
                } else {
                    panic!("Unknown player.")
                }
            } else {
                if self.letter == Letter::T {
                    String::from("T")
                } else if self.letter == Letter::O {
                    String::from("O")
                } else {
                    panic!("Unknown letter type.")
                }
            }
        }
    }

    pub fn get_occupied(&self) -> bool {
        self.occupied
    }

    pub fn get_player(&self) -> u32 {
        self.player
    }

    pub fn owned_by(&self, player: u32) -> bool {
        if self.occupied && self.player == player {
            return true;
        }

        false
    }

    pub fn matches_letter(&self, letter: Letter) -> bool {
        if self.occupied && self.letter == letter {
            return true;
        }

        false
    }
}
