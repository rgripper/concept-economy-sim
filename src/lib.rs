mod bucket;
mod construction;
mod game_items;
mod position;
mod trees;
mod village;
mod worker_tasks;

use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;
use village::{TaskBoard, VillagePlugin, WorldParams};
use wasm_bindgen::prelude::*;

use crate::worker_tasks::WorkerPlugin;

#[wasm_bindgen]
pub fn run() {
    let mut app = App::build();

    let world_params = WorldParams {
        size: Vec2::new(800.0, 800.0),
        villager_count: 4,
    };
    app.insert_resource(world_params);
    app.insert_resource(TaskBoard(VecDeque::new()));

    app.add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.add_plugin(ShapePlugin);

    app.add_plugin(WorkerPlugin);
    app.add_plugin(VillagePlugin);
    // app.add_plugin(TaskListPlugin);
    // app.add_plugin(TreePlugin);
    // app.add_plugin(ConstructionPlugin);
    // app.add_plugin(BuildingPlugin);

    app.run();
}
