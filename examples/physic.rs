use bevy::prelude::*;
use bevy_movement::physic::{PhysicDestination, PhysicMovement};
use bevy_movement::MovementPluginAnyState;
use bevy_rapier3d::prelude::GravityScale;
#[cfg(feature = "physic")]
use bevy_rapier3d::prelude::{Collider, NoUserData, RapierPhysicsPlugin, RigidBody};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(MovementPluginAnyState::any());

    #[cfg(feature = "physic")]
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_systems(Startup, setup);

    app.run();
}

#[cfg(feature = "physic")]
fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let cuboid = meshes.add(Cuboid::default());
    let debug_material = materials.add(StandardMaterial::default());

    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        Mesh3d(cuboid),
        MeshMaterial3d(debug_material.clone()),
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),
        GravityScale(0.),
        PhysicMovement {
            max_velocity: 5.,
            min_velocity: 1.,
            circle: true,
            des: vec![
                PhysicDestination::from_pos(Vec3::new(4., 4., 4.)),
                PhysicDestination::from_pos(Vec3::new(1., 1., 1.)),
                PhysicDestination::from_pos(Vec3::new(-3., 3., -2.)),
                PhysicDestination::from_pos(Vec3::new(2.3, -4., -1.)),
            ],
            ..default()
        },
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    ));
}
