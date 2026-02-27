pub mod circle;

use crate::linear::circle::circle_travel;
use crate::Arrived;
use avian3d::prelude::PhysicsSchedulePlugin;
#[cfg(all(feature = "physic"))]
use avian3d::{math::Vector, prelude::LinearVelocity};
use bevy::app::App;
use bevy::prelude::{
    in_state, Commands, Component, Entity, IntoScheduleConfigs, Plugin, Query, Res, States, Time, Transform, Update,
    Vec3, Vec3Swizzles,
};

pub(crate) struct LinearMovementPlugin<T>
where
    T: States,
{
    pub states: Vec<T>,
}

impl<T> LinearMovementPlugin<T>
where
    T: States,
{
    pub(crate) fn new(states: Vec<T>) -> Self {
        Self { states }
    }
}

impl<T> Plugin for LinearMovementPlugin<T>
where
    T: States,
{
    fn build(&self, app: &mut App) {
        #[cfg(feature = "physic")]
        if !app.is_plugin_added::<PhysicsSchedulePlugin>() {
            panic!("LinearMovementPlugin with 'physic' feature requires avian PhysicsPlugins. Add it first!");
        }

        let systems = (circle_travel, check_arrived, straight_travel);
        if self.states.is_empty() {
            app.add_systems(Update, systems);
        } else {
            for state in &self.states {
                app.add_systems(Update, systems.run_if(in_state(state.clone())));
            }
        }
    }
}

#[derive(Default, Clone)]
pub struct LinearDestination {
    pub pos: Vec3,
    pub custom_velocity: Option<f32>,
}

impl LinearDestination {
    pub fn from_pos(pos: Vec3) -> Self {
        Self { pos, ..Self::default() }
    }
}

#[derive(Component)]
pub struct LinearMovement {
    pub speed: f32,

    pub des: Vec<LinearDestination>,

    /// Repeat destination
    pub is_repeated: bool,

    pub is_freezed: bool,

    /// Minimal distance to consider object is arrived
    pub epsilon: f32,

    pub offset: Vec3,
}

impl Default for LinearMovement {
    fn default() -> Self {
        Self {
            speed: 0.,
            des: Vec::new(),
            is_repeated: false,
            is_freezed: false,
            epsilon: 1e-4,
            offset: Vec3::ZERO,
        }
    }
}

impl LinearMovement {
    pub fn freeze(&mut self) {
        self.is_freezed = true;
    }

    pub fn go(&mut self) {
        self.is_freezed = false;
    }
}

#[cfg(not(feature = "physic"))]
fn straight_travel(time: Res<Time>, mut query: Query<(&mut Transform, &LinearMovement)>) {
    for (mut transform, movement) in query.iter_mut() {
        if movement.des.is_empty() || movement.is_freezed {
            continue;
        }

        let des = movement.des.first().unwrap();
        let velocity = if let Some(custom_v) = des.custom_velocity { custom_v } else { movement.speed };

        let v = velocity * (time.delta().as_millis() as f32);
        let next_stop = movement.des.first().unwrap().pos + movement.offset;

        if cfg!(feature = "2d") {
            let xy = transform.translation.xy().move_towards(next_stop.xy(), v);
            transform.translation.x = xy.x;
            transform.translation.y = xy.y;
        } else {
            let xyz = transform.translation.move_towards(next_stop, v);
            transform.translation = xyz;
        }
    }
}

#[cfg(feature = "physic")]
fn straight_travel(mut query: Query<(&mut Transform, &LinearMovement, &mut LinearVelocity)>, time: Res<Time>) {
    for (mut transform, movement, mut velocity) in query.iter_mut() {
        if movement.des.is_empty() || movement.is_freezed {
            continue;
        }

        let des = movement.des.first().unwrap();
        let flat_vel = if let Some(custom_v) = des.custom_velocity { custom_v } else { movement.speed };
        let next_stop = movement.des.first().unwrap().pos + movement.offset;
        let direction = next_stop - transform.translation;

        let len = direction.length();
        if len <= flat_vel * time.delta_secs() {
            **velocity = Vector::ZERO;
            transform.translation = next_stop;
        } else {
            **velocity = direction / len * flat_vel;
        }
    }
}

fn check_arrived(mut commands: Commands, mut query: Query<(&Transform, &mut LinearMovement, Entity)>) {
    for (transform, mut movement, e) in query.iter_mut() {
        let next_stop = movement.des.first().unwrap().pos + movement.offset;
        let mut arrived = false;
        if cfg!(feature = "2d") {
            if transform.translation.xy().distance(next_stop.xy()) <= movement.epsilon {
                arrived = true;
            }
        } else {
            if transform.translation.distance(next_stop) <= movement.epsilon {
                arrived = true;
            }
        }

        if arrived {
            commands.trigger(Arrived { entity: e });
            if movement.is_repeated {
                let first_des = movement.as_ref().des.first().unwrap().clone();
                movement.des.push(first_des);
            }
            movement.des.remove(0);
        }
    }
}
