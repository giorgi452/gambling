use raylib::prelude::*;

pub enum SlotType {
    Seven,
    Strawberry,
    Lemon,
    Badrijani,
}

pub struct ActGame {
    pub money_to_bet: i32,
}

pub struct Slot {
    pub stype: SlotType,
}

pub struct SlotRow {
    pub slots: Vec<Slot>,
}

impl ActGame {
    pub fn new(money_to_bet: i32) -> Self {
        Self { money_to_bet }
    }

    pub fn draw(self, _d: &mut RaylibDrawHandle<'_>) {}
}

impl SlotRow {
    pub fn new(slots: Vec<Slot>) -> Self {
        Self { slots }
    }
}

impl Slot {
    pub fn new(stype: SlotType) -> Self {
        Self { stype }
    }
}
