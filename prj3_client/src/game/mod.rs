pub use board::*;
pub use slot::*;

pub mod board;
pub mod slot;

pub const PLAYER_ID: u32 = 1;
pub const AI_ID: u32 = 2;

#[derive(PartialEq, Copy, Clone)]
pub enum GameState {
    Running,
    Win(u32),
    Tie,
}

pub struct Game {
    board: Board,
    state: GameState,
}

impl Game {
    pub fn new(rows: usize, columns: usize, ai: AIConfiguration) -> Self {
        let board = Board::new(rows, columns, ai);

        Self {
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
            result.push((slot.get_player() as i32, "".to_string()));
        }

        result
    }

    /// Check if a win or tie has occurred
    /// player is the player that just performed a move
    pub fn check_state(&mut self, player: u32) -> GameState {
        if self.board.check_if_won(player) {
            self.state = GameState::Win(player);
        } else if self.board.check_if_no_more_moves() {
            self.state = GameState::Tie;
        } else {
            self.state = GameState::Running;
        }

        self.state
    }

    /// Begin process for player turn
    /// Returns true on success
    pub fn player_turn(&mut self, column_selection: usize) -> bool {
        if self.board.check_column_selection(column_selection as isize) == ColumnSelectionResult::Valid {
            self.board.place_at_column(column_selection, PLAYER_ID);
            return true;
        }

        false
    }

    /// Begin process for AI turn
    pub fn ai_turn(&mut self) {
        let column_selection = self.board.get_ai_move();
        self.board.place_at_column(column_selection, AI_ID);
        self.check_state(AI_ID);
    }

    /// Print the board
    pub fn print_board(&self) {
        println!("Current board:");
        self.board.print();
    }
}
