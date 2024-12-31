use actual_game::ActGame;
use game::{dec_vars, gamescene::Scene};
use menu::{Button, ButtonType, Menu};
use raylib::prelude::*;

pub mod actual_game;
pub mod game;
pub mod menu;

fn main() {
    let (mut rl, thread) = raylib::init().title("Gambling").build();
    let mut game_scene: Scene = Scene::MainMenu;

    // Window Options
    rl.toggle_fullscreen();
    rl.toggle_borderless_windowed();
    rl.set_window_focused();
    rl.set_target_fps(60);

    let (start_button_rec, exit_button_rec, mut money_to_bet, mut keyboard) = dec_vars();
    let mut act_game = ActGame::new(&mut money_to_bet);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(0x0D, 0x0C, 0x12, 0xFF));
        match game_scene {
            Scene::MainMenu => {
                *act_game.money_to_bet = 0;
                Menu::new(vec![
                    Button::new(
                        start_button_rec,
                        Color::new(0x19, 0x1A, 0x28, 0xFF),
                        "Start",
                        ButtonType::Start,
                    ),
                    Button::new(
                        exit_button_rec,
                        Color::new(0x19, 0x1A, 0x28, 0xFF),
                        "Exit",
                        ButtonType::Exit,
                    ),
                ])
                .draw(&mut d, &mut game_scene);
            }
            Scene::ActGame => {
                if *act_game.money_to_bet == 0 {
                    ActGame::bet(&mut *act_game.money_to_bet, &mut d, &mut keyboard);
                } else {
                    act_game.draw(&mut d, &mut game_scene);
                }
            }
            Scene::Won => {
                d.draw_text(
                    "You Won!",
                    (d.get_screen_width() / 2) - 700,
                    (d.get_screen_height() / 2) - 300,
                    300,
                    Color::get_color(0x3772ff),
                );

                let text = act_game.money_to_bet.to_string();
                let text_width = d.measure_text(&text, 300);
                let x_position = (d.get_screen_width() - text_width) / 2;

                d.draw_text(
                    &text,
                    x_position,
                    d.get_screen_height() / 2,
                    300,
                    Color::get_color(0x3772ff),
                );
            }
        }
    }
}
