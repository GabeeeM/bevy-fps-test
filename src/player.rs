use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player, spawn_camera))
            .add_systems(Update, (player_input));
    }
}

#[derive(Component)]
struct Player;

fn spawn_camera(mut commands: Commands) {
    let camera = {
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.5, 0.0).looking_at(Vec3::X, Vec3::Y),
            ..default()
        }
    };

    commands.spawn(camera);
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::CRIMSON.into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player,
    );

    commands.spawn(player);
}

fn player_input(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_q: Query<&mut Transform, With<Player>>,
    mut cam_q: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut motion_evr: EventReader<MouseMotion>,
) {
    for mut player_transform in player_q.iter_mut() {
        let mut direction = Vec3::ZERO;
        let mut cam = cam_q.get_single_mut().unwrap();

        // forward
        if keys.pressed(KeyCode::W) {
            direction += cam.forward();
        }

        // back
        if keys.pressed(KeyCode::S) {
            direction += cam.back();
        }

        // left
        if keys.pressed(KeyCode::A) {
            direction += cam.left();
        }

        // right
        if keys.pressed(KeyCode::D) {
            direction += cam.right();
        }

        // cursor locking
        if keys.pressed(KeyCode::ControlLeft) {
            let mut primary_window = q_windows.single_mut();
            primary_window.cursor.grab_mode = CursorGrabMode::None;
            primary_window.cursor.visible = true;
        } else {
            let (mut yaw, mut pitch, _) = cam.rotation.to_euler(EulerRot::YXZ);

            for ev in motion_evr.read() {
                pitch -= (ev.delta.y).to_radians();
                yaw -= (ev.delta.x).to_radians();

                cam.rotation =
                    Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }

            let mut primary_window = q_windows.single_mut();
            primary_window.cursor.grab_mode = CursorGrabMode::Locked;
            primary_window.cursor.visible = false;
        }

        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * 2.0 * time.delta_seconds();
        player_transform.translation += movement;
        cam.translation += movement;
        player_transform.look_to(cam.forward(), Vec3::Y);
    }
}

// fn mouse_motion(mut cam_q: Query<&mut Transform, (With<Camera3d>, Without<Player>)>) {
//     let mut cam = cam_q.get_single_mut().unwrap();
//     let (mut yaw, mut pitch, _) = cam.rotation.to_euler(EulerRot::YXZ);

//     for ev in motion_evr.read() {
//         pitch -= (ev.delta.y).to_radians();
//         yaw -= (ev.delta.x).to_radians();

//         cam.rotation = Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
//     }
// }
