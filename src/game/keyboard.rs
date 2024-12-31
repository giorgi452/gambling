use raylib::prelude::*;

pub struct Keyboard {
    pressed_keys: Vec<i32>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            pressed_keys: Vec::new(),
        }
    }

    pub fn get_keys(&mut self, d: &mut RaylibDrawHandle, money_to_bet: &mut i32) {
        if d.is_key_pressed(KeyboardKey::KEY_ZERO) {
            self.pressed_keys.push(0);
        } else if d.is_key_pressed(KeyboardKey::KEY_ONE) {
            self.pressed_keys.push(1);
        } else if d.is_key_pressed(KeyboardKey::KEY_TWO) {
            self.pressed_keys.push(2);
        } else if d.is_key_pressed(KeyboardKey::KEY_THREE) {
            self.pressed_keys.push(3);
        } else if d.is_key_pressed(KeyboardKey::KEY_FOUR) {
            self.pressed_keys.push(4);
        } else if d.is_key_pressed(KeyboardKey::KEY_FIVE) {
            self.pressed_keys.push(5);
        } else if d.is_key_pressed(KeyboardKey::KEY_SIX) {
            self.pressed_keys.push(6);
        } else if d.is_key_pressed(KeyboardKey::KEY_SEVEN) {
            self.pressed_keys.push(7);
        } else if d.is_key_pressed(KeyboardKey::KEY_EIGHT) {
            self.pressed_keys.push(8);
        } else if d.is_key_pressed(KeyboardKey::KEY_NINE) {
            self.pressed_keys.push(9);
        } else if d.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
            self.pressed_keys.pop();
        } else if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
            let money_str: String = self
                .pressed_keys
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join("");
            if let Ok(money) = money_str.parse::<i32>() {
                *money_to_bet = money;
            }
        }

        if self.pressed_keys.len() > 10 {
            self.pressed_keys.pop();
        }
    }

    pub fn draw_keys(&self, d: &mut RaylibDrawHandle) {
        for (i, key) in self.pressed_keys.iter().enumerate() {
            d.draw_text(
                &key.to_string(),
                (((d.get_screen_width() / 2) - (300 / 2)) + (30 * i as i32)) + 20,
                (d.get_screen_height() / 2) - (300 / 2),
                51,
                Color::new(0x4E, 0x54, 0xBD, 0xFF),
            );
        }
    }
}
