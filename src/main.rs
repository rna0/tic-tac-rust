use eframe::epi::{App, Frame};
use eframe::{NativeOptions, run_native};
use eframe::egui::{CentralPanel, Context};

struct TicTacToeCells {
    cells: Vec<TicTacToeCell>
}

struct TicTacToeCell {
    cell: String
}

impl TicTacToeCells {
    fn new() -> TicTacToeCells {
        let iter: impl Iterator<Item=TicTacToeCell> = (0..9).map(|a| TicTacToeCell {
            cell: format!("X{}", a)
        });
        TicTacToeCells{
            cells: Vec::from_iter(iter)
        }
    }
}

impl App for TicTacToeCells {
    fn update(&mut self, ctx: &Context, frame: &Frame) {
        CentralPanel::default().show(ctx, |ui| {});
    }

    fn name(&self) -> &str {
        "TicTacToe"
    }
}

fn main() {
    let app = TicTacToeCells:new();
    let win_option = NativeOptions::default();
    run_native(Box::new(app), win_option);
}
