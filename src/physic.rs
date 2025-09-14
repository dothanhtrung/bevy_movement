use crate::Arrived;
use bevy::prelude::ops::{cos, sin};
use bevy::prelude::{
    in_state, App, Commands, Component, Entity, IntoScheduleConfigs, Plugin, Query, Res, States, Time, Transform,
    Update, Vec3,
};
use bevy::utils::default;
use bevy_rapier3d::prelude::{
    AdditionalMassProperties, Collider, ColliderMassProperties, ExternalForce, NoUserData, RapierPhysicsPlugin,
    Velocity,
};

macro_rules! physic_movement_systems {
    () => {
        (travel)
    };
}

pub(crate) struct PhysicMovementPlugin<T>
where
    T: States,
{
    pub states: Vec<T>,
}

impl<T> PhysicMovementPlugin<T>
where
    T: States,
{
    pub(crate) fn new(states: Vec<T>) -> Self {
        Self { states }
    }
}

impl<T> Plugin for PhysicMovementPlugin<T>
where
    T: States,
{
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());

        if self.states.is_empty() {
            app.add_systems(Update, physic_movement_systems!());
        } else {
            for state in &self.states {
                app.add_systems(Update, physic_movement_systems!().run_if(in_state(state.clone())));
            }
        }
    }
}

#[derive(Default, Clone)]
pub struct PhysicDestination {
    pub pos: Vec3,
}

impl PhysicDestination {
    pub fn from_pos(pos: Vec3) -> Self {
        Self { pos }
    }
}

#[derive(Component)]
#[require(Velocity)]
pub struct PhysicMovement {
    pub des: Vec<PhysicDestination>,

    /// Repeat destination. Going around in a circle.
    pub circle: bool,

    /// Minimal distance to consider object is arrived
    pub epsilon: f32,

    pub max_velocity: f32,
    pub min_velocity: f32,

    /// Time to accelerate to max velocity, f32 sec
    pub acceleration_time: f32,

    pub break_time: f32,
}

impl Default for PhysicMovement {
    fn default() -> Self {
        Self {
            des: Vec::new(),
            circle: false,
            epsilon: 1e-4,
            max_velocity: 0.,
            min_velocity: 0.,
            acceleration_time: 0.3,
            break_time: 0.03,
        }
    }
}

fn travel(
    mut commands: Commands,
    mut query: Query<(
        &mut PhysicMovement,
        &Transform,
        &Velocity,
        &Collider,
        Option<&mut ExternalForce>,
        Option<&ColliderMassProperties>,
        Option<&AdditionalMassProperties>,
        Entity,
    )>,
    time: Res<Time>,
) {
    for (mut movement, transform, velocity, collider, external_force, collider_mass, additional_mass, e) in
        query.iter_mut()
    {
        if let Some(next_pos) = movement.des.first() {
            let goal_vect = next_pos.pos - transform.translation;
            let angle = if velocity.linvel == Vec3::ZERO { 0. } else { goal_vect.angle_between(velocity.linvel) };
            let real_vel = velocity.linvel.length();
            let current_velocity = real_vel * cos(angle);
            let distance = transform.translation.distance(next_pos.pos);
            if distance <= movement.epsilon || distance <= current_velocity * time.delta_secs() {
                commands.entity(e).remove::<ExternalForce>();
                commands.trigger(Arrived { entity: e });
                if movement.circle {
                    let first_des = movement.as_ref().des.first().unwrap().clone();
                    movement.des.push(first_des);
                }
                movement.des.remove(0);
                return;
            }

            if current_velocity <= movement.min_velocity {
                let mut mass = 0.;

                mass += match collider_mass {
                    None => collider.raw.mass_properties(1.).mass(),
                    Some(ColliderMassProperties::Density(density)) => collider.raw.mass_properties(*density).mass(),
                    Some(ColliderMassProperties::Mass(m)) => *m,
                    Some(ColliderMassProperties::MassProperties(p)) => p.mass,
                };

                mass += match additional_mass {
                    None => 0.,
                    Some(AdditionalMassProperties::Mass(m)) => *m,
                    Some(AdditionalMassProperties::MassProperties(p)) => p.mass,
                };

                let break_force =
                    Vec3::ZERO.move_towards(velocity.linvel, -real_vel * sin(angle) / movement.break_time * mass);

                let d = (movement.max_velocity - current_velocity) / movement.acceleration_time * mass;
                let forward_force = move_over(transform.translation, next_pos.pos, d) - transform.translation;

                let new_force = break_force + forward_force;

                if let Some(mut external_force) = external_force {
                    external_force.force = new_force;
                } else {
                    commands.entity(e).insert(ExternalForce {
                        force: new_force,
                        ..default()
                    });
                }
            } else if current_velocity >= movement.max_velocity {
                commands.entity(e).remove::<ExternalForce>();
            }
        }
    }
}

fn move_over(a: Vec3, b: Vec3, d: f32) -> Vec3 {
    let x = b - a;
    let len = x.length();
    a + x / len * d
}
