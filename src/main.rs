// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_arch = "wasm32")]
use bevy_webgl2;

use bevy::prelude::{App, ClearColor, Color, WindowDescriptor};
use bevy::DefaultPlugins;
use game_plugin::game_map::GameMap;
use game_plugin::GamePlugin;

fn main() {
    let game_map = GameMap::default();

    let mut app = App::build();
    app
        // .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(
            121. / 255.,
            179. / 255.,
            206. / 255.,
        )))
        .insert_resource(WindowDescriptor {
            width: (game_map.width + 2. * game_map.pad_x) * game_map.sprite_size,
            height: (game_map.height + 2. * game_map.pad_y) * game_map.sprite_size,
            title: "Cloud Surfer".to_string(),
            ..Default::default()
        })
        .insert_resource(game_map)
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.run();
}
