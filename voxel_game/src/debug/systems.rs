use bevy::prelude::*;

use crate::player::components::*;

use super::*;

pub fn setup(mut commands: Commands) {
  const OVERLAY_MARGIN: Val = Val::Px(5.);
  const TEXT_MARGIN: Val = Val::Px(1.);
  commands.spawn((
    DebugOverlay,
    NodeBundle {
      style: Style {
        width: Val::Percent(100.),
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::FlexEnd,
        justify_content: JustifyContent::FlexStart,
        ..Default::default()
      },
      ..Default::default()
    }
  )).with_children(|builder| {
    builder
      .spawn(NodeBundle {
          style: Style {
              flex_direction: FlexDirection::Column,
              margin: UiRect { left: OVERLAY_MARGIN, right: Val::Px(0.), top: OVERLAY_MARGIN, bottom: OVERLAY_MARGIN },
              ..Default::default()
          },
          background_color: BackgroundColor(Color::Rgba { red: 0., green: 0., blue: 0., alpha: 0.5 }),
          ..Default::default()
      }).with_children(|builder| {
        spawn_nested_text_bundle(
          builder,
          UiRect::horizontal(TEXT_MARGIN),
          "FPS:",
        );
        spawn_nested_text_bundle(
          builder,
          UiRect::horizontal(TEXT_MARGIN),
          "FT (s):",
        );
        spawn_nested_text_bundle(
          builder,
          UiRect::horizontal(TEXT_MARGIN),
          "EC:",
        );
        spawn_nested_text_bundle(
          builder,
          UiRect::horizontal(TEXT_MARGIN),
          "VEL:",
        );
      });

    builder
      .spawn(NodeBundle {
          style: Style {
              flex_direction: FlexDirection::Column,
              margin: UiRect { left: Val::Px(0.), right: OVERLAY_MARGIN, top: OVERLAY_MARGIN, bottom: OVERLAY_MARGIN },
              align_items: AlignItems::FlexEnd,
              min_width: Val::Px(70.),
              ..Default::default()
          },
          background_color: BackgroundColor(Color::Rgba { red: 0., green: 0., blue: 0., alpha: 0.5 }),
          ..Default::default()
      }).with_children(|builder| {
        spawn_nested_text_tag_bundle(
          builder,
          UiRect::horizontal(TEXT_MARGIN),
          DebugOverlayFPS,
        );
        spawn_nested_text_tag_bundle(
          builder,
          UiRect::horizontal(TEXT_MARGIN),
          DebugOverlayFT,
        );
        spawn_nested_text_tag_bundle(
          builder,
          UiRect::horizontal(TEXT_MARGIN),
          DebugOverlayEC,
        );
        spawn_nested_text_tag_bundle(
          builder,
          UiRect::horizontal(TEXT_MARGIN),
          DebugOverlayVEL,
        );
      });
  });
}

pub fn update_fps(mut query: Query<&mut Text, With<DebugOverlayFPS>>, diagnostics: Res<DiagnosticsStore>) {
  query
    .par_iter_mut()
    .for_each_mut(|mut text| {
      if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
          if let Some(section) = text.sections.get_mut(0) {
            section.value = format!("{value:.2}");
          }
        }
      }
    });
}

pub fn update_frame_time(mut query: Query<&mut Text, With<DebugOverlayFT>>, diagnostics: Res<DiagnosticsStore>) {
  query
    .par_iter_mut()
    .for_each_mut(|mut text| {
      if let Some(frame_time) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME) {
        if let Some(value) = frame_time.smoothed() {
          if let Some(section) = text.sections.get_mut(0) {
            section.value = format!("{:.4}", value / 1000.);
          }
        }
      }
    });
}

pub fn update_entity_count(mut query: Query<&mut Text, With<DebugOverlayEC>>, diagnostics: Res<DiagnosticsStore>) {
  query
    .par_iter_mut()
    .for_each_mut(|mut text| {
      if let Some(count) = diagnostics.get(EntityCountDiagnosticsPlugin::ENTITY_COUNT) {
        if let Some(value) = count.value() {
          if let Some(section) = text.sections.get_mut(0) {
            section.value = format!("{value}");
          }
        }
      }
    });
}

pub fn update_velocity(
  state_query: Query<&PlayerMovementState, With<MainPlayer>>,
  mut query: Query<&mut Text, With<DebugOverlayVEL>>,
) {

  query
    .par_iter_mut()
    .for_each_mut(|mut text| {
      if state_query.is_empty() {
        if let Some(section) = text.sections.get_mut(0) {
          section.value = format!("N/A");
        }
      }

      for state in state_query.iter() {
        if let Some(section) = text.sections.get_mut(0) {
          section.value = format!("{:.4}", state.velocity.length());
        }
      }
    });
}

pub fn spawn_nested_text_bundle(
  builder: &mut ChildBuilder,
  margin: UiRect,
  text: &str,
) {
  builder
    .spawn(NodeBundle {
      style: Style {
        margin,
        ..Default::default()
      },
      ..Default::default()
    })
    .with_children(|builder| {
      builder.spawn(
        TextBundle::from_section(
          text,
          TextStyle {
            font_size: 15.0,
            color: Color::WHITE,
            ..Default::default()
          },
        )
      );
    });
}

fn spawn_nested_text_tag_bundle(
  builder: &mut ChildBuilder,
  margin: UiRect,
  tag: impl Component,
) {
  builder
    .spawn(NodeBundle {
      style: Style {
        margin,
        ..Default::default()
      },
      ..Default::default()
    })
    .with_children(|builder| {
      builder.spawn((
        tag,
        TextBundle::from_section(
          "",
          TextStyle {
            font_size: 15.0,
            color: Color::WHITE,
            ..Default::default()
          },
        )
      ));
    });
}