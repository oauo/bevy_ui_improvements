/*
CONTENTS
- Rem - unit based on a root font size, the rem unit should be able to be changed during runtime to allow for easy scaling
- Em - this unit is affected by parent font sizes, starting a 1rem then making it 0.5em, and then 1.5em it's now 1rem*0.5*1.5 = 0.75rem
  base_font_size is entirely about Em, not needed for Rem.

Rem is essential and it should be trivial to include
Em is something that would be amazing, but isn't essential but it's a lot more complex
*/

use bevy::prelude::*;

use crate::{util::element_builder::{create_text, FontColor, TextSections}, ROOT_FONT_SIZE};

#[derive(Resource)]
pub struct RootFontSize(pub f32);

#[derive(Clone, Copy)]
pub enum FontSize {
  Px(f32),
  Rem(f32),
  Em(f32),
}

impl FontSize {
  pub fn from_parent(self, parent_font_size: f32) -> f32 {
    match self {
      FontSize::Px(font_size) => font_size,
      FontSize::Rem(font_size) => font_size * ROOT_FONT_SIZE,
      FontSize::Em(font_size) => font_size * parent_font_size,
    }
  }
}

pub fn spawn_rem_ui(
  mut commands: Commands,
) {
  // commands.insert_resource(RootFontSize(16.));
  commands.insert_resource(RootFontSize(ROOT_FONT_SIZE));
  // As a resource, players who want a larger UI can just edit this resource, make it 24. and now everything is 50% bigger.
  // This should not replace every instance of Px, for instance border thickness gets a little ridiculous if using rem or em.
  // Default value should be the smallest size that is acceptable to read.
  // Additionally using rem and em lets the developer know that things are relative to text and so what they see is what others see,
  //  "px" isn't real pixels, some people might have "1px" stretched across multiple pixels. Rem and em don't care.

  commands.spawn(
    NodeBundle {
      style: Style {
        position_type: PositionType::Absolute,
        top: /*Val::Rem(1.) ==*/ Val::Px(16.),
        left: /*Val::Em(1.) ==*/ Val::Px(16.),
        ..default()
      },
      ..default()
    },
  ).with_children(|child_builder| {
    child_builder.spawn(
      TextBundle {
        text: Text {
          sections: vec![
            TextSection {
              value: "Top left".to_string(),
              style: TextStyle {
                font_size: /*FontSize::Rem(1.5) == FontSize::Px(24.) ==*/ 24.,
                ..default()
              },
            },
          ],
          ..default()
        },
        ..default()
      },
    )
    ;
  })
  ;

  commands.spawn(
    NodeBundle {
      style: Style {
        position_type: PositionType::Absolute,
        top: Val::Px(16.),
        right: Val::Px(16.),

        display: Display::Flex,
        flex_direction: FlexDirection::Column,

        //base_font_size: Val::Rem(2.) /*==Val::Px(32.)*/, // Rem is still 16., but Em is now 32.
        ..default()
      },
      ..default()
    },
  ).with_children(|child_builder| {
    child_builder.spawn(
      TextBundle {
        text: Text {
          sections: vec![
            TextSection {
              value: "Top right".to_string(),
              style: TextStyle {
                font_size: FontSize::Em(1.) /*==32.*/             .from_parent(32.), //There wouldn't actually be a `from_parent()` obviously
                ..default()
              },
            },
          ],
          ..default()
        },
        ..default()
      },
    )
    ;

    child_builder.spawn(
      NodeBundle {
        style: Style {
          // base_font_size: Val::Em(0.75) /*==Val::Px(2. * 16. * 0.75)==Val::Px(24.)*/, // Rem is still 16., Em was inherited as 32. and is turned into 24.
          ..default()
        },
        ..default()
      }
    ).with_children(|child_builder| {
      child_builder.spawn(
        TextBundle {
          text: Text {
            sections: vec![
              TextSection {
                value: "Top left 2".to_string(),
                style: TextStyle {
                  font_size: FontSize::Em(1.) /*== FontSize::Px(24.) == 24.*/ .from_parent(32. * 0.75), // FontSize::Em(1.) is actually default, it doesn't need to be stated
                  ..default()
                },
              },
            ],
            ..default()
          },
          ..default()
        },
      )
      ;
    })
    ;

    child_builder.spawn(NodeBundle {
      style: Style {
        // base_font_size: Val::Rem(1.) /*== 16.*/, // Resets the font size

        display: Display::Flex,
        align_items: AlignItems::Center,
        column_gap: Val::Px(ROOT_FONT_SIZE / 2.),
        ..default()
      },
      ..default()
    })
      .with_children(|child_builder| {
        child_builder.spawn(
          TextBundle {
            ..create_text(
              TextSections::Single("Smaller"),
              FontSize::Em(0.75),
              16.,
              FontColor::default().on_dark()
            )
          },
        )
        ;
        child_builder.spawn(
          TextBundle {
            ..create_text(
              TextSections::Single("Normal"),
              FontSize::Em(1.),
              16.,
              FontColor::default().on_dark()
            )
          },
        )
        ;
        child_builder.spawn(
          TextBundle {
            ..create_text(
              TextSections::Single("Bigger"),
              FontSize::Em(1.5),
              16.,
              FontColor::default().on_dark()
            )
          },
        )
        ;
      })
      ;
  })
  ;
}