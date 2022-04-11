use crate::game::GameType;
use crate::game::Letter::*;

const NO_PLAYER: u32 = 0;

/// TOOT and OTTO letters
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Letter {
    T,
    O,
}

/// TOOT and OTTO player winning sequence
pub const PLAYER_WINNING_SEQ: [Letter; 4] = [T, O, O, T];

/// TOOT and OTTO AI winning sequence
pub const AI_WINNING_SEQ: [Letter; 4] = [O, T, T, O];

/// Slot class. Can either be occupied or not occupied.
/// If occupied, can contain the player's ID and letter of the piece
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

    /// Occupy this slot with a piece with the given
    /// player and letter (optional)
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

    /// Clear this piece and mark it as not occupied
    pub fn clear(&mut self) {
        self.occupied = false;
        self.player = NO_PLAYER;
    }

    /// Get string representation of what is in this slot
    pub fn to_string(&self, mode: GameType) -> String {
        return if self.player == 0 {
            return String::from("");
        } else {
            if mode == GameType::Connect4 {
                String::from("")
            } else {
                if self.letter == Letter::T {
                    String::from("T")
                } else {
                    String::from("O")
                }
            }
        }
    }

    /// Return true if this slot is occupied
    pub fn get_occupied(&self) -> bool {
        self.occupied
    }

    /// Return the player ID occupying this slot
    pub fn get_player(&self) -> u32 {
        self.player
    }

    /// Check if this slot is occupied by a piece owned by the given player
    pub fn owned_by(&self, player: u32) -> bool {
        if self.occupied && self.player == player {
            return true;
        }

        false
    }

    /// Check if this slot is occupied by a piece with the given letter
    pub fn matches_letter(&self, letter: Letter) -> bool {
        if self.occupied && self.letter == letter {
            return true;
        }

        false
    }
}
