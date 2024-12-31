use std::process::exit;

use raylib::prelude::*;

use crate::game::gamescene::Scene;

pub enum ButtonType {
    Start,
    Exit,
}

pub struct Menu<'a> {
    pub buttons: Vec<Button<'a>>,
}

pub struct Button<'a> {
    pub rec: Rectangle,
    pub color: Color,
    pub text: &'a str,
    pub btype: ButtonType,
}

impl Menu<'_> {
    pub fn new(buttons: Vec<Button>) -> Menu<'_> {
        Menu { buttons }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle<'_>, game_scene: &mut Scene) {
        // Draw Gambling Title
        d.draw_text(
            "Gambling",
            (get_monitor_width(get_current_monitor()) / 2) as i32 - (400 / 2) as i32,
            (get_monitor_height(get_current_monitor()) / 2) as i32 - (300 / 2) as i32,
            100,
            Color::new(0x73, 0x77, 0xBD, 0xFF),
        );

        // Draw Buttons
        for i in 0..self.buttons.len() {
            self.buttons[i].draw(d, game_scene);
        }
    }
}

impl Button<'_> {
    pub fn new(rec: Rectangle, color: Color, text: &'static str, btype: ButtonType) -> Self {
        Self {
            rec,
            color,
            text,
            btype,
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle<'_>, game_scene: &mut Scene) {
        // Draw Button
        d.draw_rectangle_rec(self.rec, self.color);

        // Draw Text of Button
        d.draw_text(
            self.text,
            self.rec.x as i32 + self.rec.x as i32 / 9,
            self.rec.y as i32 + 5,
            40,
            Color::new(0x4E, 0x54, 0xBD, 0xFF),
        );

        // Mouse Button Functions By Type
        match self.btype {
            ButtonType::Start => {
                if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                    let mouse_position = d.get_mouse_position();
                    if self.rec.check_collision_point_rec(mouse_position) {
                        *game_scene = Scene::ActGame;
                    }
                }
            }
            ButtonType::Exit => {
                if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                    let mouse_position = d.get_mouse_position();
                    if self.rec.check_collision_point_rec(mouse_position) {
                        exit(0);
                    }
                }
            }
        }
    }
}
