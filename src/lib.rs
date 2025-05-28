// Copyright 2025 Trung Do <dothanhtrung@pm.me>

pub mod linear;
pub mod physic;

use crate::linear::LinearMovementPlugin;
use bevy::prelude::{App, Event, IntoScheduleConfigs, Plugin, States};

/// The main plugin
#[derive(Default)]
pub struct MovementPlugin<T>
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

#[derive(Event)]
pub struct Arrived;
