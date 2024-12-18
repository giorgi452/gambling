use game::gamescene::Scene;
use menu::{Button, Menu};
use raylib::prelude::*;

pub mod game;
pub mod menu;

fn main() {
    let (mut rl, thread) = raylib::init().title("Hello, World").build();
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

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLUE);
        match game_scene {
            Scene::MainMenu => {
                Menu::new(vec![Button::new(
                    start_button_rec,
                    Color::DARKBLUE,
                    "Start",
                )])
                .draw(&mut d, &mut game_scene);
            }
            Scene::ActGame => {
                d.draw_text("Gamble", 50, 50, 50, Color::RED);
            }
        }
    }
}
