use eframe::egui::{Button, CentralPanel, Context, Response, Ui};
use eframe::epi::{App, Frame};
use eframe::{run_native, NativeOptions};

const BOARD_LEN: usize = 3;
struct TicTacToe {
    player: Player,
    cells: [Option<Player>; BOARD_LEN * BOARD_LEN],
}

impl TicTacToe {
    fn new() -> TicTacToe {
        TicTacToe {
            player: Player::X,
            cells: [None; BOARD_LEN * BOARD_LEN],
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Player {
    X,
    O,
}

impl Player {
    fn as_str(&self) -> &'static str {
        match self {
            Player::X => "X",
            Player::O => "O",
        }
    }

    pub fn opponent(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl App for TicTacToe {
    fn setup(
        &mut self,
        _ctx: &Context,
        _frame: &Frame,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
    }

    fn update(&mut self, ctx: &Context, _frame: &Frame) {
        let board = self.cells.clone();

        CentralPanel::default().show(ctx, |ui| {
            for (r, row) in board.chunks_exact(BOARD_LEN).enumerate() {
                ui.horizontal(|ui| {
                    for (c, cell) in row.iter().enumerate() {
                        if cool_button(ui, cell).clicked() {
                            if let None = cell {
                                play_cell(&mut self.cells[r * BOARD_LEN + c], self.player);

                                println!(
                                    "Has {} won? {}",
                                    self.player.as_str(),
                                    check_win(self.cells, self.player)
                                );
                                if check_draw(self.cells) {
                                    println!("Draw");
                                }

                                self.player = self.player.opponent();
                            }
                        }
                    }
                });
            }
        });
    }

    fn name(&self) -> &str {
        "TicTacToe"
    }
}

fn play_cell(cell: &mut Option<Player>, player: Player) {
    *cell = Some(player);
}

fn check_draw(cells: [Option<Player>; BOARD_LEN * BOARD_LEN]) -> bool {
    !cells.contains(&None)
}

//TODO: Maybe use fuse? and try to have iterator instead of slice
fn won_with_cells(cells: &[Option<Player>], player: Player) -> bool {
    cells.iter().all(|o| o.map_or(false, |c| c == player))
}

// TODO: explore of using the board as 9 bit number for each player and just &= the hell out of it
fn check_win(cells: [Option<Player>; BOARD_LEN * BOARD_LEN], player: Player) -> bool {
    // rows TODO: extract to functions
    for row in cells.chunks_exact(BOARD_LEN) {
        if won_with_cells(row, player) {
            return true;
        }
    }
    // col
    for col in 0..BOARD_LEN {
        let running = cells[col..].to_owned().into_iter();
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
            .to_owned()
            .into_iter()
            .step_by(2)
            .take(BOARD_LEN)
            .collect::<Vec<Option<Player>>>(),
        player,
    ) {
        return true;
    }

    false
}

fn cool_button(ui: &mut Ui, cell: &Option<Player>) -> Response {
    let button = Button::new(cell.map_or("", |c| c.as_str()));
    ui.add_sized([100., 100.], button)
}

fn main() {
    let size = [340., 340.];
    let app = TicTacToe::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(size.into());
    run_native(Box::new(app), win_option);
}
