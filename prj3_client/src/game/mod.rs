pub use board::*;
use shared_types::types::GameType;
pub use slot::*;

pub mod board;
pub mod slot;

pub const PLAYER_ID: u32 = 1;
pub const AI_ID: u32 = 2;

/// Stores the game state. A game can be
/// running, win, or tie. If a game is won,
/// it contains the ID of the player that won.
#[derive(PartialEq, Copy, Clone)]
pub enum GameState {
    Running,
    Win(u32),
    Tie,
}

/// Game
/// game_type: Connect4 or TOOT and OTTO
/// board: stores the board of the current game
/// state: current game state (running, win, or tie)
pub struct Game {
    game_type: GameType,
    board: Board,
    state: GameState,
}

impl Game {
    pub fn new(rows: usize, columns: usize, game_type: GameType, ai: AIConfiguration) -> Self {
        let board = Board::new(rows, columns, game_type, ai);

        Self {
            game_type,
            board,
            state: GameState::Running,
        }
    }

    /// Get game state
    pub fn get_state(&self) -> GameState {
        self.state
    }

    /// Get board state (used to render)
    pub fn get_board_state(&self) -> Vec<(i32, String)> {
        let mut result = Vec::new();

        for slot in self.board.storage.clone() {
            result.push((slot.get_player() as i32, slot.to_string(self.game_type)));
        }

        result
    }

    /// Check if a win or tie has occurred
    /// player is the player that just performed a move
    pub fn check_state(&mut self) -> GameState {
        if self.board.check_if_won(PLAYER_ID) {
            self.state = GameState::Win(PLAYER_ID);
        } else if self.board.check_if_won(AI_ID) {
            self.state = GameState::Win(AI_ID);
        } else if self.board.check_if_no_more_moves() {
            self.state = GameState::Tie;
        } else {
            self.state = GameState::Running;
        }

        self.state
    }

    /// Begin process for player turn
    /// Returns true on success
    pub fn player_turn(&mut self, column_selection: usize, letter: Option<Letter>) -> bool {
        if self.board.check_column_selection(column_selection as isize) == ColumnSelectionResult::Valid {
            let possible_move = PossibleMove {
                column: column_selection,
                letter
            };
            self.board.place_at_column(possible_move, PLAYER_ID);
            return true;
        }

        false
    }

    /// Begin process for AI turn
    pub fn ai_turn(&mut self) {
        let possible_move = self.board.get_ai_move();
        self.board.place_at_column(possible_move, AI_ID);
    }

    pub fn get_num_moves(&self) -> u32 {
        self.board.get_moves()
    }

    /// Print the board
    pub fn _print_board(&self) {
        println!("Current board:");
        self.board._print();
    }
}
