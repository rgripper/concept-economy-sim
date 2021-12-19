use std::collections::VecDeque;

use bevy::prelude::{AppBuilder, Commands, Entity, IntoSystem, Plugin, Query, With};

use crate::{
    bucket::spawn_house,
    construction::ConstructionZone,
    game_items::{GameItemKind, GameItemPile},
    position::Position,
    trees::Tree,
};

pub struct Worker;

#[derive(Debug)]
pub enum WorkerTask {
    CutTree,
    CarryResourceToConstruction(Entity),
    Construct(Entity),
}

pub struct Carriage(pub Vec<GameItemPile>);

fn progress_with_tasks(
    mut commands: Commands,
    mut workers: Query<(Entity, &mut WorkerTaskQue)>,
    mut construction_zones: Query<(&mut ConstructionZone, &Position)>,
    mut trees: Query<Entity, With<Tree>>,
    mut carriages: Query<&mut Carriage>,
) {
    for (worker_id, mut task_que) in workers.iter_mut() {
        if let Some(task) = task_que.0.pop_front() {
            perform_task_by_worker(
                worker_id,
                &task,
                &mut commands,
                &mut construction_zones,
                &mut trees,
                &mut carriages,
            );
        }
    }

    // commands.entity(worker_id).remove::<VillagerAssignment>();
}

fn perform_task_by_worker(
    worker_id: Entity,
    task: &WorkerTask,
    commands: &mut Commands,
    construction_zones: &mut Query<(&mut ConstructionZone, &Position)>,
    trees: &mut Query<Entity, With<Tree>>,
    carriages: &mut Query<&mut Carriage>,
) {
    println!("perform_task_by_worker {:?}", task);
    match task {
        WorkerTask::CutTree => {
            let tree_id = trees.iter().next().unwrap();
            commands.entity(tree_id).despawn();
            let mut carriage = carriages.get_mut(worker_id).unwrap();
            carriage.0.push(GameItemPile {
                kind: GameItemKind::WoodenLog,
                amount: 1,
            });
        }
        WorkerTask::CarryResourceToConstruction(costruction_zone_id) => {
            let (mut zone, position) = construction_zones.get_mut(*costruction_zone_id).unwrap();
            // TODO: Dummy resource transfer logic for now
            let mut carriage = carriages.get_mut(worker_id).unwrap();
            let pile = carriage.0.pop().unwrap();
            zone.items_needed.pop();
            zone.items_ready.push(pile);
        }
        WorkerTask::Construct(construction_zone_id) => {
            // TODO: right now it throws if entity disappears
            let (mut zone, position) = construction_zones.get_mut(*construction_zone_id).unwrap();
            // TODO: Dummy construction logic for now
            zone.items_ready.pop();

            if zone.items_needed.len() == 0 && zone.items_ready.len() == 0 {
                commands.entity(*construction_zone_id).despawn();
                spawn_house(commands, position);
            }
        }
    }
}

pub struct WorkerTaskQue(pub VecDeque<WorkerTask>);

pub struct WorkerPlugin;

impl Plugin for WorkerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(progress_with_tasks.system());
    }
}
