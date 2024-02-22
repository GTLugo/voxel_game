use std::f32::consts::FRAC_PI_2;

use bevy::{prelude::*, input::mouse::MouseMotion, window::{CursorGrabMode, PrimaryWindow}, sprite::MaterialMesh2dBundle, pbr::{ScreenSpaceAmbientOcclusionSettings, ScreenSpaceAmbientOcclusionBundle}, core_pipeline::experimental::taa::TemporalAntiAliasBundle};
use bevy_atmosphere::prelude::*;

use super::*;

type AmbientOcclusionLevel = bevy::pbr::ScreenSpaceAmbientOcclusionQualityLevel;

fn toggle_cursor_lock(window: &mut Window, movement_config: &mut PlayerMovementConfig) {
  match window.cursor.grab_mode {
    CursorGrabMode::None => {
      window.cursor.grab_mode = CursorGrabMode::Confined;
      window.cursor.visible = false;
      movement_config.control_enabled = true;
    }
    _ => {
      window.cursor.grab_mode = CursorGrabMode::None;
      window.cursor.visible = true;
      movement_config.control_enabled = false;
    }
  }
}

pub fn spawn_player_system(
  mut next_state: ResMut<NextState<WorldState>>,
  mut commands: Commands, 
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  info!("Spawning player...");

  // commands.insert_resource(AtmosphereModel::new(
  //   Gradient {
  //     sky: Color::rgb(0.49, 0.61, 0.70),
  //     horizon: Color::rgb(0.48, 0.62, 0.69),
  //     ground: Color::rgb(0.71, 0.69, 0.57),
  //   }
  // ));

  commands.insert_resource(AtmosphereModel::new(
    Gradient {
      sky: Color::rgb(0.4, 0.6, 0.8),
      horizon: Color::rgb(0.6, 0.87, 1.0),
      ground: Color::DARK_GRAY,
    }
  ));

  // commands.insert_resource(AtmosphereModel::new(
  //   Nishita {
  //     sun_intensity: 16.0,
  //     ..Default::default()
  //   }
  // ));

  commands.spawn((
    PlayerCamera,
    Camera3dBundle {
      camera: Camera {
        hdr: true,
        ..default()
      },
      projection: Default::default(),
      visible_entities: Default::default(),
      frustum: Default::default(),
      transform: Transform::default(),
      global_transform: Default::default(),
      camera_3d: Default::default(),
      tonemapping: Default::default(),
      dither: Default::default(),
      color_grading: Default::default(),
      ..Default::default()
    },
    AtmosphereCamera::default(), // i can't figure out why this won't work (no, it's not the commented out resource)
    FogSettings {
      color: Color::rgba(0.5, 0.8, 1.0, 1.0),
      // color: Color::rgba(0.1, 0.2, 0.4, 1.0),
      directional_light_color: Color::rgba(1.0, 0.95, 0.75, 0.5),
      directional_light_exponent: 30.0,
      falloff: FogFalloff::from_visibility_colors(
        100.0, // distance in world units up to which objects retain visibility (>= 5% contrast)
        Color::rgb(0.35, 0.5, 0.66), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
        Color::rgb(0.8, 0.844, 1.0), // atmospheric inscattering color (light gained due to scattering from the sun)
      ),
    },
  )).insert(ScreenSpaceAmbientOcclusionBundle {
    settings: ScreenSpaceAmbientOcclusionSettings {
      quality_level: AmbientOcclusionLevel::Ultra,
    },
    ..Default::default()
  }).insert(TemporalAntiAliasBundle::default());

  commands.spawn((
    Player,
    MainPlayer,
    PlayerMovementConfig {
      ..Default::default()
    },
    PlayerCameraState {
      ..Default::default()
    },
    PlayerMovementState {
      ..Default::default()
    },
    SpatialBundle {
      transform: Transform::from_xyz(1.0, 1.0, 3.0),
      ..Default::default()
    },
  )).with_children(|builder| {
    builder.spawn((
      Crosshair,
      MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::from_xyz(1.0, 1.0, -5.0).with_scale(Vec3::splat(128.)),
        material: materials.add(ColorMaterial::from(Color::AQUAMARINE)),
        ..default()
      }
    ));
  });

  next_state.set(WorldState::InWorld);
}

#[allow(clippy::type_complexity)]
pub fn player_movment_system(
  time: Res<Time>,
  keyboard_input: Res<Input<KeyCode>>,
  mouse_button_input: Res<Input<MouseButton>>,
  controls: Res<PlayerControls>,
  mut movement_query: Query<(&PlayerMovementConfig, &mut PlayerMovementState), With<MainPlayer>>,
  mut transform_set: ParamSet<(
    Query<&mut Transform, With<MainPlayer>>,
    Query<&mut Transform, With<PlayerCamera>>,
  )>,
) {
  for (movement_settings, mut movement) in movement_query.iter_mut() {
    let player_translation = if let Ok(mut player_transform) = transform_set.p0().get_single_mut() {
      // x: forward, y: up, z: right
      let input = if movement_settings.control_enabled {
        controls.bindings.movement_axes(&keyboard_input, &mouse_button_input)
      } else {
        Vec3::ZERO
      };

      movement.forward_walking_vector = player_transform.forward();
      movement.strafe_vector = player_transform.right();

      let acceleration = (movement.strafe_vector * input.z) + (movement.forward_walking_vector * input.x) + (Vec3::Y * input.y);
      let acceleration = if acceleration.length() != 0.0 {
        acceleration.normalize() * movement_settings.move_acceleration
      } else {
        Vec3::ZERO
      };

      let friction = if movement.velocity.length() != 0.0 {
        movement.velocity.normalize() * movement_settings.friction * -1.0
      } else {
        Vec3::ZERO
      };

      if input.length() > 0.0 {
        let delta_v = acceleration * time.delta_seconds();

        movement.velocity += delta_v;

        // clamp length (clamp function does a lot of unnecesary squaring and squart-rooting)
        if movement.velocity.length() > movement_settings.max_move_speed {
          movement.velocity = movement.velocity.normalize() * movement_settings.max_move_speed;
        }
      } else {
        let delta_v = friction * time.delta_seconds();

        if (movement.velocity + delta_v).signum() != movement.velocity.signum() {
          movement.velocity = Vec3::ZERO;
        } else {
          movement.velocity += delta_v;
        };
      }

      player_transform.translation += movement.velocity * time.delta_seconds();
      
      player_transform.translation
    } else {
      warn!("Failed to find Main Player body tranform for movement");
      return;
    };

    if let Ok(mut camera_transform) = transform_set.p1().get_single_mut() {
      camera_transform.translation = player_translation;
    } else {
      warn!("Failed to find Main Player camera tranform for movement");
      return;
    }
  }
}

