use std::usize;

use rand::prelude::*;

use crate::game::{AI_ID, PLAYER_ID};
use super::GameState;

use super::slot::*;

#[derive(Clone, Copy, Debug)]
pub struct Move {
    pub row: usize,
    pub column: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColumnSelectionResult {
    Valid,
    ColumnFull,
    ColumnDoesNotExist,
}

#[derive(Clone, Copy, Debug)]
pub struct AIConfiguration {
    search_depth: u32,
    random_iterations: u32,
}

pub const AI_EASY: AIConfiguration = AIConfiguration {
    search_depth: 2,
    random_iterations: 2,
};

pub const AI_MEDIUM: AIConfiguration = AIConfiguration {
    search_depth: 2,
    random_iterations: 500,
};

pub const AI_HARD: AIConfiguration = AIConfiguration {
    search_depth: 3,
    random_iterations: 1500,
};

#[derive(Debug)]
pub struct Board {
    pub rows: usize,
    pub columns: usize,
    player_turn: bool,
    pub storage: Vec<Slot>,
    heights: Vec<usize>,
    moves: u32,
    move_history: Vec<Move>,
    column_order: Vec<usize>,
    ai: AIConfiguration,
}

impl Board {
    pub fn new(rows: usize, columns: usize, ai: AIConfiguration) -> Self {
        // Create board storage
        let mut storage = Vec::new();

        for _ in 0..(rows * columns) {
            storage.push(Slot::new());
        }

        // Create height storage
        let mut heights = Vec::new();

        for _ in 0..columns {
            heights.push(0);
        }

        // Create column order (optimizes AI search)
        let mut column_order = Vec::new();

        for column in 0..columns as i32 {
            let order = columns as i32 / 2 + (1 - 2 * (column % 2)) * (column + 1) / 2;
            column_order.push(order as usize);
        }

        Self {
            rows,
            columns,
            player_turn: true,
            storage,
            heights,
            moves: 0,
            move_history: Vec::new(),
            column_order,
            ai,
        }
    }

    /// Returns the slot at the given row and column.
    pub fn get_slot(&self, row: usize, column: usize) -> &Slot {
        match self.storage.get(row * self.columns + column) {
            None => {
                panic!("Slot does not exist at {}, {}", row, column);
            }
            Some(slot) => {
                slot
            }
        }
    }

    /// Returns the slot (mut) at the given row and column.
    pub fn get_slot_mut(&mut self, row: usize, column: usize) -> &mut Slot {
        match self.storage.get_mut(row * self.columns + column) {
            None => {
                panic!("Slot does not exist at {}, {}", row, column);
            }
            Some(slot) => {
                slot
            }
        }
    }

    /// Check if a given column is valid to place a piece into.
    pub fn check_column_selection(&self, column: isize) -> ColumnSelectionResult {
        if column < 0 || column as usize >= self.columns {
            return ColumnSelectionResult::ColumnDoesNotExist;
        }

        if self.heights[column as usize] >= self.rows {
            ColumnSelectionResult::ColumnFull
        } else {
            ColumnSelectionResult::Valid
        }
    }

    /// Place a piece at a given column.
    /// Assumes the given column is valid to place into.
    pub fn place_at_column(&mut self, column: usize, player: u32) {
        // Get index to place into
        let lowest_row = self.rows - self.heights[column] - 1;

        // Occupy the slot
        self.get_slot_mut(lowest_row, column).place(player);
        self.heights[column] += 1;
        self.moves += 1;
        self.move_history.push(Move {
            row: lowest_row,
            column,
        });

        // Switch turns
        self.player_turn = !self.player_turn;
    }

    /// Undo the last move. Used by the AI to scout winning moves.
    pub fn undo_move(&mut self) {
        match self.move_history.pop() {
            None => {
                panic!("Error undoing last move: no more previous moves");
            }
            Some(last_move) => {
                match self.storage.get_mut(last_move.row * self.columns + last_move.column) {
                    None => {
                        panic!("Error undoing last move: slot not found");
                    }
                    Some(lowest_slot) => {
                        // Clear the slot
                        if lowest_slot.get_occupied() {
                            lowest_slot.clear();
                            self.heights[last_move.column] -= 1;
                            self.moves -= 1;
                        } else {
                            println!("Warning: cleared an already empty slot")
                        }

                        // Switch turns
                        self.player_turn = !self.player_turn;
                    }
                }
            }
        }
    }

    /// Check if the given player has four connected pieces
    /// Return true if the player has won
    pub fn check_if_won(&self, player: u32) -> bool {
        // Horizontal check
        for i in 0..(self.rows) {
            for j in 0..(self.columns - 3) {
                if self.get_slot(i, j).owned_by(player)
                    && self.get_slot(i, j + 1).owned_by(player)
                    && self.get_slot(i, j + 2).owned_by(player)
                    && self.get_slot(i, j + 3).owned_by(player)
                {
                    return true;
                }
            }
        }

        // Vertical check
        for j in 0..(self.columns) {
            for i in 0..(self.rows - 3) {
                if self.get_slot(i, j).owned_by(player)
                    && self.get_slot(i + 1, j).owned_by(player)
                    && self.get_slot(i + 2, j).owned_by(player)
                    && self.get_slot(i + 3, j).owned_by(player)
                {
                    return true;
                }
            }
        }

        // Ascending diagonal check
        for i in 3..(self.rows) {
            for j in 0..(self.columns - 3) {
                if self.get_slot(i, j).owned_by(player)
                    && self.get_slot(i - 1, j + 1).owned_by(player)
                    && self.get_slot(i - 2, j + 2).owned_by(player)
                    && self.get_slot(i - 3, j + 3).owned_by(player)
                {
                    return true;
                }
            }
        }

        // Descending diagonal check
        for i in 3..(self.rows) {
            for j in 3..(self.columns) {
                if self.get_slot(i, j).owned_by(player)
                    && self.get_slot(i - 1, j - 1).owned_by(player)
                    && self.get_slot(i - 2, j - 2).owned_by(player)
                    && self.get_slot(i - 3, j - 3).owned_by(player)
                {
                    return true;
                }
            }
        }

        false
    }

