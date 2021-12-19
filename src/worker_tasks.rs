use std::collections::VecDeque;

use bevy::prelude::{AppBuilder, Commands, Entity, IntoSystem, Plugin, Query, Transform, Without};

use crate::{
    bucket::spawn_house,
    construction::ConstructionZone,
    game_items::{GameItemKind, GameItemPile},
    position::Position,
};

pub struct Worker;

pub struct MovingTo(Entity);

#[derive(Debug)]
pub enum WorkerTask {
    CutTree(Entity),
    CarryResourceToConstruction(Entity),
    Construct(Entity),
}

pub struct Carriage(pub Vec<GameItemPile>);

fn progress_with_tasks(
    mut commands: Commands,
    mut workers: Query<(Entity, &mut WorkerTaskQue), Without<MovingTo>>,
    mut construction_zones: Query<(&mut ConstructionZone, &Position)>,
    positions: Query<&Position>,
    mut carriages: Query<&mut Carriage>,
) {
    for (worker_id, mut task_que) in workers.iter_mut() {
        if let Some(task) = task_que.0.front() {
            if perform_task_by_worker(
                worker_id,
                &task,
                &mut commands,
                &positions,
                &mut construction_zones,
                &mut carriages,
            ) {
                task_que.0.pop_front();
            }
        }
    }

    // commands.entity(worker_id).remove::<VillagerAssignment>();
}

fn perform_task_by_worker(
    worker_id: Entity,
    task: &WorkerTask,
    commands: &mut Commands,
    positions: &Query<&Position>,
    construction_zones: &mut Query<(&mut ConstructionZone, &Position)>,
    carriages: &mut Query<&mut Carriage>,
) -> bool {
    println!("perform_task_by_worker {:?}", task);
    match task {
        // WorkerTask::PickTree => {
        //     let tree_id = trees.iter().next().unwrap();
        // }
        WorkerTask::CutTree(tree_id) => {
            if !is_near_target(positions, *tree_id, worker_id, commands, 20.0) {
                return false;
            }

            commands.entity(*tree_id).despawn();
            let mut carriage = carriages.get_mut(worker_id).unwrap();
            carriage.0.push(GameItemPile {
                kind: GameItemKind::WoodenLog,
                amount: 1,
            });
        }
        WorkerTask::CarryResourceToConstruction(construction_zone_id) => {
            if !is_near_target(positions, *construction_zone_id, worker_id, commands, 20.0) {
                return false;
            }
            let (mut zone, position) = construction_zones.get_mut(*construction_zone_id).unwrap();
            // TODO: Dummy resource transfer logic for now
            let mut carriage = carriages.get_mut(worker_id).unwrap();
            let pile = carriage.0.pop().unwrap();
            println!("CarryResourceToConstruction before {:?}", zone);

            zone.items_needed.pop();
            zone.items_ready.push(pile);
            println!("CarryResourceToConstruction after {:?}", zone);
        }
        WorkerTask::Construct(construction_zone_id) => {
            if !is_near_target(positions, *construction_zone_id, worker_id, commands, 20.0) {
                return false;
            }
            // TODO: right now it throws if entity disappears
            let (mut zone, position) = construction_zones.get_mut(*construction_zone_id).unwrap();
            // TODO: Dummy construction logic for now
            zone.items_ready.pop();

            println!("Construct {:?}", zone);

            if zone.items_needed.len() == 0 && zone.items_ready.len() == 0 {
                commands.entity(*construction_zone_id).despawn();
                spawn_house(commands, position);
            }
        }
    }

    return true;
}

fn is_near_target(
    positions: &Query<&Position>,
    target_id: Entity,
    worker_id: Entity,
    commands: &mut Commands,
    distance: f32,
) -> bool {
    let Position(target_coords) = positions.get(target_id).unwrap();
    let Position(worker_coords) = positions.get(worker_id).unwrap();
    if target_coords.distance(*worker_coords) > distance {
        // println!(
        //     "is_near_target {:?} {:?}  false",
        //     target_coords, worker_coords
        // );

        commands.entity(worker_id).insert(MovingTo(target_id));
        return false;
    }
    true
}

fn moving(
    moving: Query<(Entity, &MovingTo)>,
    mut positions: Query<(&mut Position, &mut Transform)>,
    mut commands: Commands,
) {
    for (entity_id, MovingTo(target_id)) in moving.iter() {
        let velocity = 3.0;
        let target_pos = positions.get_mut(*target_id).unwrap().0 .0;
        let (mut this_pos_res, mut this_transform) = positions.get_mut(entity_id).unwrap();
        // println!(
        //     "moving {:?} to {:?} => {:?} to {:?}",
        //     entity_id, target_id, this_pos_res.0, target_pos
        // );

        let distance = this_pos_res.0.distance(target_pos.into());
        if distance > 10.0 {
            this_pos_res.0 = this_pos_res.0.lerp(target_pos, velocity / distance);
            this_transform.translation = this_pos_res.0;
        } else {
            commands.entity(entity_id).remove::<MovingTo>();
        }
    }
}

pub struct WorkerTaskQue(pub VecDeque<WorkerTask>);

pub struct WorkerPlugin;

impl Plugin for WorkerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(progress_with_tasks.system());
        app.add_system(moving.system());
    }
}
