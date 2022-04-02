use eframe::egui::{Button, CentralPanel, Context, Response, Ui};
use eframe::epi::{App, Frame};
use eframe::{run_native, NativeOptions};

struct TicTacToe {
    initial_window_size: [f32; 2],
    x_turn: bool,
    cells: [Cell; 9],
}

impl TicTacToe {
    fn new(size: [f32; 2]) -> TicTacToe {
        TicTacToe {
            x_turn: true,
            cells: [Cell::Empty; 9],
            initial_window_size: size,
        }
    }
}

fn click_button(cell: &mut Cell, x_turn: bool) {
    if x_turn {
        *cell = Cell::X;
    } else {
        *cell = Cell::O;
    }
}

#[derive(Copy, Clone)]
enum Cell {
    X,
    O,
    Empty,
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
                            click_button(cell, self.x_turn);
                            self.x_turn = !self.x_turn;
                            // TODO: check if won. how to do it mid borrow of mut self?
                            // TODO: change the layout to endgame?
                            return;
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

fn cool_button(ui: &mut Ui, cell: &Cell) -> Response {
    let button = Button::new(cell.as_str());
    ui.add_sized([100., 100.], button)
}

fn main() {
    let size = [340., 340.];
    let app = TicTacToe::new(size);
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(size.into());
    run_native(Box::new(app), win_option);
}
