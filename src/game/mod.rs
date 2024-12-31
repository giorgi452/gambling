use keyboard::Keyboard;
use raylib::prelude::*;

pub mod gamescene;
pub mod keyboard;

pub fn dec_vars() -> (Rectangle, Rectangle, i32, Keyboard) {
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

    let money_to_bet: i32 = 0;
    let keyboard = Keyboard::new();

    (start_button_rec, exit_button_rec, money_to_bet, keyboard)
}
