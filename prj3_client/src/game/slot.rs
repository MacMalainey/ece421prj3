use crate::game::{AI_ID, PLAYER_ID};

const NO_PLAYER: u32 = 0;

#[derive(Clone, Copy, Debug)]
pub struct Slot {
    occupied: bool,
    player: u32
}

impl Slot {
    pub fn new() -> Self {
        Self {
            occupied: false,
            player: NO_PLAYER
        }
    }

    pub fn place(&mut self, player: u32) {
        self.occupied = true;
        self.player = player;
    }

    pub fn clear(&mut self) {
        self.occupied = false;
        self.player = NO_PLAYER;
    }

    pub fn to_string(&self) -> String {
        return if self.player == 0 {
            String::from("O")
        } else if self.player == PLAYER_ID {
            String::from("1")
        } else if self.player == AI_ID {
            String::from("2")
        } else {
            panic!("Unknown player.")
        };
    }

    pub fn get_occupied(&self) -> bool {
        self.occupied
    }

    pub fn get_player(&self) -> u32 {
        self.player
    }

    pub fn owned_by(&self, player: u32) -> bool {
        if self.player == player {
            return true;
        }

        false
    }
}
