use bevy::prelude::*;

use crate::game::states::GameState;

use self::components::*;
use self::resources::*;
use self::systems::*;

pub mod components;
pub mod resources;
pub mod systems;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
  fn build(&self, app: &mut App) {
    app
      // systems
      .add_systems(OnEnter(GameState::Menu), setup_menu_system)
      .add_systems(Update, 
        menu_system
          .run_if(in_state(GameState::Menu))
      )
      .add_systems(OnExit(GameState::Menu), cleanup_menu_system)
      ;
  }
}