use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct SpherePlugin;

impl Plugin for SpherePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_system)
            .add_systems(Update, update_system);
    }
}

fn init_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ball = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.5,
                ..default()
            })),
            material: materials.add(Color::CRIMSON.into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        RigidBody::KinematicVelocityBased,
        Collider::ball(0.5),
        KinematicCharacterController::default(),
    );

    commands.spawn(ball);
}

fn update_system(
    mut controllers: Query<&mut KinematicCharacterController>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for mut controller in controllers.iter_mut() {
        let mut direction = Vec3::ZERO;

        // forward
        if keys.pressed(KeyCode::Up) {
            direction += Vec3::new(5.0, 0.0, 0.0);
        }

        // back
        if keys.pressed(KeyCode::Down) {
            direction += Vec3::new(-5.0, 0.0, 0.0);
        }

        // left
        if keys.pressed(KeyCode::Left) {
            direction += Vec3::new(1.0, 0.0, -5.0);
        }

        // right
        if keys.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 5.0);
        }

        let movement = direction.normalize_or_zero() * time.delta_seconds();

        controller.translation = Some(movement);
    }
}
