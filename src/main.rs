use eframe::egui::{Button, CentralPanel, Context, Response, Ui};
use eframe::epi::{App, Frame};
use eframe::{run_native, NativeOptions};

const BOARD_LEN: usize = 3;
struct TicTacToe {
    x_turn: bool,
    cells: [Cell; BOARD_LEN * BOARD_LEN],
}

impl TicTacToe {
    fn new() -> TicTacToe {
        TicTacToe {
            x_turn: true,
            cells: [Cell::Empty; BOARD_LEN * BOARD_LEN],
        }
    }
}

fn play_cell(cell: &mut Cell, x_turn: bool) {
    if x_turn {
        *cell = Cell::X;
    } else {
        *cell = Cell::O;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cell {
    Empty,
    X,
    O,
}

impl Cell {
    fn as_str(&self) -> &'static str {
        match self {
            Cell::X => "X",
            Cell::O => "O",
            Cell::Empty => " ",
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
                        if cool_button(ui, cell).clicked() && *cell == Cell::Empty {
                            play_cell(&mut self.cells[r * BOARD_LEN + c], self.x_turn);

                            let player = if self.x_turn { Cell::X } else { Cell::O };
                            println!("Has won? {}", check_win(self.cells, player));
                            if check_draw(self.cells) {
                                println!("Draw");
                            }

                            self.x_turn = !self.x_turn;
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

fn check_draw(cells: [Cell; BOARD_LEN * BOARD_LEN]) -> bool {
    cells.iter().all(|c| *c != Cell::Empty)
}

fn won_with_cells(cells: &[Cell], player: Cell) -> bool {
    cells.iter().all(|c| *c == player)
}

fn check_win(cells: [Cell; BOARD_LEN * BOARD_LEN], player: Cell) -> bool {
    // rows TODO: extract to functions
    for row in cells.chunks_exact(BOARD_LEN) {
        if won_with_cells(row, player) {
            return true;
        }
    }
    // col
    for col in 0..BOARD_LEN {
        let running = cells[col..].to_owned().into_iter();
        if won_with_cells(&running.step_by(BOARD_LEN).collect::<Vec<Cell>>(), player) {
            return true;
        }
    }
    // diagonal
    if won_with_cells(&cells.into_iter().step_by(4).collect::<Vec<Cell>>(), player) {
        return true;
    }
    // reverse diagonal
    if won_with_cells(
        &cells[2..]
            .to_owned()
            .into_iter()
            .step_by(2)
            .take(BOARD_LEN)
            .collect::<Vec<Cell>>(),
        player,
    ) {
        return true;
    }

    false
}

fn cool_button(ui: &mut Ui, cell: &Cell) -> Response {
    let button = Button::new(cell.as_str());
    ui.add_sized([100., 100.], button)
}

fn main() {
    let size = [340., 340.];
    let app = TicTacToe::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(size.into());
    run_native(Box::new(app), win_option);
}
