pub mod circle;

use crate::linear::circle::circle_travel;
use crate::Arrived;
use bevy::app::App;
use bevy::prelude::{
    in_state, Commands, Component, Entity, IntoScheduleConfigs, Plugin, Query, Res, States, Time, Transform, Update,
    Vec3, Vec3Swizzles,
};

macro_rules! linear_movement_systems {
    () => {
        (straight_travel, circle_travel)
    };
}

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
        if self.states.is_empty() {
            app.add_systems(Update, linear_movement_systems!());
        } else {
            for state in &self.states {
                app.add_systems(Update, linear_movement_systems!().run_if(in_state(state.clone())));
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
}

impl Default for LinearMovement {
    fn default() -> Self {
        Self {
            speed: 0.,
            des: Vec::new(),
            is_repeated: false,
            is_freezed: false,
            epsilon: 1e-4,
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

fn straight_travel(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut LinearMovement, Entity)>,
) {
    for (mut transform, mut movement, e) in query.iter_mut() {
        if movement.des.is_empty() || movement.is_freezed {
            continue;
        }

        let mut arrived = false;
        let des = movement.des.first().unwrap();
        let velocity = if let Some(custom_v) = des.custom_velocity { custom_v } else { movement.speed };

        let v = velocity * (time.delta().as_millis() as f32);
        let next_stop = movement.des.first().unwrap().pos;

        if cfg!(feature = "3d") {
            let xyz = transform.translation.move_towards(next_stop, v);
            transform.translation = xyz;

            if transform.translation.distance(next_stop) <= movement.epsilon {
                arrived = true;
            }
        } else {
            let xy = transform.translation.xy().move_towards(next_stop.xy(), v);
            transform.translation.x = xy.x;
            transform.translation.y = xy.y;

            if transform.translation.xy().distance(next_stop.xy()) <= movement.epsilon {
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
