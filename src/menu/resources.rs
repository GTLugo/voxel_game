use bevy::prelude::*;

#[derive(Resource)]
pub struct MenuData {
  pub camera_entity: Entity,
  pub button_entity: Entity,
}