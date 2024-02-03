use bevy::{
    audio::{Volume, VolumeLevel},
    core_pipeline::bloom::BloomSettings,
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use crate::jumbotile::Kovaak;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player, spawn_camera))
            .add_plugins(EguiPlugin)
            .add_systems(Update, (player_input, sens_slider, toggle_bloom, shot_tar))
            .add_event::<BloomEvent>()
            .add_event::<ShotTar>();
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
    let camera = (
        PerspectiveProjection {
            fov: 1.79769,
            ..default()
        },
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.5, 0.0).looking_at(Vec3::X, Vec3::Y),
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        BloomSettings::NATURAL,
    );

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
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.5,
                ..default()
            })),
            material: materials.add(Color::CRIMSON.into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player,
        Paused(false),
        Sensitivity(0.06),
        Speed(2.0),
        RigidBody::Dynamic,
        Collider::ball(0.5),
        Velocity::default(),
        LockedAxes::ROTATION_LOCKED,
    );

    commands.spawn(player);
}

#[derive(Event)]
struct ShotTar(Entity);

fn player_input(
    keys: Res<Input<KeyCode>>,
    mouse_buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
    mut player_q: Query<
        (
            &Transform,
            &mut Paused,
            &mut Sensitivity,
            &mut Speed,
            &mut Velocity,
        ),
        With<Player>,
    >,
    rapier_context: Res<RapierContext>,
    mut cam_q: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut shot_tar: EventWriter<ShotTar>,
) {
    for (player_transform, mut player_paused, player_sens, mut player_speed, mut velocity) in
        player_q.iter_mut()
    {
        let mut direction = Vec3::ZERO;
        let mut cam = cam_q.get_single_mut().unwrap();
        let hit = rapier_context.cast_ray(
            player_transform.translation - Vec3::new(0.0, 0.6, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            0.2,
            true,
            QueryFilter::only_fixed(),
        );

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

        // jump
        if keys.pressed(KeyCode::Space) && hit.is_some() {
            velocity.linvel.y = 10.0;
        }

        // sprinting
        if keys.pressed(KeyCode::ShiftLeft) {
            player_speed.0 = 30.0;
        } else {
            player_speed.0 = 5.0;
        }

        // "pause"
        if keys.just_pressed(KeyCode::Escape) {
            if player_paused.0 {
                player_paused.0 = false;
            } else {
                player_paused.0 = true;
            }
        }

        // shoot
        if mouse_buttons.just_pressed(MouseButton::Left) && !player_paused.0 {
            if let Some((entity, _distance)) = rapier_context.cast_ray(
                player_transform.translation,
                cam.forward(),
                100.0,
                true,
                QueryFilter::only_fixed(),
            ) {
                println!("{:?}", entity);
                shot_tar.send(ShotTar(entity));
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

        let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
        velocity.linvel.x = movement.x * 100.0;
        velocity.linvel.z = movement.z * 100.0;

        cam.translation = player_transform.translation;
        // player_transform.look_to(cam.forward(), Vec3::Y);
    }
}

fn shot_tar(
    mut events: EventReader<ShotTar>,
    mut query: Query<&mut Transform, With<Kovaak>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for ShotTar(entity) in events.read() {
        if let Ok(mut shot_thing) = query.get_mut(*entity) {
            shot_thing.translation = Vec3::new(
                -15.0,
                fastrand::i32(100..900) as f32 / 100.0,
                fastrand::i32(100..900) as f32 / 100.0,
            );
            commands.spawn(AudioBundle {
                source: asset_server.load("Hitsound.ogg"),
                settings: PlaybackSettings {
                    volume: Volume::Relative(VolumeLevel::new(0.1)),
                    ..default()
                },
            });
        }

        // if let Ok(shot_thing) = query.get(*entity) {}

        // if let Ok(material_handle) = query.get(*entity) {
        //     if let Some(material) = materials.get_mut(material_handle) {
        //         material.base_color = Color::rgb(
        //             fastrand::i32(0..10) as f32 / 10.0,
        //             fastrand::i32(0..10) as f32 / 10.0,
        //             fastrand::i32(0..10) as f32 / 10.0,
        //         );
        //     }
        // }
    }
}

#[derive(Event)]
struct BloomEvent;

fn sens_slider(
    mut contexts: EguiContexts,
    mut player_q: Query<(&mut Sensitivity, &Paused), With<Player>>,
    mut camera_q: Query<&mut PerspectiveProjection>,
    mut bloom_e: EventWriter<BloomEvent>,
) {
    for (mut player_sens, paused) in player_q.iter_mut() {
        for mut camera in camera_q.iter_mut() {
            if paused.0 {
                egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
                    ui.label("Sensitiviy");
                    ui.add(egui::DragValue::new(&mut player_sens.0).speed(0.01));
                    if ui.add(egui::Button::new("Bloom")).clicked() {
                        bloom_e.send(BloomEvent);
                    }
                    ui.label("Fov");
                    ui.add(egui::DragValue::new(&mut camera.fov).speed(0.05));
                });
            }
        }
    }
}

fn toggle_bloom(
    mut cam_q: Query<&mut Camera, (With<Camera3d>, Without<Player>)>,
    mut events: EventReader<BloomEvent>,
) {
    if events.read().next().is_some() {
        for mut cam_settings in cam_q.iter_mut() {
            cam_settings.hdr = !cam_settings.hdr;
        }
    }
}
