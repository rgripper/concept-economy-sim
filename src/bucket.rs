use bevy::prelude::{Color, Commands, Transform};
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillOptions, GeometryBuilder, ShapeColors, StrokeOptions},
    shapes,
};

use crate::{
    construction::{Construction, ConstructionKind},
    game_items::GameItemPile,
    position::Position,
};

// Temp module for random stuff

pub fn spawn_house(commands: &mut Commands, position: &Position) {
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shapes::Rectangle {
                height: 20.0,
                width: 20.0,
                ..shapes::Rectangle::default()
            },
            ShapeColors::outlined(Color::ANTIQUE_WHITE, Color::ORANGE),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(1.0),
            },
            Transform {
                translation: position.0,
                ..Transform::default()
            },
        ))
        .insert(Construction {
            kind: ConstructionKind::House,
        })
        .insert(*position)
        .id();
}
