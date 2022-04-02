use eframe::egui::{Button, CentralPanel, Context, Response, Ui};
use eframe::epaint::Vec2;
use eframe::epi::{App, Frame};
use eframe::{run_native, NativeOptions};

struct TicTacToeCells {
    is_x_turn: bool,
    cells: Vec<TicTacToeCell>,
}

impl TicTacToeCells {
    fn new() -> TicTacToeCells {
        let iter = (0..9).map(|a| TicTacToeCell {
            cell: "■",
        });
        TicTacToeCells {
            is_x_turn: true,
            cells: Vec::from_iter(iter),
        }
    }
}

struct TicTacToeCell {
    cell: &'static str,
}

impl App for TicTacToeCells {
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
                        let cool_button = create_cool_button(ui, cell);
                        if cool_button.clicked() {
                            if self.is_x_turn{
                                cell.cell = "X";
                            }
                            else{
                                cell.cell = "O";
                            }
                            self.is_x_turn = !self.is_x_turn;
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

fn create_cool_button(ui: &mut Ui, cell: &TicTacToeCell) -> Response {
    let button = Button::new(cell.cell);
    ui.add_sized([100., 100.], button)
}

fn main() {
    let app = TicTacToeCells::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(540., 540.));
    run_native(Box::new(app), win_option);
}
