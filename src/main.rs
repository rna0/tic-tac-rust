use eframe::{
    egui::{Button, CentralPanel, Context, Frame, Grid, Layout, Response, RichText, Ui},
    emath::Align,
    epaint::{Color32, Vec2},
    epi::{self, App},
    run_native,
};

mod tic_tac_toe;
use tic_tac_toe::*;

#[derive(Default)]
struct TTTApp {
    state: State,
}

enum State {
    Start,
    Game(TicTacToe),
    Finish(Option<Player>),
}

impl Default for State {
    fn default() -> Self {
        State::Start
    }
}

impl App for TTTApp {
    fn setup(
        &mut self,
        _ctx: &Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
    }

    fn update(&mut self, ctx: &Context, _frame: &epi::Frame) {
        self.state = match &mut self.state {
            State::Start => update_start(ctx),
            State::Game(game) => update_game(game.clone(), ctx),
            State::Finish(winner) => update_finish(ctx, winner),
        }
    }

    fn name(&self) -> &str {
        "TicTacToe"
    }
}

fn update_start(ctx: &Context) -> State {
    let mut state = State::Start;
    CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(Layout::left_to_right(), |ui| {
            ui.set_max_height(ui.available_height() / 2.0);
            ui.spacing_mut().item_spacing.y = 10.0;

            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                if ui.button(RichText::new("Start Game").size(20.0)).clicked() {
                    state = State::Game(TicTacToe::default());
                }

                add_theme_toggle(ctx, ui);
            });
        });
    });
    state
}

fn add_theme_toggle(ctx: &Context, ui: &mut Ui) {
    let style = (*ctx.style()).clone();
    let new_visuals = style.visuals.light_dark_small_toggle_button(ui);
    if let Some(visuals) = new_visuals {
        ctx.set_visuals(visuals);
    }
}

fn update_finish(ctx: &Context, winner: &Option<Player>) -> State {
    let mut state = State::Finish(*winner);

    CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(Layout::left_to_right(), |ui| {
            ui.set_max_height(ui.available_height() / 2.0);
            ui.spacing_mut().item_spacing.y = 10.0;

            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                let winner = winner.map_or("No one".to_string(), |w| w.to_string());
                ui.heading(
                    RichText::new(format!("{winner} has won"))
                        .color(Color32::RED)
                        .strong()
                        .size(40.0),
                );
                ui.add_space(10.0);

                if ui.button(RichText::new("Restart").size(20.0)).clicked() {
                    state = State::Start;
                }
            });
        });
    });

    state
}

fn update_game(mut game: TicTacToe, ctx: &Context) -> State {
    let mut board = Vec::with_capacity(game.board.len());

    CentralPanel::default().show(ctx, |ui| {
        let win_size = ui.available_size();
        ui.spacing_mut().item_spacing = Vec2::new(10.0, 10.0);

        // TODO: extract to function which returns board to check responses, how to initialize board tho?
        Grid::new("Board").show(ui, |ui| {
            for row in game.board.chunks_exact(BOARD_LEN) {
                for cell in row.iter() {
                    board.push(cool_button(ui, win_size, cell));
                }
                ui.end_row();
            }
        });
    });

    let pressed = board.iter().position(|button| button.clicked());
    if let Some(i) = pressed {
        if game.board[i].is_none() {
            play_cell(&mut game.board[i], game.playing);

            if check_win(game.board, game.playing) {
                return State::Finish(Some(game.playing));
            }
            if check_draw(game.board) {
                return State::Finish(None);
            }

            game.playing = game.playing.opponent();
        }
    }

    State::Game(game)
}

fn cool_button(ui: &mut Ui, win_size: Vec2, cell: &Option<Player>) -> Response {
    let button = Button::new(cell.map_or("".to_owned(), |c| c.to_string()));
    let size = win_size / BOARD_LEN as f32 - ui.spacing().item_spacing;
    ui.add_sized(size, button)
}

fn main() {
    let size = [340., 340.];
    let app = TTTApp::default();
    let win_option = eframe::NativeOptions {
        initial_window_size: Some(size.into()),
        ..Default::default()
    };
    run_native(Box::new(app), win_option);
}
