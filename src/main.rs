use bevy::{app::App, DefaultPlugins};
use bevy_rapier3d::prelude::*;

mod player;
// mod sphere;
mod world;

use player::PlayerPlugin;
// use sphere::SpherePlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            // RapierDebugRenderPlugin::default(),
            PlayerPlugin,
            WorldPlugin,
        ))
        .run();
}
