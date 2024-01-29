use bevy::prelude::*;

pub struct CrosshairPlugin;

impl Plugin for CrosshairPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_crosshair);
    }
}

fn setup_crosshair(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let thingy = NodeBundle {
        style: Style {
            width: Val::Vw(100.0),
            height: Val::Vh(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };

    let poo = ImageBundle {
        image: UiImage {
            texture: asset_server.load("crosshairtiny.png"),
            ..default()
        },
        ..default()
    };

    commands.spawn(thingy).with_children(|parent| {
        parent.spawn(poo);
    });
}
