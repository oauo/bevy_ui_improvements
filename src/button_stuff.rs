use bevy::prelude::*;

use crate::{rem::FontSize, util::element_builder::{create_text, FontColor, TextSections}, ROOT_FONT_SIZE};

#[derive(Component)]
pub struct LastButtonInteractionWasClick(bool); // Very descriptive name that probably needs to be renamed

fn process_buttons(
  mut buttons: Query<(
    &mut LastButtonInteractionWasClick,
    &Interaction
  ), (
    Changed<Interaction>,
    With<Button>
  )>
) {
  for (
    mut last_clicked,
    interaction,
  ) in &mut buttons {
    match interaction {
      Interaction::Pressed =>
        last_clicked.0 = true,
      Interaction::Hovered | Interaction::None =>
        last_clicked.0 = false,
    }
  }
}

#[derive(Component)]
struct ButtonCursor(CursorIcon);

fn process_button_cursor(
  mut windows: Query<&mut Window>,
  mut buttons: Query<(
    &Interaction,
    &ButtonCursor,
  ), (
    Changed<Interaction>,
    With<Button>
  )>
) {
  let mut window = windows.single_mut();
  for (
    interaction,
    cursor,
  ) in &mut buttons {
    match interaction {
      Interaction::Hovered =>
        window.cursor.icon = cursor.0,
      Interaction::None =>
        window.cursor.icon = CursorIcon::Default,
      _ => {}
    }
  }
}

pub fn button_plugin(
  app: &mut App
) {
  app.add_systems(
    PreUpdate,
    (
      process_buttons,
      process_button_cursor,
    )
  );
}

// Usage

#[derive(Component)]
pub struct Clicks(pub usize);

#[derive(Component)]
pub struct ClicksText;

pub fn handle_button(
  mut buttons: Query<(
    &mut Clicks,
    &LastButtonInteractionWasClick,
    &Children,
  ), (
    Changed<Interaction>,
    With<Button>
  )>,
  mut texts: Query<&mut Text, With<ClicksText>>,
) {
  for (
    mut clicks,
    last_clicked,
    children,
  ) in &mut buttons {
    if last_clicked.0 { // No dealing with interaction when it just isn't necessary
      clicks.0 += 1;
      for &child in children {
        if let Ok(mut text) = texts.get_mut(child) {
          text.sections[1].value = clicks.0.to_string();
        }
      }
    }
  }
}

pub fn spawn_button_ui(
  mut commands: Commands
) {
  commands.spawn((
    LastButtonInteractionWasClick(false),
    ButtonCursor(CursorIcon::Pointer),

    Clicks(0),
    ButtonBundle {
      style: Style {
        position_type: PositionType::Absolute,
        bottom: Val::Px(ROOT_FONT_SIZE),
        right: Val::Px(ROOT_FONT_SIZE),
        ..default()
      },
      ..default()
    }
  )).with_children(|child_builder| {
    child_builder.spawn((
      ClicksText,
      TextBundle {
        ..create_text(
          TextSections::Multiple(vec!["Clicked ", "0", " times"]),
          FontSize::Rem(2.),
          ROOT_FONT_SIZE,
          FontColor::Red.on_light()
        )
      }
    ))
    ;
  })
  ;
}