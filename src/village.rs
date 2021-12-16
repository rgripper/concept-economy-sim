use std::collections::VecDeque;

use bevy::{
    math::{Vec2, Vec3},
    prelude::{
        AppBuilder, Color, Commands, Entity, IntoSystem, OrthographicCameraBundle, Plugin, Query,
        Res, SystemStage, Transform, Without,
    },
};
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillOptions, GeometryBuilder, ShapeColors, StrokeOptions},
    shapes,
};
use rand::Rng;

use crate::{
    construction::spawn_construction_zone,
    position::Position,
    trees::spawn_tree,
    worker_tasks::{Carriage, Worker, WorkerTask, WorkerTaskQue},
};

pub struct WorldParams {
    pub size: Vec2,
    pub villager_count: u32,
}

pub struct VillagePlugin;

impl Plugin for VillagePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage("game_setup", SystemStage::single(plan_houses.system()));
        // app.add_system(assign_tasks.system());
        // app.add_system(progress_with_tasks.system());
    }
}

// TODO: this shoudl actually go to World init plugin

fn plan_houses(
    world_params: Res<WorldParams>,
    mut idle_workers: Query<Entity, Without<WorkerTaskQue>>,
    mut commands: Commands,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let mut rng = rand::thread_rng();
    let world_half = world_params.size / 2.0;

    let mut gen_position = || {
        Position(Vec3::new(
            rng.gen_range(-world_half.x..world_half.x),
            rng.gen_range(-world_half.y..world_half.y),
            0.0,
        ))
    };

    for _ in 0..20 {
        spawn_tree(&mut commands, &gen_position());
    }

    for _ in 0..world_params.villager_count {
        spawn_worker(&mut commands, &gen_position());

        let construction_zone_id = spawn_construction_zone(&mut commands, &gen_position());

        plan_construction_zone(&mut commands, &mut idle_workers, construction_zone_id)
    }
}

fn plan_construction_zone(
    commands: &mut Commands,
    idle_workers: &mut Query<Entity, Without<WorkerTaskQue>>,
    construction_zone_id: Entity,
) {
    let worker_id = idle_workers.iter().next().unwrap();
    commands
        .entity(worker_id)
        .insert(WorkerTaskQue(VecDeque::from([
            WorkerTask::CutTree,
            WorkerTask::CarryResourceToConstruction(construction_zone_id),
            WorkerTask::Construct(construction_zone_id),
        ])));
}

fn spawn_worker(commands: &mut Commands, position: &Position) {
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(200.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        ShapeColors::outlined(Color::TEAL, Color::BLACK),
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(10.0),
        },
        Transform {
            translation: position.0,
            ..Transform::default()
        },
    ));

    commands
        .spawn()
        .insert(Worker)
        .insert(Carriage(vec![]))
        .insert(*position);
}
