use bevy::prelude::*;

use crate::game::states::GameState;

use super::{resources::*, components::*};

pub fn setup_menu_system(mut commands: Commands) {
  let camera_entity = commands.spawn(
    Camera2dBundle::default()
  ).id();

  let button_entity = commands.spawn(
    NodeBundle {
      style: Style {
        // center button
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
      },
      ..default()
    }
  ).with_children(|parent| {
    parent
      .spawn(ButtonBundle {
        style: Style {
          width: Val::Px(150.),
          height: Val::Px(65.),
          // horizontally center child text
          justify_content: JustifyContent::Center,
          // vertically center child text
          align_items: AlignItems::Center,
          ..default()
        },
        background_color: NORMAL_BUTTON.into(),
        ..default()
      })
      .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
          "Play",
          TextStyle {
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
            ..default()
          },
        ));
      });
  })
  .id();

  commands.insert_resource(MenuData {
    camera_entity,
    button_entity,
  });
}

#[allow(clippy::type_complexity)]
pub fn menu_system(
  mut next_state: ResMut<NextState<GameState>>,
  mut interaction_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<Button>),
  >,
) {
  for (interaction, mut color) in &mut interaction_query {
    match *interaction {
      Interaction::Pressed => {
        *color = PRESSED_BUTTON.into();
        next_state.set(GameState::InGame);
      }
      Interaction::Hovered => {
        *color = HOVERED_BUTTON.into();
      }
      Interaction::None => {
        *color = NORMAL_BUTTON.into();
      }
    }
  }
}

pub fn cleanup_menu_system(mut commands: Commands, menu_data: Res<MenuData>) {
  commands.entity(menu_data.camera_entity).despawn_recursive();
  commands.entity(menu_data.button_entity).despawn_recursive();
}