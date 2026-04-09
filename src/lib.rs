#[cfg(feature = "kb_control")]
pub mod kb_control;
pub mod linear;
#[cfg(feature = "mouse_control")]
pub mod mouse_control;

use crate::linear::LinearMovementPlugin;
use bevy::prelude::{
    App,
    Entity,
    EntityEvent,
    Plugin,
    States,
    Vec3,
};

/// The main plugin
#[derive(Default)]
pub struct MovementPlugin<T = DummyState>
where
    T: States,
{
    /// List of game state that this plugin will run in.
    pub states: Vec<T>,
}

impl<T> Plugin for MovementPlugin<T>
where
    T: States,
{
    fn build(&self, app: &mut App) {
        app.add_plugins(LinearMovementPlugin::new(self.states.clone()));

        #[cfg(feature = "mouse_control")]
        app.add_plugins(mouse_control::MouseControlMovementPlugin::new(self.states.clone()));

        #[cfg(feature = "kb_control")]
        app.add_plugins(kb_control::KbControlMovementPlugin::new(self.states.clone()));
    }
}

impl<T> MovementPlugin<T>
where
    T: States,
{
    pub fn new(states: Vec<T>) -> Self {
        Self { states }
    }

    pub fn any() -> Self {
        Self { states: Vec::new() }
    }
}

#[derive(States, Clone, Debug, Hash, Eq, PartialEq)]
pub enum DummyState {}

/// Use this if you don't care to state and want this plugin's systems always run.
pub struct MovementPluginAnyState;

impl MovementPluginAnyState {
    pub fn any() -> MovementPlugin<DummyState> {
        MovementPlugin::new(Vec::new())
    }
}

#[derive(EntityEvent)]
pub struct Arrived {
    pub entity: Entity,
    pub pos: Vec3,
}

#[derive(EntityEvent)]
pub struct NextDes {
    pub entity: Entity,
    pub des: Destination,
    pub is_chain: bool,
}

#[derive(Default, Clone)]
pub struct Destination {
    pub pos: Vec3,
    pub custom_velocity: Option<f32>,
}

impl Destination {
    pub fn from_pos(pos: Vec3) -> Self {
        Self { pos, ..Self::default() }
    }
}
