use std::process::exit;

use raylib::prelude::*;

use crate::game::gamescene::Scene;

pub enum ButtonType {
    Start,
    Exit,
}

pub struct Menu {
    pub buttons: Vec<Button>,
}

pub struct Button {
    pub rec: Rectangle,
    pub color: Color,
    pub text: &'static str,
    pub btype: ButtonType,
}

impl Menu {
    pub fn new(buttons: Vec<Button>) -> Self {
        Self { buttons }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle<'_>, game_scene: &mut Scene) {
        for i in 0..self.buttons.len() {
            self.buttons[i].draw(d, game_scene);
        }
        d.draw_text(
            "Gambling",
            (get_monitor_width(get_current_monitor()) / 2) as i32 - (400 / 2) as i32,
            (get_monitor_height(get_current_monitor()) / 2) as i32 - (300 / 2) as i32,
            100,
            Color::RED,
        );
    }
}

impl Button {
    pub fn new(rec: Rectangle, color: Color, text: &'static str, btype: ButtonType) -> Self {
        Self {
            rec,
            color,
            text,
            btype,
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle<'_>, game_scene: &mut Scene) {
        d.draw_rectangle_rec(self.rec, self.color);
        d.draw_text(
            self.text,
            self.rec.x as i32 + self.rec.x as i32 / 9,
            self.rec.y as i32 + 5,
            40,
            Color::RED,
        );
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
