use serde::{Serialize, Deserialize};

use pijersi_rs::board::Board;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Position {
    #[serde(with = "serde_bytes")]
    pub cells: [u8; 45],
    pub current_player: u8,
}

impl Position {
    pub fn new(board: &Board) -> Position {
        Position {cells: board.cells, current_player: board.current_player}
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Response {
    pub position: Position,
    pub action: u64,
    pub score: i64,
}

impl Response {
    pub fn new(position: Position, action: u64, score: i64) -> Response {
        Response {position, action, score}
    }
}