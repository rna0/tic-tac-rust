use std::fmt::{Display, Formatter, Result};
pub const BOARD_LEN: usize = 3;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Player {
    X,
    O,
}

pub struct TicTacToe {
    pub player: Player,
    pub board: [Option<Player>; BOARD_LEN * BOARD_LEN],
}

impl TicTacToe {
    pub fn new() -> TicTacToe {
        TicTacToe {
            player: Player::X,
            board: [None; BOARD_LEN * BOARD_LEN],
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

impl Player {
    pub fn opponent(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

pub fn play_cell(cell: &mut Option<Player>, player: Player) {
    *cell = Some(player);
}

pub fn check_draw(cells: [Option<Player>; BOARD_LEN * BOARD_LEN]) -> bool {
    !cells.contains(&None)
}

//TODO: Maybe use fuse? and try to have iterator instead of slice
fn won_with_cells(cells: &[Option<Player>], player: Player) -> bool {
    cells.iter().all(|o| o.map_or(false, |c| c == player))
}

// TODO: explore of using the board as 9 bit number for each player and just &= the hell out of it
pub fn check_win(cells: [Option<Player>; BOARD_LEN * BOARD_LEN], player: Player) -> bool {
    // rows TODO: extract to functions
    for row in cells.chunks_exact(BOARD_LEN) {
        if won_with_cells(row, player) {
            return true;
        }
    }
    // col
    for col in 0..BOARD_LEN {
        let running = cells[col..].iter().copied();
        if won_with_cells(
            &running.step_by(BOARD_LEN).collect::<Vec<Option<Player>>>(),
            player,
        ) {
            return true;
        }
    }
    // diagonal
    if won_with_cells(
        &cells
            .into_iter()
            .step_by(4)
            .collect::<Vec<Option<Player>>>(),
        player,
    ) {
        return true;
    }
    // reverse diagonal
    if won_with_cells(
        &cells[2..]
            .iter()
            .copied()
            .step_by(2)
            .take(BOARD_LEN)
            .collect::<Vec<Option<Player>>>(),
        player,
    ) {
        return true;
    }

    false
}
