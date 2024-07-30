use bevy::{prelude::*, ui::FocusPolicy};

use crate::{rem::FontSize, util::element_builder::{create_text, FontColor, TextSections}, ROOT_FONT_SIZE};

#[derive(Default)]
pub struct OptionalStyle {
  pub display: Option<Display>,
  pub position_type: Option<PositionType>,
  pub overflow: Option<Overflow>,
  pub direction: Option<Direction>,
  pub left: Option<Val>,
  pub right: Option<Val>,
  pub top: Option<Val>,
  pub bottom: Option<Val>,
  pub width: Option<Val>,
  pub height: Option<Val>,
  pub min_width: Option<Val>,
  pub min_height: Option<Val>,
  pub max_width: Option<Val>,
  pub max_height: Option<Val>,
  pub aspect_ratio: Option<Option<f32>>,
  pub align_items: Option<AlignItems>,
  pub justify_items: Option<JustifyItems>,
  pub align_self: Option<AlignSelf>,
  pub justify_self: Option<JustifySelf>,
  pub align_content: Option<AlignContent>,
  pub justify_content: Option<JustifyContent>,
  pub margin: Option<UiRect>,
  pub padding: Option<UiRect>,
  pub border: Option<UiRect>,
  pub flex_direction: Option<FlexDirection>,
  pub flex_wrap: Option<FlexWrap>,
  pub flex_grow: Option<f32>,
  pub flex_shrink: Option<f32>,
  pub flex_basis: Option<Val>,
  pub row_gap: Option<Val>,
  pub column_gap: Option<Val>,
  pub grid_auto_flow: Option<GridAutoFlow>,
  pub grid_template_rows: Option<Vec<RepeatedGridTrack>>,
  pub grid_template_columns: Option<Vec<RepeatedGridTrack>>,
  pub grid_auto_rows: Option<Vec<GridTrack>>,
  pub grid_auto_columns: Option<Vec<GridTrack>>,
  pub grid_row: Option<GridPlacement>,
  pub grid_column: Option<GridPlacement>,
}

#[derive(Default)]
pub struct OtherStyles {
  pub background_color: BackgroundColor,
  pub border_color: BorderColor,
  pub border_radius: BorderRadius,
  pub focus_policy: FocusPolicy,
  pub transform: Transform,
  pub global_transform: GlobalTransform,
  pub visibility: Visibility,
  pub inherited_visibility: InheritedVisibility,
  pub view_visibility: ViewVisibility,
  pub z_index: ZIndex,
}

#[derive(Default)]
pub struct OptionalOtherStyles {
  pub background_color: Option<BackgroundColor>,
  pub border_color: Option<BorderColor>,
  pub border_radius: Option<BorderRadius>,
  pub focus_policy: Option<FocusPolicy>,
  pub transform: Option<Transform>,
  pub global_transform: Option<GlobalTransform>,
  pub visibility: Option<Visibility>,
  pub inherited_visibility: Option<InheritedVisibility>,
  pub view_visibility: Option<ViewVisibility>,
  pub z_index: Option<ZIndex>,
}

