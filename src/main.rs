use eframe::egui::{Button, CentralPanel, Context, Response, Ui};
use eframe::epi::{App, Frame};
use eframe::{run_native, NativeOptions};

mod tic_tac_toe;
use tic_tac_toe::*;

impl App for TicTacToe {
    fn setup(
        &mut self,
        _ctx: &Context,
        _frame: &Frame,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
    }

    fn update(&mut self, ctx: &Context, _frame: &Frame) {
        let board = self.board.clone();

        CentralPanel::default().show(ctx, |ui| {
            for (r, row) in board.chunks_exact(BOARD_LEN).enumerate() {
                ui.horizontal(|ui| {
                    for (c, cell) in row.iter().enumerate() {
                        if cool_button(ui, cell).clicked() {
                            if let None = cell {
                                play_cell(&mut self.board[r * BOARD_LEN + c], self.player);

                                println!(
                                    "Has {} won? {}",
                                    self.player,
                                    check_win(self.board, self.player)
                                );
                                if check_draw(self.board) {
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

fn cool_button(ui: &mut Ui, cell: &Option<Player>) -> Response {
    let button = Button::new(cell.map_or("".to_owned(), |c| c.to_string()));
    ui.add_sized([100., 100.], button)
}

fn main() {
    let size = [340., 340.];
    let app = TicTacToe::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(size.into());
    run_native(Box::new(app), win_option);
}
