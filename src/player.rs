use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player, spawn_camera))
            .add_plugins(EguiPlugin)
            .add_systems(Update, (player_input, sens_slider));
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Paused(bool);

#[derive(Component)]
struct Sensitivity(f32);

#[derive(Component)]
struct Speed(f32);

fn spawn_camera(mut commands: Commands) {
    let camera = {
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.5, 0.0).looking_at(Vec3::X, Vec3::Y),
            ..default()
        }
    };

    commands.spawn(camera);
}

//hi there
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
        Paused(false),
        Sensitivity(0.06),
        Speed(2.0),
        KinematicCharacterController::default(),
    );

    commands.spawn(player);
}

fn player_input(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_q: Query<(&mut Transform, &mut Paused, &mut Sensitivity, &mut Speed), With<Player>>,
    mut cam_q: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut motion_evr: EventReader<MouseMotion>,
) {
    for (mut player_transform, mut player_paused, player_sens, mut player_speed) in
        player_q.iter_mut()
    {
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

        // sprinting
        if keys.pressed(KeyCode::ShiftLeft) {
            player_speed.0 = 50.0;
        } else {
            player_speed.0 = 2.0;
        }

        // "pause"
        if keys.just_pressed(KeyCode::Escape) {
            if player_paused.0 {
                player_paused.0 = false;
            } else {
                player_paused.0 = true;
            }
        }

        // cursor locking
        if player_paused.0 {
            let mut primary_window = q_windows.single_mut();
            primary_window.cursor.grab_mode = CursorGrabMode::None;
            primary_window.cursor.visible = true;
        } else {
            let (mut yaw, mut pitch, _) = cam.rotation.to_euler(EulerRot::YXZ);

            for ev in motion_evr.read() {
                pitch -= (ev.delta.y * player_sens.0).to_radians();
                yaw -= (ev.delta.x * player_sens.0).to_radians();

                cam.rotation =
                    Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }

            let mut primary_window = q_windows.single_mut();
            primary_window.cursor.grab_mode = CursorGrabMode::Locked;
            primary_window.cursor.visible = false;
        }

        // direction.y = 0.0;
        let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
        player_transform.translation += movement;
        cam.translation += movement;
        player_transform.look_to(cam.forward(), Vec3::Y);
    }
}

fn sens_slider(
    mut contexts: EguiContexts,
    mut player_q: Query<(&mut Sensitivity, &Paused), With<Player>>,
) {
    for (mut player_sens, paused) in player_q.iter_mut() {
        if paused.0 {
            egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
                ui.label("Sensitiviy");
                ui.add(egui::DragValue::new(&mut player_sens.0).speed(0.1));
            });
        }
    }
}
