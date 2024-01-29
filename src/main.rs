use bevy::{app::App, math::Vec3, utils::default, DefaultPlugins};
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
        .insert_resource(RapierConfiguration {
            gravity: Vec3::from((0.0, -10.0, 0.0)),
            ..default()
        })
        .run();
}
