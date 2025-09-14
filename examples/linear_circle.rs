use bevy::prelude::*;
use bevy_movement::linear::circle::LinearCircleMovement;
use bevy_movement::MovementPluginAnyState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MovementPluginAnyState::any())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let default_mat = materials.add(StandardMaterial::default());

    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(default_mat.clone()),
        LinearCircleMovement {
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