struct StyleBuilder {
  style: Style,
  other: OtherStyles,
}
impl StyleBuilder {
  fn new() -> Self {
    Self {
      style: Style::default(),
      other: OtherStyles::default(),
    }
  }
  fn from(
    style:Option<Style>,
    other: Option<OtherStyles>
  ) -> Self {
    Self {
      style: style.unwrap_or_default(),
      other: other.unwrap_or_default(),
    }
  }
  fn combine(self, optional_style: OptionalStyle) -> Self {
    Self {
      style: Style {
        display: optional_style.display.unwrap_or(self.style.display),
        position_type: optional_style.position_type.unwrap_or(self.style.position_type),
        overflow: optional_style.overflow.unwrap_or(self.style.overflow),
        direction: optional_style.direction.unwrap_or(self.style.direction),
        left: optional_style.left.unwrap_or(self.style.left),
        right: optional_style.right.unwrap_or(self.style.right),
        top: optional_style.top.unwrap_or(self.style.top),
        bottom: optional_style.bottom.unwrap_or(self.style.bottom),
        width: optional_style.width.unwrap_or(self.style.width),
        height: optional_style.height.unwrap_or(self.style.height),
        min_width: optional_style.min_width.unwrap_or(self.style.min_width),
        min_height: optional_style.min_height.unwrap_or(self.style.min_height),
        max_width: optional_style.max_width.unwrap_or(self.style.max_width),
        max_height: optional_style.max_height.unwrap_or(self.style.max_height),
        aspect_ratio: optional_style.aspect_ratio.unwrap_or(self.style.aspect_ratio),
        align_items: optional_style.align_items.unwrap_or(self.style.align_items),
        justify_items: optional_style.justify_items.unwrap_or(self.style.justify_items),
        align_self: optional_style.align_self.unwrap_or(self.style.align_self),
        justify_self: optional_style.justify_self.unwrap_or(self.style.justify_self),
        align_content: optional_style.align_content.unwrap_or(self.style.align_content),
        justify_content: optional_style.justify_content.unwrap_or(self.style.justify_content),
        margin: optional_style.margin.unwrap_or(self.style.margin),
        padding: optional_style.padding.unwrap_or(self.style.padding),
        border: optional_style.border.unwrap_or(self.style.border),
        flex_direction: optional_style.flex_direction.unwrap_or(self.style.flex_direction),
        flex_wrap: optional_style.flex_wrap.unwrap_or(self.style.flex_wrap),
        flex_grow: optional_style.flex_grow.unwrap_or(self.style.flex_grow),
        flex_shrink: optional_style.flex_shrink.unwrap_or(self.style.flex_shrink),
        flex_basis: optional_style.flex_basis.unwrap_or(self.style.flex_basis),
        row_gap: optional_style.row_gap.unwrap_or(self.style.row_gap),
        column_gap: optional_style.column_gap.unwrap_or(self.style.column_gap),
        grid_auto_flow: optional_style.grid_auto_flow.unwrap_or(self.style.grid_auto_flow),
        grid_template_rows: optional_style.grid_template_rows.unwrap_or(self.style.grid_template_rows),
        grid_template_columns: optional_style.grid_template_columns.unwrap_or(self.style.grid_template_columns),
        grid_auto_rows: optional_style.grid_auto_rows.unwrap_or(self.style.grid_auto_rows),
        grid_auto_columns: optional_style.grid_auto_columns.unwrap_or(self.style.grid_auto_columns),
        grid_row: optional_style.grid_row.unwrap_or(self.style.grid_row),
        grid_column: optional_style.grid_column.unwrap_or(self.style.grid_column),
      },
      other: self.other,
    }
  }
  

  fn combine_other(self, optional_styles: OptionalOtherStyles) -> Self {
    Self {
      style: self.style,
      other: OtherStyles {
        background_color: optional_styles.background_color.unwrap_or(self.other.background_color),
        border_color: optional_styles.border_color.unwrap_or(self.other.border_color),
        border_radius: optional_styles.border_radius.unwrap_or(self.other.border_radius),
        focus_policy: optional_styles.focus_policy.unwrap_or(self.other.focus_policy),
        transform: optional_styles.transform.unwrap_or(self.other.transform),
        global_transform: optional_styles.global_transform.unwrap_or(self.other.global_transform),
        visibility: optional_styles.visibility.unwrap_or(self.other.visibility),
        inherited_visibility: optional_styles.inherited_visibility.unwrap_or(self.other.inherited_visibility),
        view_visibility: optional_styles.view_visibility.unwrap_or(self.other.view_visibility),
        z_index: optional_styles.z_index.unwrap_or(self.other.z_index),
      }
    }
  }
}

impl Into<Style> for StyleBuilder {
  fn into(self) -> Style {
    self.style
  }
}

