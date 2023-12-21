use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    // create and run the Bevy app
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::Key1)),
        ))
        .insert_resource(ClearColor(Color::SALMON))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // spawn a 2D camera.
    commands.spawn(Camera2dBundle::default());

    // spawn a red rectangle sprite
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLUE,
            // we need to specify custom_size since we didn't load a texture
            custom_size: Some(Vec2::new(30.0, 60.0)),
            ..default()
        },
        ..default()
    });
}
