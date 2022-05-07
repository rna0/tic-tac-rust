use eframe::egui::{Button, CentralPanel, Context, Response, Ui};
use eframe::epi::{App, Frame};
use eframe::run_native;

mod tic_tac_toe;
use tic_tac_toe::*;

enum TTTApp {
    Start,
    Game(TicTacToe),
    Finish(Option<Player>),
}

impl App for TTTApp {
    fn setup(
        &mut self,
        _ctx: &Context,
        _frame: &Frame,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
    }

    fn update(&mut self, ctx: &Context, _frame: &Frame) {
        *self = match self {
            TTTApp::Start => update_start(ctx),
            TTTApp::Game(game) => update_game(game, ctx),
            TTTApp::Finish(winner) => update_finish(ctx, winner),
        }
    }

    fn name(&self) -> &str {
        "TicTacToe"
    }
}

fn update_start(ctx: &Context) -> TTTApp {
    let mut state = TTTApp::Start;
    CentralPanel::default().show(ctx, |ui| {
        if ui.button("StartGame").clicked() {
            state = TTTApp::Game(TicTacToe::new());
        }
    });

    state
}

fn update_finish(ctx: &Context, winner: &mut Option<Player>) -> TTTApp {
    let mut state = TTTApp::Finish(*winner);
    CentralPanel::default().show(ctx, |ui| {
        if ui
            .button(format!(
                "{} has won",
                winner.map_or("No one".to_string(), |w| w.to_string())
            ))
            .clicked()
        {
            state = TTTApp::Start;
        }
    });

    state
}

fn update_game(game: &mut TicTacToe, ctx: &Context) -> TTTApp {
    let mut board = Vec::with_capacity(game.board.len());

    CentralPanel::default().show(ctx, |ui| {
        for row in game.board.chunks_exact(BOARD_LEN) {
            ui.horizontal(|ui| {
                for cell in row.iter() {
                    board.push(cool_button(ui, cell))
                }
            });
        }
    });

    let pressed = board.iter().position(|button| button.clicked());
    if let Some(i) = pressed {
        if game.board[i].is_none() {
            play_cell(&mut game.board[i], game.playing);

            if check_win(game.board, game.playing) {
                return TTTApp::Finish(Some(game.playing));
            }
            if check_draw(game.board) {
                return TTTApp::Finish(None);
            }

            game.playing = game.playing.opponent();
        }
    }

    TTTApp::Game(game.clone())
}

fn cool_button(ui: &mut Ui, cell: &Option<Player>) -> Response {
    let button = Button::new(cell.map_or("".to_owned(), |c| c.to_string()));
    ui.add_sized([100., 100.], button)
}

fn main() {
    let size = [340., 340.];
    let app = TTTApp::Start;
    let win_option = eframe::NativeOptions {
        initial_window_size: Some(size.into()),
        ..Default::default()
    };
    run_native(Box::new(app), win_option);
}