impl Into<NodeBundle> for StyleBuilder {
  fn into(self) -> NodeBundle {
    NodeBundle {
      background_color: self.other.background_color,
      border_color: self.other.border_color,
      border_radius: self.other.border_radius,
      focus_policy: self.other.focus_policy,
      transform: self.other.transform,
      global_transform: self.other.global_transform,
      visibility: self.other.visibility,
      inherited_visibility: self.other.inherited_visibility,
      view_visibility: self.other.view_visibility,
      z_index: self.other.z_index,
      style: self.style,
      ..default()
    }
  }
}

// Usage

struct CardBase;
impl CardBase {
  fn get() -> StyleBuilder {
    StyleBuilder::from(
      Some(Style {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        align_content: AlignContent::Center,
        border: UiRect::all(Val::Px(2.)),
        ..default()
      }),
      Some(OtherStyles {
        background_color: BackgroundColor(
          Color::oklch(0.43, 0., 0.)
        ),
        border_color: BorderColor(
          Color::oklcha(1., 0., 0., 1.)
        ),
        ..default()
      })
    )
  }
}

struct BlueCard;
impl BlueCard {
  fn get() -> StyleBuilder {
    CardBase::get()
      .combine(OptionalStyle {
        border: Some(UiRect::all(Val::Px(4.))),
        ..default()
      }).combine_other(OptionalOtherStyles {
        background_color: Some(BackgroundColor(
          Color::oklch(0.64, 0.177, 250.)
        )),
        ..default()
      })
  }
}

struct GeneralCard;
impl GeneralCard {
  fn get(background_color: Color) -> StyleBuilder {
    CardBase::get()
      .combine_other(OptionalOtherStyles {
        background_color: Some(BackgroundColor(
          background_color
        )),
        ..default()
      })
  }
}

struct CardText;
impl CardText {
  fn get() -> StyleBuilder {
    StyleBuilder::from(
      Some(Style {
        margin: UiRect::all(Val::Px(ROOT_FONT_SIZE / 2.)),
        ..default()
      }),
      None
    )
  }
}


pub fn spawn_style_builder_ui(
  mut commands: Commands
) {
  commands.spawn(
    NodeBundle {
      style: Style {
        position_type: PositionType::Absolute,
        bottom: Val::Px(ROOT_FONT_SIZE),
        left: Val::ZERO,
        right: Val::ZERO,
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        row_gap: Val::Px(ROOT_FONT_SIZE),
        ..default()
      },
      ..default()
    }
  ).with_children(|child_builder| {
    child_builder.spawn(
      NodeBundle {
        ..CardBase::get().into() // Applies styles and other stuff
      }
    ).with_children(|child_builder| {
      child_builder.spawn(
        TextBundle {
          style: CardText::get().into(), // Can be used directly on style
          ..create_text(
            TextSections::Single("Card Base"),
            FontSize::Rem(1.),
            ROOT_FONT_SIZE,
            FontColor::Text.on_dark()
          )
        }
      )
      ;
    })
    ;
    child_builder.spawn(
      NodeBundle {
        ..BlueCard::get().into()
      }
    ).with_children(|child_builder| {
      child_builder.spawn(
        TextBundle {
          style: CardText::get().into(),
          ..create_text(
            TextSections::Single("Blue Card 2"),
            FontSize::Rem(1.),
            ROOT_FONT_SIZE,
            FontColor::Text.on_light()
          )
        }
      )
      ;
    })
    ;

    child_builder.spawn(
      NodeBundle {
        ..GeneralCard::get(
          Color::oklch(0.571, 0.259, 287.)
        ).into() // Applies styles and other stuff
      }
    ).with_children(|child_builder| {
      child_builder.spawn(
        TextBundle {
          style: CardText::get().into(),
          ..create_text(
            TextSections::Single("Custom Card 2"),
            FontSize::Rem(1.),
            ROOT_FONT_SIZE,
            FontColor::Text.on_light()
          )
        }
      )
      ;
    })
    ;
  })
  ;
}