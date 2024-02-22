use bevy::prelude::*;

use self::controls::*;

mod controls;

#[derive(Resource, Default)]
pub struct PlayerControls {
  pub mouse_config: MouseConfig,
  pub bindings: Bindings,
}