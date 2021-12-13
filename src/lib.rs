pub mod bucket;
pub mod construction;
pub mod game_items;
pub mod position;
pub mod worker_tasks;

use bevy::prelude::*;
use wasm_bindgen::prelude::*;

use crate::worker_tasks::WorkerPlugin;

#[wasm_bindgen]
pub fn run() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.add_plugin(WorkerPlugin);
    // app.add_plugin(TaskListPlugin);
    // app.add_plugin(TreePlugin);
    // app.add_plugin(ConstructionPlugin);
    // app.add_plugin(BuildingPlugin);

    app.run();
}
