#[cfg(feature = "physic_2d")]
use avian2d::{
    prelude::{
        Collider,
        RigidBody,
    },
    PhysicsPlugins,
};
#[cfg(feature = "physic_3d")]
use avian3d::{
    prelude::{
        Collider,
        RigidBody,
    },
    PhysicsPlugins,
};
use bevy::prelude::*;
#[cfg(feature = "kb_control")]
use bevy_movement::kb_control::KbMovementObject;
use bevy_movement::linear::LinearMovement;
use bevy_movement::MovementPluginAnyState;

fn main() {
    let mut app = App::new();

    #[cfg(any(feature = "physic_2d", feature = "physic_3d"))]
    app.add_plugins(PhysicsPlugins::default());

    app.add_plugins(DefaultPlugins)
        .add_plugins(MovementPluginAnyState::any())
        .add_systems(Startup, setup);

    app.run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let default_mat = materials.add(StandardMaterial::default());

    #[cfg(feature = "physic_3d")]
    {
        commands.spawn((
            Collider::cuboid(20., 1., 20.),
            RigidBody::Static,
            Mesh3d(meshes.add(Cuboid::new(20., 1., 20.))),
            Transform::from_xyz(0.0, 0., 0.0),
            MeshMaterial3d(default_mat.clone()),
        ));
        commands.spawn((
            Collider::cuboid(20., 4., 2.),
            RigidBody::Static,
            Transform::from_xyz(0.0, 0.5, -10.),
            MeshMaterial3d(default_mat.clone()),
            Mesh3d(meshes.add(Cuboid::new(20., 4., 2.))),
        ));

        commands.spawn((
            Collider::cuboid(20., 4., 2.),
            RigidBody::Static,
            Transform::from_xyz(0.0, 0.5, 10.),
            MeshMaterial3d(default_mat.clone()),
            Mesh3d(meshes.add(Cuboid::new(20., 4., 2.))),
        ));

        commands.spawn((
            Collider::cuboid(2., 4., 20.),
            RigidBody::Static,
            Transform::from_xyz(-10.0, 0.5, 0.),
            MeshMaterial3d(default_mat.clone()),
            Mesh3d(meshes.add(Cuboid::new(2., 4., 20.))),
        ));

        commands.spawn((
            Collider::cuboid(2., 4., 20.),
            RigidBody::Static,
            Transform::from_xyz(10.0, 0.5, 0.),
            MeshMaterial3d(default_mat.clone()),
            Mesh3d(meshes.add(Cuboid::new(2., 4., 20.))),
        ));
    }

    #[cfg(not(any(feature = "physic_2d", feature = "physic_3d")))]
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(20., 1., 20.))),
        Transform::from_xyz(0.0, 0., 0.0),
    ));

    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 3.0, 0.0)),
        KbMovementObject::new(),
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(default_mat),
        #[cfg(any(feature = "physic_2d", feature = "physic_3d"))]
        RigidBody::Dynamic,
        #[cfg(feature = "physic_3d")]
        Collider::cuboid(0.5,0.5,0.5),
        LinearMovement {
            #[cfg(not(any(feature = "physic_2d", feature = "physic_3d")))]
            speed: 0.01,
            #[cfg(any(feature = "physic_2d", feature = "physic_3d"))]
            speed: 5.,
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
