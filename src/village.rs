use std::{iter, thread};

use bevy::{
    math::{Vec2, Vec3},
    prelude::{AppBuilder, Commands, IntoSystem, Plugin, Res, SystemStage},
};
use rand::Rng;

use crate::{
    construction::{spawn_construction_zone, ConstructionZone},
    position::Position,
    worker_tasks::{Carriage, Worker},
};

pub struct WorldParams {
    size: Vec2,
    villager_count: u32,
}

pub struct VillagePlugin;

impl Plugin for VillagePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage("game_setup", SystemStage::single(plan_houses.system()));
        // app.add_system(assign_tasks.system());
        // app.add_system(progress_with_tasks.system());
    }
}

fn plan_houses(world_params: Res<WorldParams>, mut commands: Commands) {
    let mut rng = rand::thread_rng();
    let world_half = world_params.size / 2.0;

    let mut gen_position = || {
        Position(Vec3::new(
            rng.gen_range(-world_half.x..world_half.x),
            rng.gen_range(-world_half.y..world_half.y),
            0.0,
        ))
    };

    for _ in 0..world_params.villager_count {
        spawn_worker(&mut commands, &gen_position());
        spawn_construction_zone(&mut commands, &gen_position());
    }
}

fn spawn_worker(commands: &mut Commands, position: &Position) {
    commands
        .spawn()
        .insert(Worker)
        .insert(Carriage(vec![]))
        .insert(*position);
}
