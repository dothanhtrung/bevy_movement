//! Move object by mouse click or keyboard in 2D.
//! Object won't go through wall if collider_2d is enabled.

#[cfg(feature = "collider_2d")]
use avian2d::{
    prelude::{
        Collider,
        GravityScale,
        PhysicsDebugPlugin,
        RigidBody,
    },
    PhysicsPlugins,
};
use bevy::color::palettes::basic::WHITE;
use bevy::prelude::*;
use bevy_movement::kb_control::KbMovementObject;
use bevy_movement::linear::LinearMovement;
use bevy_movement::mouse_control::MouseMovementObject;
use bevy_movement::MovementPluginAnyState;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    #[cfg(feature = "collider_2d")]
    app.add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin));

    // Add the plugin MovementPlugin or MovementPluginAnyState
    app.add_plugins(MovementPluginAnyState::any())
        .add_systems(Startup, setup);

    app.run();
}

fn setup(mut commands: Commands) {
    let wall_size = Vec2::new(640., 60.);

    // Wall
    commands.spawn((
        #[cfg(feature = "collider_2d")]
        Collider::rectangle(wall_size.x, wall_size.y),
        #[cfg(feature = "collider_2d")]
        RigidBody::Static,
        Sprite {
            color: WHITE.into(),
            custom_size: Some(wall_size),
            ..default()
        },
        Transform::from_xyz(0.0, 210., 0.0),
    ));
    commands.spawn((
        #[cfg(feature = "collider_2d")]
        Collider::rectangle(wall_size.x, wall_size.y),
        #[cfg(feature = "collider_2d")]
        RigidBody::Static,
        Sprite {
            color: WHITE.into(),
            custom_size: Some(wall_size),
            ..default()
        },
        Transform::from_xyz(0.0, -210., 0.0),
    ));
    commands.spawn((
        #[cfg(feature = "collider_2d")]
        Collider::rectangle(wall_size.y, wall_size.x),
        #[cfg(feature = "collider_2d")]
        RigidBody::Static,
        Sprite {
            color: WHITE.into(),
            custom_size: Some(Vec2::new(wall_size.y, wall_size.x)),
            ..default()
        },
        Transform::from_xyz(-350.0, 0., 0.0),
    ));
    commands.spawn((
        #[cfg(feature = "collider_2d")]
        Collider::rectangle(wall_size.y, wall_size.x),
        #[cfg(feature = "collider_2d")]
        RigidBody::Static,
        Sprite {
            color: WHITE.into(),
            custom_size: Some(Vec2::new(wall_size.y, wall_size.x)),
            ..default()
        },
        Transform::from_xyz(350.0, 0., 0.0),
    ));

    // Movement object
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 3.0, 0.0)),
        KbMovementObject::default(),    // Move by keyboard/gamepad input
        MouseMovementObject::default(), // Move by mouse input
        Sprite {
            color: WHITE.into(),
            custom_size: Some(Vec2::new(64., 64.)),
            ..default()
        },
        #[cfg(feature = "collider_2d")]
        RigidBody::Dynamic,
        #[cfg(feature = "collider_2d")]
        Collider::rectangle(64., 64.),
        #[cfg(feature = "collider_2d")]
        GravityScale(0.),
        LinearMovement {
            speed: 100.,
            ..default()
        },
    ));

    commands.spawn(Camera2d);
}
