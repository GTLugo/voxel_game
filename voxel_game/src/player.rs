// Original camera code incrementally adapted from 
// bevy_flycam: https://github.com/sburris0/bevy_flycam/
// and 
// bevy_fly_camera: https://github.com/mcpar-land/bevy_fly_camera
//
// bevy_flycam ISC License:
//
// Copyright 2020 Spencer Burris
//
// Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee is hereby granted, provided 
// that the above copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE 
// INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR 
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF 
// USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF 
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
// MIT License
//
// Copyright (c) mcpar-land
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
use bevy::prelude::*;

use crate::game::states::GameState;
use crate::world::states::WorldState;

use self::components::*;
use self::resources::*;
use self::systems::*;

pub mod components;
pub mod resources;
pub mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      // resources
      .init_resource::<PlayerControls>()
      // systems
      .add_systems(OnEnter(WorldState::Spawning), spawn_player_system)
      .add_systems(OnEnter(WorldState::InWorld), lock_cursor_on_start_system)
      .add_systems(Update, (
          lock_cursor_system,
          player_aim_system,
          player_movment_system,
        )
        .run_if(in_state(GameState::InGame))
      )
      ;
  }
}

// Vec3::new(10.0e-6, 15.0e-6, 20.0e-6) -> foggy grey skies
// Vec3::new(0.5e-6, 4.0e-6, 15.0e-6) -> deep blue skies

// TODO: separate out all the different player info into separate resources and components