use eframe::egui::{Button, CentralPanel, Context, Response, Ui};
use eframe::epi::{App, Frame};
use eframe::{run_native, NativeOptions};

struct TicTacToe {
    x_turn: bool,
    cells: [Cell; 9],
}

impl TicTacToe {
    fn new() -> TicTacToe {
        TicTacToe {
            x_turn: true,
            cells: [Cell::Empty; 9],
        }
    }
}

fn click_button(cell: &mut Cell, x_turn: &mut bool) {
    if *x_turn {
        *cell = Cell::X;
    } else {
        *cell = Cell::O;
    }
    *x_turn = !*x_turn
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
        CentralPanel::default().show(ctx, |ui| {
            for row in self.cells.chunks_exact_mut(3) {
                ui.horizontal(|ui| {
                    for cell in row.iter_mut() {
                        let cool_button = cool_button(ui, cell);
                        if cool_button.clicked() && matches!(cell, Cell::Empty) {
                            click_button(cell, &mut self.x_turn);
                            // TODO: check if won. how to do it mid borrow of mut self?
                            // TODO: change the layout to endgame?
                            // break;
                        }
                    }
                });
            }
        });

        let player = if !self.x_turn { Cell::X } else { Cell::O };
        println!("{}", check_win(self.cells, &player));
        check_draw(self.cells);
    }

    fn name(&self) -> &str {
        "TicTacToe"
    }
}

fn check_draw(cells: [Cell; 9]) -> bool {
    cells.iter().all(|c| *c == Cell::Empty)
}

// TODO: implement Player type (or Option<Cell> is better)
fn won_with_cells(cells: &[Cell], player: &Cell) -> bool {
    cells.iter().all(|c| *c == *player)
}

fn check_win(cells: [Cell; 9], player: &Cell) -> bool {
    // rows TODO: extract to functions
    for row in cells.chunks_exact(3) {
        if won_with_cells(row, player) {
            return true;
        }
    }
    // col
    for col in 0..2 {
        let running = cells[col..].to_owned().into_iter();
        if won_with_cells(&running.step_by(3).collect::<Vec<Cell>>(), player) {
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
            .take(3)
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
