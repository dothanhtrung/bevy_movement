pub mod circle;

use crate::linear::circle::circle_travel;
use crate::Arrived;
#[cfg(all(feature = "physic_2d"))]
use avian2d::{
    math::Vector,
    prelude::{
        LinearVelocity,
        PhysicsSchedulePlugin,
    },
};
#[cfg(all(feature = "physic_3d"))]
use avian3d::{
    math::Vector,
    prelude::{
        LinearVelocity,
        PhysicsSchedulePlugin,
    },
};
use bevy::app::App;
use bevy::prelude::{in_state, info, Commands, Component, Entity, IntoScheduleConfigs, Plugin, Query, Res, States, Time, Transform, Update, Vec3, Vec3Swizzles};
#[cfg(feature = "path_finding")]
use bevy::prelude::{
    Deref,
    DerefMut,
    Resource,
};
#[cfg(feature = "path_finding")]
use bevy_northstar::prelude::{
    AgentPos,
    NextPos,
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
        #[cfg(any(feature = "physic_2d", feature = "physic_3d"))]
        if !app.is_plugin_added::<PhysicsSchedulePlugin>() {
            panic!("LinearMovementPlugin with 'physic' feature requires avian PhysicsPlugins. Add it first!");
        }

        #[cfg(not(feature = "path_finding"))]
        let systems = (circle_travel, check_arrived, straight_travel);
        #[cfg(feature = "path_finding")]
        let systems = (circle_travel, check_arrived, straight_travel, update_travel_stop);

        #[cfg(feature = "path_finding")]
        app.insert_resource(TileSize(Vec3::new(1., 1., 1.)));

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

    pub is_stopped: bool,

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
            is_stopped: false,
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

    /// Stop current movement
    pub fn stop(&mut self) {
        self.des = Vec::new();
        self.is_stopped = true;
    }
}

#[cfg(feature = "path_finding")]
#[derive(Resource, Deref, DerefMut)]
pub struct TileSize(pub Vec3);

#[cfg(not(any(feature = "physic_2d", feature = "physic_3d")))]
fn straight_travel(time: Res<Time>, mut query: Query<(&mut Transform, &LinearMovement)>) {
    for (mut transform, movement) in query.iter_mut() {
        if movement.des.is_empty() || movement.is_freezed {
            continue;
        }

        let des = movement.des.first().unwrap();
        let velocity = if let Some(custom_v) = des.custom_velocity { custom_v } else { movement.speed };

        let v = velocity * time.delta_secs();
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

#[cfg(any(feature = "physic_2d", feature = "physic_3d"))]
fn straight_travel(mut query: Query<(&mut Transform, &mut LinearMovement, &mut LinearVelocity)>, time: Res<Time>) {
    for (mut transform, mut movement, mut velocity) in query.iter_mut() {
        if movement.is_stopped {
            **velocity = Vector::ZERO;
            movement.is_stopped = false;
            continue;
        }
        if movement.des.is_empty() || movement.is_freezed {
            continue;
        }

        let des = movement.des.first().unwrap();
        let flat_vel = if let Some(custom_v) = des.custom_velocity { custom_v } else { movement.speed };
        let next_stop = movement.des.first().unwrap().pos + movement.offset;
        #[cfg(feature = "physic_3d")]
        let direction = next_stop - transform.translation;
        #[cfg(feature = "physic_2d")]
        let direction = next_stop.xy() - transform.translation.xy();

        let len = direction.length();
        if len <= flat_vel * time.delta_secs() {
            **velocity = Vector::ZERO;
            // FIXME: If the destination is closer than the distance object can travel in 1 tick,
            //        the object will go through the collider when enable physic
            transform.translation = next_stop;
        } else {
            **velocity = direction / len * flat_vel;
        }
    }
}

fn check_arrived(
    mut commands: Commands,
    #[cfg(not(feature = "path_finding"))] mut query: Query<(&Transform, &mut LinearMovement, Entity, Entity, Entity)>,
    #[cfg(feature = "path_finding")] mut query: Query<(
        &Transform,
        &mut LinearMovement,
        Entity,
        &mut AgentPos,
        &NextPos,
    )>,
) {
    for (transform, mut movement, e, mut _agent_pos, _next_pos) in query.iter_mut() {
        if movement.des.is_empty() || movement.is_freezed {
            continue;
        }
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

            #[cfg(feature = "path_finding")]
            {
                commands.entity(e).remove::<NextPos>();
                _agent_pos.0 = _next_pos.0;
            }
        }
    }
}

#[cfg(feature = "path_finding")]
fn update_travel_stop(mut query: Query<(&NextPos, &mut LinearMovement)>, tile_size: Res<TileSize>) {
    for (next_pos, mut movement) in query.iter_mut() {
        let next_pos_f = next_pos.0.as_vec3() * tile_size.0;
        if let Some(des) = movement.des.first() {
            if next_pos_f != des.pos {
                info!("Set real next pos:  {:?}", next_pos_f);
                movement.des = vec![LinearDestination::from_pos(next_pos_f)];
            }
        }
    }
}
