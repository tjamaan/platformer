use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

const PIXELS_PER_METER: f32 = 100.0;

fn main() {
    // create and run the Bevy app
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::Key1)),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER),
            RapierDebugRenderPlugin::default(),
        ))
        .insert_resource(ClearColor(Color::SALMON))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // spawn a 2D camera.
    commands.spawn(Camera2dBundle::default());

    // spawn a blue rectangle sprite
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                // we need to specify custom_size since we didn't load a texture
                custom_size: Some(Vec2::new(30.0, 60.0)),
                ..default()
            },
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(15.0, 30.0),
    ));

    // spawn a gray floor
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GRAY,
                // we need to specify custom_size since we didn't load a texture
                custom_size: Some(Vec2::new(300.0, 30.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -300.0, 0.0)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(150.0, 15.0),
    ));
}
