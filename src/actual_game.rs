use ::core::panic;

use raylib::prelude::*;

use crate::game::gamescene::Scene;

#[derive(PartialEq, Eq)]
pub enum SlotType {
    Seven = 0,
    Strawberry = 1,
    Lemon = 2,
    Badrijani = 3,
    ShokoladianiKrepi = 4,
}

pub struct ActGame<'a> {
    pub money_to_bet: i32,
    pub seven_image: &'a Texture2D,
}

pub struct Slot {
    pub stype: SlotType,
}

impl ActGame<'_> {
    pub fn new(money_to_bet: i32, seven_image: &Texture2D) -> ActGame<'_> {
        ActGame {
            money_to_bet,
            seven_image,
        }
    }

    pub fn draw(self, d: &mut RaylibDrawHandle<'_>, game_scene: &mut Scene) {
        unsafe {
            static mut SLOT_ROWS: Vec<Slot> = vec![];

            SLOT_ROWS.push(Slot::new(SlotType::Seven));
            SLOT_ROWS.push(Slot::new(SlotType::Badrijani));
            SLOT_ROWS.push(Slot::new(SlotType::ShokoladianiKrepi));

            if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
                for i in 0..SLOT_ROWS.len() {
                    let row = d.get_random_value(std::ops::Range { start: 0, end: 4 });
                    match row {
                        0 => {
                            SLOT_ROWS[i].change(SlotType::Seven);
                        }
                        1 => {
                            SLOT_ROWS[i].change(SlotType::Badrijani);
                        }
                        2 => {
                            SLOT_ROWS[i].change(SlotType::Lemon);
                        }
                        3 => {
                            SLOT_ROWS[i].change(SlotType::Strawberry);
                        }
                        4 => {
                            SLOT_ROWS[i].change(SlotType::ShokoladianiKrepi);
                        }
                        _ => {
                            panic!("Error!");
                        }
                    }
                }
            } else if d.is_key_pressed(KeyboardKey::KEY_E) {
                *game_scene = Scene::MainMenu;
            }

            if (SLOT_ROWS[0].stype == SLOT_ROWS[1].stype)
                && (SLOT_ROWS[0].stype == SLOT_ROWS[2].stype)
                && (SLOT_ROWS[1].stype == SLOT_ROWS[2].stype)
            {
                *game_scene = Scene::Won;
            } else {
                for i in 0..3 {
                    match SLOT_ROWS[i].stype {
                        SlotType::Seven => {
                            d.draw_text(
                                "7",
                                (d.get_screen_width() / 2) - (250 * i as i32),
                                (d.get_screen_height() / 2) - 200,
                                300,
                                Color::RED,
                            );
                        }
                        SlotType::Badrijani => {
                            d.draw_text(
                                "B",
                                (d.get_screen_width() / 2) - (250 * i as i32),
                                (d.get_screen_height() / 2) - 200,
                                300,
                                Color::RED,
                            );
                        }
                        SlotType::Lemon => {
                            d.draw_text(
                                "L",
                                (d.get_screen_width() / 2) - (250 * i as i32),
                                (d.get_screen_height() / 2) - 200,
                                300,
                                Color::RED,
                            );
                        }
                        SlotType::Strawberry => {
                            d.draw_text(
                                "S",
                                (d.get_screen_width() / 2) - (250 * i as i32),
                                (d.get_screen_height() / 2) - 200,
                                300,
                                Color::RED,
                            );
                        }
                        SlotType::ShokoladianiKrepi => {
                            d.draw_text(
                                "K",
                                (d.get_screen_width() / 2) - (250 * i as i32),
                                (d.get_screen_height() / 2) - 200,
                                300,
                                Color::RED,
                            );
                        }
                    }
                }
            }
        }
    }
}

impl Slot {
    pub fn new(stype: SlotType) -> Self {
        Self { stype }
    }

    pub fn change(&mut self, stype: SlotType) {
        self.stype = stype;
    }
}
