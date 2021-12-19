use bevy::prelude::{Color, Commands, Entity, Transform};
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillOptions, GeometryBuilder, ShapeColors, StrokeOptions},
    shapes,
};

use crate::{
    game_items::{GameItemKind, GameItemPile},
    position::Position,
};

#[derive(Debug)]
pub struct ConstructionZone {
    pub items_needed: Vec<GameItemPile>,
    pub items_ready: Vec<GameItemPile>,
    pub kind: ConstructionKind,
}

pub struct Construction {
    pub kind: ConstructionKind,
}

#[derive(Debug)]
pub enum ConstructionKind {
    House,
}

pub fn spawn_construction_zone(commands: &mut Commands, position: &Position) -> Entity {
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shapes::Rectangle {
                height: 20.0,
                width: 20.0,
                ..shapes::Rectangle::default()
            },
            ShapeColors::outlined(Color::NONE, Color::ORANGE),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(1.0),
            },
            Transform {
                translation: position.0,
                ..Transform::default()
            },
        ))
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
