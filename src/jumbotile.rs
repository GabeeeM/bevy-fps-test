use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct JumboTilePlugin;

impl Plugin for JumboTilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tiles);
    }
}

#[derive(Component)]
pub struct Kovaak;

fn spawn_tiles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tile1 = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            transform: Transform::from_xyz(-15.0, 7.0, 7.0),
            material: materials.add(Color::RED.into()),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(0.5, 0.5, 0.5),
        Kovaak,
    );

    let tile2 = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            transform: Transform::from_xyz(-15.0, 3.0, 3.0),
            material: materials.add(Color::RED.into()),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(0.5, 0.5, 0.5),
        Kovaak,
    );

    commands.spawn(tile1);
    commands.spawn(tile2);
}
