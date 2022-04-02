use eframe::egui::{Button, CentralPanel, Context, ScrollArea};
use eframe::epaint::Vec2;
use eframe::epi::{App, Frame};
use eframe::{run_native, NativeOptions};

struct TicTacToeCells {
    cells: Vec<TicTacToeCell>,
}

impl TicTacToeCells {
    fn new() -> TicTacToeCells {
        let iter = (0..9).map(|a| TicTacToeCell {
            cell: "■".to_string(),
        });
        TicTacToeCells {
            cells: Vec::from_iter(iter),
        }
    }
}

struct TicTacToeCell {
    cell: String,
}

impl App for TicTacToeCells {
    fn setup(&mut self, _ctx: &eframe::egui::Context, _frame: &Frame, _storage: Option<&dyn eframe::epi::Storage>) {
        
    }

    fn update(&mut self, ctx: &Context, frame: &Frame) {
        CentralPanel::default().show(ctx, |ui| {
            for a in &self.cells {
                
                if ui.button(&a.cell).clicked() {
                    
                }
            }
        });
    }

    fn name(&self) -> &str {
        "TicTacToe"
    }
}

fn main() {
    let app = TicTacToeCells::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(540., 480.));
    run_native(Box::new(app), win_option);
}
