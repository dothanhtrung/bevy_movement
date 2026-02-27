#[cfg(feature = "physic")]
use avian3d::prelude::{Collider, RigidBody};
use bevy::prelude::*;
#[cfg(not(feature = "physic"))]
use bevy_movement::linear::LinearMovement;
use bevy_movement::mouse_control::{ClickCatcher, MovementObject};
use bevy_movement::MovementPluginAnyState;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(MovementPluginAnyState::any())
        .add_systems(Startup, setup);

    app.run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let default_mat = materials.add(StandardMaterial::default());

    #[cfg(feature = "physic")]
    {
        commands.spawn((
            Collider::cuboid(10., 0.5, 10.),
            RigidBody::Kinematic,
            ClickCatcher,
            Mesh3d(meshes.add(Cuboid::new(20., 1., 20.))),
            Transform::from_xyz(0.0, 0., 0.0),
            MeshMaterial3d(default_mat.clone()),
        ));
        commands.spawn((
            Collider::cuboid(10., 3., 1.),
            RigidBody::Kinematic,
            Transform::from_xyz(0.0, 0.5, -10.),
            MeshMaterial3d(default_mat.clone()),
            Mesh3d(meshes.add(Cuboid::new(20., 4., 2.))),
        ));

        commands.spawn((
            Collider::cuboid(10., 3., 1.),
            RigidBody::Kinematic,
            Transform::from_xyz(0.0, 0.5, 10.),
            MeshMaterial3d(default_mat.clone()),
            Mesh3d(meshes.add(Cuboid::new(20., 4., 2.))),
        ));

        commands.spawn((
            Collider::cuboid(1., 3., 10.),
            RigidBody::Kinematic,
            Transform::from_xyz(-10.0, 0.5, 0.),
            MeshMaterial3d(default_mat.clone()),
            Mesh3d(meshes.add(Cuboid::new(2., 4., 20.))),
        ));

        commands.spawn((
            Collider::cuboid(1., 3., 10.),
            RigidBody::Kinematic,
            Transform::from_xyz(10.0, 0.5, 0.),
            MeshMaterial3d(default_mat.clone()),
            Mesh3d(meshes.add(Cuboid::new(2., 4., 20.))),
        ));
    }

    #[cfg(not(feature = "physic"))]
    commands.spawn((
        ClickCatcher,
        Mesh3d(meshes.add(Cuboid::new(20., 1., 20.))),
        Transform::from_xyz(0.0, 0., 0.0),
    ));

    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 2.0, 0.0)),
        MovementObject::default(),
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(default_mat),
        #[cfg(feature = "physic")]
        RigidBody::Dynamic,
        #[cfg(feature = "physic")]
        Collider::sphere(0.5),
        // #[cfg(feature = "physic")]
        // PhysicMovement {
        //     max_velocity: 15.,
        //     min_velocity: 10.,
        //     epsilon: 2.5,
        //     des: vec![],
        //     ..default()
        // },
        #[cfg(not(feature = "physic"))]
        LinearMovement {
            speed: 0.01,
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
