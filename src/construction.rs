use bevy::prelude::{Commands, Entity};

use crate::{
    game_items::{GameItemKind, GameItemPile},
    position::Position,
};

pub struct ConstructionZone {
    pub items_needed: Vec<GameItemPile>,
    pub items_ready: Vec<GameItemPile>,
    pub kind: ConstructionKind,
}

pub struct Construction {
    pub kind: ConstructionKind,
}

pub enum ConstructionKind {
    House,
}

pub fn spawn_construction_zone(commands: &mut Commands, position: &Position) -> Entity {
    commands
        .spawn()
        .insert(ConstructionZone {
            kind: ConstructionKind::House,
            items_needed: vec![GameItemPile {
                kind: GameItemKind::WoodenLog,
                amount: 2,
            }],
            items_ready: vec![],
        })
        .insert(*position)
        .id()
}
