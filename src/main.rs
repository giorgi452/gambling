use game::gamescene::Scene;
use menu::{Button, ButtonType, Menu};
use raylib::prelude::*;

pub mod actual_game;
pub mod game;
pub mod menu;

fn main() {
    let (mut rl, thread) = raylib::init().title("Gambling").build();
    let mut game_scene: Scene = Scene::MainMenu;

    rl.toggle_fullscreen();
    rl.toggle_borderless_windowed();
    rl.set_window_focused();
    rl.set_target_fps(60);

    let start_button_rec = Rectangle::new(
        (get_monitor_width(get_current_monitor()) / 2) as f32 - (300 / 2) as f32,
        (get_monitor_height(get_current_monitor()) / 2) as f32 - (50 / 2) as f32,
        300 as f32,
        50 as f32,
    );

    let exit_button_rec = Rectangle::new(
        (get_monitor_width(get_current_monitor()) / 2) as f32 - (300 / 2) as f32,
        (get_monitor_height(get_current_monitor()) / 2) as f32 + (65 / 2) as f32,
        300 as f32,
        50 as f32,
    );

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLUE);
        match game_scene {
            Scene::MainMenu => {
                Menu::new(vec![
                    Button::new(
                        start_button_rec,
                        Color::DARKBLUE,
                        "Start",
                        ButtonType::Start,
                    ),
                    Button::new(exit_button_rec, Color::DARKBLUE, "Exit", ButtonType::Exit),
                ])
                .draw(&mut d, &mut game_scene);
            }
            Scene::ActGame => {
                static mut MONEY_TO_BET: i32 = 0;
                static mut PRESSED_KEYS: Vec<i32> = Vec::new();
                unsafe {
                    if MONEY_TO_BET == 0 {
                        d.draw_text(
                            "Bet",
                            (d.get_screen_width() / 2) - (70 / 2),
                            (d.get_screen_height() / 2) - (500 / 2),
                            70,
                            Color::RED,
                        );

                        d.draw_rectangle(
                            (d.get_screen_width() / 2) - (300 / 2),
                            (d.get_screen_height() / 2) - (300 / 2),
                            300,
                            50,
                            Color::DARKBLUE,
                        );

                        if d.is_key_pressed(KeyboardKey::KEY_ZERO) {
                            PRESSED_KEYS.push(0);
                        } else if d.is_key_pressed(KeyboardKey::KEY_ONE) {
                            PRESSED_KEYS.push(1);
                        } else if d.is_key_pressed(KeyboardKey::KEY_TWO) {
                            PRESSED_KEYS.push(2);
                        } else if d.is_key_pressed(KeyboardKey::KEY_THREE) {
                            PRESSED_KEYS.push(3);
                        } else if d.is_key_pressed(KeyboardKey::KEY_FOUR) {
                            PRESSED_KEYS.push(4);
                        } else if d.is_key_pressed(KeyboardKey::KEY_FIVE) {
                            PRESSED_KEYS.push(5);
                        } else if d.is_key_pressed(KeyboardKey::KEY_SIX) {
                            PRESSED_KEYS.push(6);
                        } else if d.is_key_pressed(KeyboardKey::KEY_SEVEN) {
                            PRESSED_KEYS.push(7);
                        } else if d.is_key_pressed(KeyboardKey::KEY_EIGHT) {
                            PRESSED_KEYS.push(8);
                        } else if d.is_key_pressed(KeyboardKey::KEY_NINE) {
                            PRESSED_KEYS.push(9);
                        } else if d.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
                            PRESSED_KEYS.pop();
                        } else if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
                            let moneyt_bet: String = PRESSED_KEYS
                                .iter()
                                .map(|v| v.to_string())
                                .collect::<Vec<String>>()
                                .join("");
                            MONEY_TO_BET = i32::from_str_radix(&moneyt_bet, 10).unwrap();
                        }

                        for i in 0..PRESSED_KEYS.len() {
                            d.draw_text(
                                PRESSED_KEYS[i].to_string().as_str(),
                                ((d.get_screen_width() / 2) - (300 / 2)) + (30 * i as i32),
                                (d.get_screen_height() / 2) - (300 / 2),
                                50,
                                Color::RED,
                            );
                        }

                        if PRESSED_KEYS.len() > 10 {
                            PRESSED_KEYS.pop();
                        }
                    } else {
                        d.draw_text(&MONEY_TO_BET.to_string(), 10, 10, 100, Color::RED);
                    }
                }
            }
        }
    }
}
