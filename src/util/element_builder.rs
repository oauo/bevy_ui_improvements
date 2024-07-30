use bevy::prelude::*;

use crate::rem::FontSize;

#[derive(Default)]
pub enum FontColor {
  #[default]
  Text,
  Red,
}

impl FontColor {
  pub fn on_dark(self) -> Color {
    match self {
      FontColor::Text => Color::WHITE,
      FontColor::Red => Color::oklch(0.653, 0.232, 26.5),
    }
  }
  pub fn on_light(self) -> Color {
    match self {
      FontColor::Text => Color::BLACK,
      FontColor::Red => Color::oklch(0.607, 0.219, 26.5),
    }
  }
}

pub enum TextSections<'a> {
  Single(&'a str),
  Multiple(Vec<&'a str>)
}

pub fn create_text(
  value: TextSections,
  font_size: FontSize,
  parent_font_size: f32,
  color: Color,
) -> TextBundle {
  TextBundle {
    text: Text {
      sections: match value {
        TextSections::Single(single) => vec![single],
        TextSections::Multiple(multiple) => multiple,
      }.iter().map(|section_text| {
        TextSection {
          value: section_text.to_string(),
          style: TextStyle {
            font_size: font_size.from_parent(parent_font_size),
            color,
            ..default()
          },
        }
      }).collect(),
      ..default()
    },
    ..default()
  }
}