#[cfg(feature = "physic_2d")]
use avian2d::{
    prelude::{
        LinearVelocity,
        RigidBody,
    },
    PhysicsPlugins,
};
use bevy::color::palettes::basic::WHITE;
use bevy::prelude::*;
use bevy_movement::linear::{
    LinearDestination,
    LinearMovement,
};
use bevy_movement::{
    Arrived,
    MovementPluginAnyState,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            #[cfg(feature = "physic_2d")]
            PhysicsPlugins::default(),
        ))
        .add_plugins(MovementPluginAnyState::any())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    let id = commands
        .spawn((
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            Sprite {
                color: WHITE.into(),
                custom_size: Some(Vec2::new(64., 64.)),
                ..default()
            },
            LinearMovement {
                speed: 50.,
                ..default()
            },
            #[cfg(feature = "physic_2d")]
            LinearVelocity::default(),
            #[cfg(feature = "physic_2d")]
            RigidBody::Kinematic,
        ))
        .observe(arrived)
        .id();

    commands.spawn(Camera2d);

    // Just for example.
    // Trigger function 'arrived' below to add new destination
    commands.trigger(Arrived { entity: id });
}

#[derive(Component)]
struct Target;
fn arrived(
    trigger: On<Arrived>,
    mut query: Query<&mut LinearMovement>,
    mut commands: Commands,
    target: Query<Entity, With<Target>>,
) {
    for entity in target.iter() {
        commands.entity(entity).despawn();
    }
    if let Ok(mut movement) = query.get_mut(trigger.entity) {
        let next_target = Vec3::new(fastrand::i32(-640..640) as f32, fastrand::i32(-360..360) as f32, 0.);

        movement.des.push(LinearDestination::from_pos(next_target));

        commands.spawn((
            Sprite {
                color: WHITE.into(),
                custom_size: Some(Vec2::new(32., 32.)),
                ..default()
            },
            Transform::from_translation(next_target),
            Target,
        ));
    }
}