#[allow(clippy::type_complexity)]
// uhhhh ok so it seems like i DON'T want to adjust the aim based on framerate... brain hort
pub fn player_aim_system(
  mut mouse_motion: EventReader<MouseMotion>,
  controls: Res<PlayerControls>,
  primary_window: Query<&Window, With<PrimaryWindow>>,
  mut movement_query: Query<(&PlayerMovementConfig, &mut PlayerCameraState), With<MainPlayer>>,
  mut transform_set: ParamSet<(
    Query<&mut Transform, With<MainPlayer>>,
    Query<&mut Transform, With<PlayerCamera>>,
  )>,
) {
  for (movement_settings, mut camera) in movement_query.iter_mut() {
    if !movement_settings.control_enabled {
      return;
    }

    // if mouse_motion.is_empty() {
    //   return;
    // }

    for event in mouse_motion.iter() {
      if event.delta.is_nan() {
        return;
      }

      // normalize from window ratio to 1:1, with vertical being held constant
      let mouse_delta = if let Ok(window) = primary_window.get_single() {
        event.delta * Vec2::new(window.height() / window.width(), 1.0)
      } else {
        warn!("Primary window not found when attempting to aim camera");
        event.delta
      };

      // adjust with factor so it doesn't yeet the cursor into the shadow realm
      const SENS_FACTOR: f32 = 0.0025f32;
      let sens = controls.mouse_config.mouse_sensitivity * SENS_FACTOR;

      // apply
      camera.yaw -= sens * mouse_delta.x;
      camera.pitch += sens * mouse_delta.y;

      // offset for max pitch either way to avoid weird visual artifacting. likely a rendering bug.
      const MAX_PITCH_OFFSET: f32 = 0.0001; 
      camera.pitch = camera.pitch.clamp(-FRAC_PI_2 - MAX_PITCH_OFFSET, FRAC_PI_2 - MAX_PITCH_OFFSET);

      for mut player_transform in transform_set.p0().iter_mut() {
        player_transform.rotation = Quat::from_axis_angle(Vec3::Y, camera.yaw) * Quat::from_axis_angle(-Vec3::X, 0.0);
      }
  
      for mut camera_transform in transform_set.p1().iter_mut() {
        camera_transform.rotation = Quat::from_axis_angle(Vec3::Y, camera.yaw) * Quat::from_axis_angle(-Vec3::X, camera.pitch);
      }
    }
  }
}

pub fn lock_cursor_system(
  controls: Res<PlayerControls>,
  keyboard_input: Res<Input<KeyCode>>,
  mouse_button_input: Res<Input<MouseButton>>,
  mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
  mut query: Query<&mut PlayerMovementConfig, With<MainPlayer>>,
) {
  for mut movement_config in query.iter_mut() {
    if let Ok(mut window) = primary_window.get_single_mut() {
      if controls.bindings.cursor_lock.was_just_activated(&keyboard_input, &mouse_button_input) {
        toggle_cursor_lock(&mut window, &mut movement_config);
      }
    } else {
      warn!("Primary window not found when attempting to lock cursor");
    }
  }
}

pub fn lock_cursor_on_start_system(
  mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
  mut query: Query<&mut PlayerMovementConfig, With<MainPlayer>>,
) {
  if let Ok(mut window) = primary_window.get_single_mut() {
    debug!("Locking cursor...");
    debug!("{:?}", query);
    for mut movement_config in query.iter_mut() {
      window.cursor.grab_mode = CursorGrabMode::Confined;
      window.cursor.visible = false;
      movement_config.control_enabled = true;
      debug!("Locked cursor");
    }
  } else {
    warn!("Primary window not found when attempting to lock cursor");
  }
}

// fn forward_vector(rotation: &Quat) -> Vec3 {
//   rotation.mul_vec3(Vec3::Z).normalize()
// }

// fn forward_walk_vector(rotation: &Quat) -> Vec3 {
//   let vector = Self::forward_vector(rotation);
//   Vec3::new(vector.x, 0., vector.z).normalize()
// }

// fn strafe_vector(rotation: &Quat) -> Vec3 {
//   Quat::from_rotation_y(FRAC_PI_2)
//     .mul_vec3(Self::forward_walk_vector(rotation))
//     .normalize()
// }


// fn initial_cursor_lock_on_camera_spawn(
//   mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
//   query: Query<Entity, Added<FlyCam>>,
// ) {
//   if query.is_empty() {
//     return;
//   }

//   if let Ok(mut window) = primary_window.get_single_mut() {
//     Self::toggle_cursor_lock(&mut window);
//   } else {
//     warn!("Primary window not found when attempting to lock cursor");
//   }
// }