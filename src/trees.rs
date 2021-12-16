use bevy::prelude::Commands;

use crate::position::Position;

pub struct Tree;

pub fn spawn_tree(commands: &mut Commands, position: &Position) {
    commands.spawn().insert(*position);
}
