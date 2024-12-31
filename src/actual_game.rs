use raylib::prelude::*;

use crate::game::{gamescene::Scene, keyboard::Keyboard};

pub struct ActGame<'a> {
    pub money_to_bet: &'a mut i32,
    slot_rows: Vec<Slot>,
}

impl<'a> ActGame<'a> {
    pub fn new(money_to_bet: &'a mut i32) -> Self {
        Self {
            money_to_bet,
            slot_rows: vec![
                Slot::new(SlotType::Seven),
                Slot::new(SlotType::Badrijani),
                Slot::new(SlotType::ShokoladianiKrepi),
            ],
        }
    }

    pub fn bet(money_to_bet: &'a mut i32, d: &mut RaylibDrawHandle, keyboard: &mut Keyboard) {
        d.draw_text(
            "Bet",
            (d.get_screen_width() / 2) - 35,
            (d.get_screen_height() / 2) - 250,
            70,
            Color::new(0x73, 0x77, 0xBD, 0xFF),
        );

        d.draw_rectangle(
            (d.get_screen_width() / 2) - 155,
            (d.get_screen_height() / 2) - 150,
            350,
            50,
            Color::new(0x19, 0x1A, 0x28, 0xFF),
        );

        keyboard.get_keys(d, money_to_bet);
        keyboard.draw_keys(d);
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, game_scene: &mut Scene) {
        d.draw_text(&self.money_to_bet.to_string(), 10, 10, 30, Color::RED);

        if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
            self.randomize_slots(d);
            *self.money_to_bet -= 1000;
        } else if d.is_key_pressed(KeyboardKey::KEY_E) {
            *game_scene = Scene::MainMenu;
        }

        self.draw_slots(d);

        if let Some(multiplier) = self.check_winning_conditions() {
            *self.money_to_bet *= multiplier;
            *game_scene = Scene::Won;
        }
    }

    fn randomize_slots(&mut self, d: &mut RaylibDrawHandle) {
        for slot in &mut self.slot_rows {
            let row = d.get_random_value(0..4);
            let slot_type = match row {
                0 => SlotType::Seven,
                1 => SlotType::Badrijani,
                2 => SlotType::Lemon,
                3 => SlotType::Strawberry,
                4 => SlotType::ShokoladianiKrepi,
                _ => unreachable!(),
            };
            slot.change(slot_type);
        }
    }

    fn draw_slots(&self, d: &mut RaylibDrawHandle) {
        const SLOT_COUNT: usize = 3;
        const SLOT_GAP: i32 = 150;
        const SLOT_FONT_SIZE: i32 = 700;
        const SLOT_TEXT_WIDTH: i32 = SLOT_FONT_SIZE / 2;

        let total_width =
            (SLOT_COUNT as i32 * SLOT_TEXT_WIDTH) + ((SLOT_COUNT as i32 - 1) * SLOT_GAP);
        let start_x = (d.get_screen_width() - total_width) / 2;
        let center_y = (d.get_screen_height() / 2) - (SLOT_FONT_SIZE / 2);

        for (i, slot) in self.slot_rows.iter().enumerate() {
            let x_position = start_x + i as i32 * (SLOT_TEXT_WIDTH + SLOT_GAP);
            d.draw_text(
                &slot.stype.to_string(),
                x_position,
                center_y,
                SLOT_FONT_SIZE,
                Color::new(0xB1, 0x0, 0x0, 0xFF),
            );
        }
    }

    fn check_winning_conditions(&self) -> Option<i32> {
        if self
            .slot_rows
            .iter()
            .all(|slot| slot.stype == self.slot_rows[0].stype)
        {
            match self.slot_rows[0].stype {
                SlotType::Seven => Some(4),
                SlotType::Lemon => Some(2),
                SlotType::Strawberry => Some(3),
                SlotType::ShokoladianiKrepi => Some(5),
                _ => Some(1),
            }
        } else {
            None
        }
    }
}

// Slot and SlotType implementation
#[derive(Clone, PartialEq)]
pub enum SlotType {
    Seven,
    Badrijani,
    Lemon,
    Strawberry,
    ShokoladianiKrepi,
}

impl ToString for SlotType {
    fn to_string(&self) -> String {
        match self {
            SlotType::Seven => "7",
            SlotType::Badrijani => "B",
            SlotType::Lemon => "L",
            SlotType::Strawberry => "S",
            SlotType::ShokoladianiKrepi => "K",
        }
        .to_string()
    }
}

pub struct Slot {
    pub stype: SlotType,
}

impl Slot {
    pub fn new(stype: SlotType) -> Self {
        Self { stype }
    }

    pub fn change(&mut self, new_type: SlotType) {
        self.stype = new_type;
    }
}
