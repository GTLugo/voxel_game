use std::f32::consts::{FRAC_PI_4, FRAC_PI_6};

use bevy::app::AppExit;
use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap};
use bevy::prelude::*;
use tracing::info;

use super::states::*;
use super::components::*;


pub fn failed_to_load_assets_system(mut quit: EventWriter<AppExit>) {
  info!("Failed to load assets!");
  info!("Quitting the application...");
  quit.send(AppExit);
}

pub fn start_loading_assets_system(
  next_game_state: ResMut<NextState<GameState>>,
  asset_state: Res<State<AssetState>>,
  mut next_asset_state: ResMut<NextState<AssetState>>,
) {
  // WorldState should always stay defaulted to Unloaded during launch, so no need to handle it
  match asset_state.get() {
    AssetState::Unloaded => {
      info!("Loading assets...");
      next_asset_state.set(AssetState::Loading);
    },
    AssetState::Loaded => {
      finish_loading_assets_system(next_game_state);
    },
    _ => {}
  }
}

pub fn finish_loading_assets_system(
  mut next_game_state: ResMut<NextState<GameState>>,
) {
  // WorldState should always stay defaulted to Unloaded during launch, so no need to handle it
  info!("Assets loaded.");
  info!("Entering menus...");
  next_game_state.set(GameState::Menu);
}

pub fn spawn_lights_system(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  info!("hello hello!");

  // commands.insert_resource(ClearColor(Color::rgba(0.5, 0.8, 1.0, 1.0)));
  commands.insert_resource(DirectionalLightShadowMap { size: 4096 });

  let sunlight = Color::rgb(0.98, 0.95, 0.82);

  let cascade_shadow_config = CascadeShadowConfigBuilder {
    first_cascade_far_bound: 5.0,
    maximum_distance: 1000.0,
    num_cascades: 4,
    ..default()
  }
  .build();

  commands.spawn((
    Sun, // Marks the light as the Sun
    DirectionalLightBundle {
      directional_light: DirectionalLight {
        color: sunlight,
        shadows_enabled: true,
        illuminance: 100_000.0,
        ..Default::default()
      },
      transform: Transform {
        translation: Vec3::ZERO,
        rotation: Quat::from_rotation_y(FRAC_PI_6) * Quat::from_rotation_x(-FRAC_PI_4),
        ..Default::default()
      },
      cascade_shadow_config,
      ..Default::default()
    },
  ));

  commands.insert_resource(AmbientLight {
    color: sunlight,
    brightness: 0.5,
  });

  commands.spawn(
    PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Cube {
        size: 1.0,
      })),
      material: materials.add(StandardMaterial {
        base_color: Color::GRAY,
        perceptual_roughness: 1.0,
        reflectance: 0.0,
        ..Default::default()
      }),
      transform: Transform::from_xyz(0., 0., 0.),
      ..Default::default()
    }
  );  

  // X
  commands.spawn(
    PbrBundle {
      mesh: meshes.add(Mesh::from(shape::UVSphere {
        radius: 0.25,
        sectors: 72,
        stacks: 36,
      })),
      material: materials.add(StandardMaterial {
        base_color: Color::RED,
        perceptual_roughness: 1.0,
        reflectance: 0.0,
        ..Default::default()
      }),
      transform: Transform::from_xyz(0.5, 0., 0.),
      ..Default::default()
    }
  );  

  // Y
  commands.spawn(
    PbrBundle {
      mesh: meshes.add(Mesh::from(shape::UVSphere {
        radius: 0.25,
        sectors: 72,
        stacks: 36,
      })),
      material: materials.add(StandardMaterial {
        base_color: Color::GREEN,
        perceptual_roughness: 1.0,
        reflectance: 0.0,
        ..Default::default()
      }),
      transform: Transform::from_xyz(0., 0.5, 0.),
      ..Default::default()
    }
  );  

  // Z
  commands.spawn(
    PbrBundle {
      mesh: meshes.add(Mesh::from(shape::UVSphere {
        radius: 0.25,
        sectors: 72,
        stacks: 36,
      })),
      material: materials.add(StandardMaterial {
        base_color: Color::BLUE,
        perceptual_roughness: 1.0,
        reflectance: 0.0,
        ..Default::default()
      }),
      transform: Transform::from_xyz(0., 0., 0.5),
      ..Default::default()
    }
  );

  info!("bau bau!");
}