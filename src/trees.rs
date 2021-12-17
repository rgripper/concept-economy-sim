use crate::position::Position;
use bevy::{
    math::Vec2,
    prelude::{Color, Commands, Transform},
};
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillOptions, GeometryBuilder, ShapeColors, StrokeOptions},
    shapes,
};

pub struct Tree;

pub fn spawn_tree(commands: &mut Commands, position: &Position) {
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shapes::Polygon {
                closed: true,
                points: vec![
                    Vec2::new(-6.0, -6.0),
                    Vec2::new(6.0, -6.0),
                    Vec2::new(0.0, 6.0),
                ],
            },
            ShapeColors::outlined(Color::NONE, Color::DARK_GREEN),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(1.0),
            },
            Transform {
                translation: position.0,
                ..Transform::default()
            },
        ))
        .insert(*position);
}
