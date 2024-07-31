use bevy::prelude::*;
use button_stuff::{button_plugin, handle_button, spawn_button_ui};
use rem::spawn_rem_ui;
use style_builder::spawn_style_builder_ui;
use util::run_utilities;
use val_calc::spawn_val_calc_ui;

mod util;

mod rem;
mod val_calc;
mod style_builder; // Opinionated, Implemented
mod button_stuff; // Opinionated, implemented

pub const ROOT_FONT_SIZE: f32 = 16.;

fn main() {
  let mut app = App::new();

  app.add_plugins(DefaultPlugins);
  app.add_plugins((
    run_utilities,
    button_plugin,
  ));

  app.add_systems(
    Startup,
    (
      spawn_rem_ui,
      spawn_val_calc_ui,
      spawn_button_ui,
      spawn_style_builder_ui,
    )
  );

  app.add_systems(
    Update,
    (
      handle_button,
    )
  );

  app.run();
}
