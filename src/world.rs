use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_floor, spawn_mirror, spawn_light));
    }
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = {
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(800.0))),
            material: materials.add(Color::INDIGO.into()),
            ..default()
        }
    };

    commands.spawn(floor);
}

fn spawn_mirror(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mirror = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 5.0 })),
        transform: Transform::from_xyz(5.0, 2.5, 5.0),
        material: materials.add(StandardMaterial {
            reflectance: 1.0,
            metallic: 1.0,
            ..default()
        }),
        ..default()
    };

    commands.spawn(mirror);
}

fn spawn_light(mut commands: Commands) {
    let ceiling_light = PointLightBundle {
        transform: Transform::from_xyz(0.0, 3.0, 0.0),
        point_light: PointLight {
            intensity: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    };

    commands.spawn(ceiling_light);
}
