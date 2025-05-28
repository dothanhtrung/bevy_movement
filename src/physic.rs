use crate::Arrived;
use bevy::prelude::{in_state, App, Commands, Component, Entity, IntoScheduleConfigs, Plugin, Query, States, Transform, Update, Vec3};
use bevy::utils::default;
use bevy_rapier3d::prelude::{AdditionalMassProperties, Collider, ColliderMassProperties, ExternalForce, ExternalImpulse, Velocity};

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

#[derive(Component, Default)]
#[require(Velocity)]
pub struct PhysicMovement {
    pub des: Vec<PhysicDestination>,

    /// Repeat destination. Going around in a circle.
    pub circle: bool,

    /// Minimal distance to consider object is arrived
    pub epsilon: f32,

    pub max_velocity: f32,
    pub min_velocity: f32,
}

fn travel(
    mut commands: Commands,
    mut query: Query<(
        &mut PhysicMovement,
        &Transform,
        &Velocity,
        &Collider,
        Option<&ColliderMassProperties>,
        Option<&AdditionalMassProperties>,
        Entity,
    )>,
) {
    for (mut movement, transform, velocity, collider, collider_mass, additional_mass, e) in query.iter_mut() {
        if let Some(next_pos) = movement.des.first() {
            if transform.translation.distance(next_pos.pos) <= movement.epsilon {
                commands.entity(e).remove::<ExternalImpulse>();
                commands.trigger_targets(Arrived, e);
                if movement.circle {
                    let first_des = movement.as_ref().des.first().unwrap().clone();
                    movement.des.push(first_des);
                }
                movement.des.remove(0);
                return;
            }

            let current_velocity = velocity.linvel.distance(Vec3::ZERO);
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

                let i = movement.max_velocity * mass;
                let force = transform.translation.move_towards(next_pos.pos, i) - transform.translation;
                commands.entity(e).insert(ExternalForce { force, ..default() });
            } else if current_velocity >= movement.max_velocity {
                commands.entity(e).remove::<ExternalForce>();
            }
        }
    }
}
