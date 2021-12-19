use std::collections::VecDeque;

use bevy::{
    math::{Vec2, Vec3},
    prelude::{
        AppBuilder, Color, Commands, Entity, IntoSystem, OrthographicCameraBundle, Plugin, Query,
        Res, ResMut, SystemStage, Transform, With, Without,
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
    trees::{spawn_tree, Tree},
    worker_tasks::{Carriage, Worker, WorkerTask, WorkerTaskQue},
};

pub struct WorldParams {
    pub size: Vec2,
    pub villager_count: u32,
}

pub struct VillagePlugin;

impl Plugin for VillagePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage("game_setup", SystemStage::single(init.system()));
        app.add_startup_stage("game_setup2", SystemStage::single(plan_houses.system()));
        app.add_system(assign_tasks.system());
    }
}

// TODO: this shoudl actually go to World init plugin

fn init(world_params: Res<WorldParams>, mut commands: Commands) {
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
    }
}

pub struct TaskBoard(pub VecDeque<WorkerTaskQue>);

fn plan_houses(
    world_params: Res<WorldParams>,
    mut commands: Commands,
    mut task_board: ResMut<TaskBoard>,
    trees: Query<Entity, With<Tree>>,
) {
    let mut rng = rand::thread_rng();
    let world_half = world_params.size / 2.0;

    let mut gen_position = || {
        Position(Vec3::new(
            rng.gen_range(-world_half.x..world_half.x),
            rng.gen_range(-world_half.y..world_half.y),
            0.0,
        ))
    };

    let mut tree_iter = trees.iter();

    for _ in 0..world_params.villager_count {
        let construction_zone_id = spawn_construction_zone(&mut commands, &gen_position());

        let tree_id = tree_iter.next().unwrap();

        task_board.0.push_back(WorkerTaskQue(VecDeque::from([
            WorkerTask::CutTree(tree_id),
            WorkerTask::CarryResourceToConstruction(construction_zone_id),
            WorkerTask::Construct(construction_zone_id),
        ])));
        // plan_construction_zone(&mut commands, &mut idle_workers, construction_zone_id)
    }
}

fn assign_tasks(
    mut commands: Commands,
    mut idle_workers: Query<Entity, (With<Worker>, Without<WorkerTaskQue>)>,
    mut task_board: ResMut<TaskBoard>,
) {
    for worker_id in idle_workers.iter_mut() {
        if let Some(task_que) = task_board.0.pop_front() {
            commands.entity(worker_id).insert(task_que);
        } else {
            break;
        }
    }
}

fn spawn_worker(commands: &mut Commands, position: &Position) {
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shapes::Circle {
                radius: 4.0,
                ..shapes::Circle::default()
            },
            ShapeColors::outlined(Color::NONE, Color::BEIGE),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(1.0),
            },
            Transform {
                translation: position.0,
                ..Transform::default()
            },
        ))
        .insert(Worker)
        .insert(Carriage(vec![]))
        .insert(*position);
}
