use bevy::{app::App, DefaultPlugins};
use bevy_rapier3d::prelude::*;

mod player;
mod world;

use player::PlayerPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            WorldPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
        ))
        .run();
}
