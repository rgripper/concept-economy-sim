use bevy::prelude::Commands;

use crate::{position::Position, construction::{Construction, ConstructionKind}, game_items::GameItemPile};

// Temp module for random stuff

pub fn spawn_house(commands: &mut Commands, position: &Position) {
    commands
        .spawn()
        .insert(Construction {
            kind: ConstructionKind::House,
        })
        .insert(*position)
        .id();
}

pub fn spawn_item_pile(commands: &mut Commands, pile: GameItemPile) {
    commands.spawn().insert(pile).id();
}
