use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
  #[default]
  Launching,
  Menu,
  InGame,
  Crash,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum SimulationState {
  #[default]
  Paused,
  Playing,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AssetState {
  #[default]
  Unloaded,
  Loading,
  Building,
  Loaded,
  Error,
}