    /// Return true if there are no more possible moves.
    pub fn check_if_no_more_moves(&self) -> bool {
        for column in 0..self.columns {
            if self.heights[column] <= self.rows - 1 {
                return false;
            }
        }

        true
    }

    /// Return true if the given move by the given player would be
    /// a winning move.
    pub fn check_if_winning_move(&mut self, column: usize, player: u32) -> bool {
        if self.check_column_selection(column as isize) == ColumnSelectionResult::Valid {
            // Place the piece in the column, check if won, and then remove the piece
            self.place_at_column(column, player);
            let won = self.check_if_won(player);
            self.undo_move();

            return won;
        }

        false
    }

    /// Calculate the score of the given position
    pub fn negamax(&mut self, depth: u32, mut alpha: i32, mut beta: i32) -> i32 {
        if alpha >= beta {
            panic!();
        }

        // Limit the number of recursive calls
        if depth == 0 {
            return 0;
        }

        let rows = self.rows as i32;
        let columns = self.columns as i32;
        let moves = self.moves as i32;
        let player = if self.player_turn { PLAYER_ID } else { AI_ID };

        if self.moves == (rows * columns) as u32 {
            return 0;
        }

        // Check if current player can win on next turn
        for column in 0..columns {
            if self.check_if_winning_move(column as usize, player) {
                return (rows * columns + 1 - moves) / 2;
            }
        }

        // Upper bound of score
        let max = (rows * columns - 1 - moves) / 2;

        if beta > max {
            beta = max;

            if alpha >= beta {
                return beta;
            }
        }

        // Compute score of all possible next moves and keep the best one
        for i in 0..columns {
            let column = self.column_order[i as usize];

            if self.check_column_selection(column as isize) == ColumnSelectionResult::Valid {
                self.place_at_column(column as usize, player);
                let score = -self.negamax(depth - 1, -beta, -alpha);
                self.undo_move();

                if score >= beta {
                    return score;
                }

                if score > alpha {
                    alpha = score;
                }
            }
        }

        alpha
    }

    /// Randomly pick moves and determine the score
    pub fn random_search(&mut self) -> i32 {
        let mut score = 0;

        for _ in 0..self.ai.random_iterations {
            let mut moves = 0;
            let mut scout_state = GameState::Running;

            loop {
                match scout_state {
                    GameState::Running => {
                        // Collect all available moves
                        let mut m = Vec::new();

                        for column in 0..self.columns {
                            if self.check_column_selection(column as isize) == ColumnSelectionResult::Valid {
                                m.push(column);
                            }
                        }

                        if m.len() == 0 {
                            break;
                        }

                        let rand_choice = m[random::<usize>() % m.len()];
                        let player = if self.player_turn { PLAYER_ID } else { AI_ID };

                        // Temporarily place piece on board
                        self.place_at_column(rand_choice, player);

                        if self.check_if_won(player) {
                            scout_state = GameState::Win(player);
                        } else if self.check_if_no_more_moves() {
                            scout_state = GameState::Tie;
                        }

                        moves += 1;
                    }
                    GameState::Win(player) => {
                        match player {
                            AI_ID => {
                                score += 1;
                            }
                            _ => {
                                score -= 1;
                            }
                        }
                        break;
                    }
                    GameState::Tie => {
                        break;
                    }
                }
            }

            // Undo the scout moves
            for _ in 0..moves {
                self.undo_move()
            }
        }

        score
    }

    /// Get the next column the AI should play
    pub fn get_ai_move(&mut self) -> usize {
        // Play each possible move and find the highest score
        let n = (self.rows * self.columns) as i32;
        let mut highest_score = i32::MIN;
        let mut choice = 0;

        for column in 0..self.columns {
            if self.check_column_selection(column as isize) == ColumnSelectionResult::Valid {
                // Place the piece in the column, see if score is highest, and then remove the piece
                self.place_at_column(column, AI_ID);

                let mut score = (-self.negamax(self.ai.search_depth, -n / 2, n / 2)) << 14;

                // 0 is returned if we reached maximum search depth
                // Use random search instead if that happens
                if score == 0 {
                    score = self.random_search();
                }

                if score > highest_score {
                    highest_score = score;
                    choice = column;
                }

                self.undo_move();
                // println!("Column {} has a score of {}.", column + 1, score);
            }
        }

        // println!("Selected column {} with a score of {}.", choice + 1, highest_score);
        choice
    }

    /// Print the board.
    pub fn print(&self) {
        let latest_move = match self.move_history.last() {
            Some(latest_move) => {
                latest_move
            }
            None => {
                &Move {
                    row: usize::MAX,
                    column: usize::MAX,
                }
            }
        };

        for i in 0..self.rows {
            print!("|");

            for j in 0..self.columns {
                let slot = self.get_slot(i, j);

                if i == latest_move.row && j == latest_move.column {
                    print!("*{}*|", slot.to_string());
                } else {
                    print!(" {} |", slot.to_string());
                }
            }

            println!();
        }

        for j in 0..self.columns {
            print!("{number:>width$} ", number = j + 1, width = 3);
        }

        println!();
    }
}
