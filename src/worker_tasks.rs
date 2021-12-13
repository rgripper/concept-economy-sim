use std::collections::VecDeque;

use bevy::prelude::{AppBuilder, Commands, Entity, IntoSystem, Plugin, Query, ResMut, Without};

use crate::{
    bucket::{spawn_house, spawn_item_pile},
    construction::ConstructionZone,
    game_items::{GameItemKind, GameItemPile},
    position::Position,
};

pub struct Worker;

pub enum WorkerTask {
    CutTree(Entity),
    CarryResourceToConstruction(Entity),
    Construct(Entity),
}

pub struct Carriage(pub Vec<GameItemPile>);

fn progress_with_tasks(
    mut commands: Commands,
    mut task_board: ResMut<TaskBoard>,
    workers: Query<(Entity, &WorkerTask)>,
    mut construction_zones: Query<(&mut ConstructionZone, &Position)>,
    mut carriages: Query<&mut Carriage>,
) {
    for (worker_id, task) in workers.iter() {
        perform_task_by_worker(
            worker_id,
            task,
            &mut task_board,
            &mut commands,
            &mut construction_zones,
            &mut carriages,
        );
    }

    // commands.entity(worker_id).remove::<VillagerAssignment>();
}

fn perform_task_by_worker(
    worker_id: Entity,
    task: &WorkerTask,
    task_board: &mut ResMut<TaskBoard>,
    commands: &mut Commands,
    construction_zones: &mut Query<(&mut ConstructionZone, &Position)>,
    carriages: &mut Query<&mut Carriage>,
) {
    match task {
        WorkerTask::CutTree(tree_id) => {
            commands.entity(*tree_id).despawn();
            spawn_item_pile(
                commands,
                GameItemPile {
                    kind: GameItemKind::WoodenLog,
                    amount: 1,
                },
            );
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

pub struct TaskBoard {
    pub tasks: VecDeque<WorkerTask>,
}

impl Default for TaskBoard {
    fn default() -> Self {
        Self {
            tasks: VecDeque::new(),
        }
    }
}

pub struct WorkerPlugin;

impl Plugin for WorkerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(assign_tasks.system());
        app.add_system(progress_with_tasks.system());
    }
}

fn assign_tasks(
    mut commands: Commands,
    mut task_board: ResMut<TaskBoard>,
    idle_workers: Query<Entity, Without<WorkerTask>>,
) {
    for worker_id in idle_workers.iter() {
        if let Option::Some(task) = task_board.tasks.pop_front() {
            commands.entity(worker_id).insert(task);
        } else {
            break;
        }
    }
}
