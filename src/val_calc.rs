use bevy::prelude::*;

use crate::ROOT_FONT_SIZE;


pub fn spawn_val_calc_ui(
  mut commands: Commands
) {
  commands.spawn(
    NodeBundle {
      style: Style {
        position_type: PositionType::Absolute,
        top: Val::Px(ROOT_FONT_SIZE),
        left: Val::Px(ROOT_FONT_SIZE * 10.),
        // height: Val::Calc(|properties| {// properties include: parent height and width, viewport height an width, root font size and inherited font size
        //   properties.parent.height - (properties.font_size.root * 2.) //=> f32 as Px
        // }),

        bottom: Val::Px(ROOT_FONT_SIZE),
        // This achieves the same result in this situation, however it's more difficult for child elements


        width: Val::Px(100.),
        min_height: Val::Px(100.),
        ..default()
      },
      background_color: BackgroundColor(Color::WHITE),
      ..default()
    },
  )
  ;
}