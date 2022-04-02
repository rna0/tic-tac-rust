use eframe::egui::{self, CentralPanel, Context, CtxRef, Response, Ui};
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
    fn setup(&mut self, ctx: &CtxRef, _frame: &Frame, _storage: Option<&dyn eframe::epi::Storage>) {
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &Context, frame: &Frame) {
        CentralPanel::default().show(ctx, |ui| {
            for row in self.cells.chunks(3) {
                ui.horizontal(|ui| {
                    for cell in row {
                        let coolButton = create_cool_button(ui, cell);
                        if coolButton.clicked() {}
                    }
                });
            }
        });
    }

    fn name(&self) -> &str {
        "TicTacToe"
    }
}

impl TicTacToeCells {
    pub fn configure_fonts(&mut self, ctx: &CtxRef) {
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "MesloLGS".to_string(),
            Cow::Borrowed(include_bytes!("../../MesloLGS_NF_Regular.ttf")),
        );
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Heading,
            (FontFamily::Proportional, 35.),
        );
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Body,
            (FontFamily::Proportional, 20.),
        );
        font_def
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "MesloLGS".to_string());
        ctx.set_fonts(font_def);
    }
}

fn create_cool_button(ui: &mut Ui, cell: &TicTacToeCell) -> Response {
    ui.button(&cell.cell)
}

fn main() {
    let app = TicTacToeCells::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(540., 480.));
    run_native(Box::new(app), win_option);
}
