use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum WorldState {
  #[default]
  Unloaded,
  Loading,
  Spawning,
  InWorld,
  Despawning,
}