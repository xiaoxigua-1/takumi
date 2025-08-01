use takumi::{
  Color, ContainerNode, Display, GridTrackSize, LengthUnit::Px, Style, TrackSizingFunction,
};

mod test_utils;
use test_utils::run_style_width_test;

#[test]
fn test_style_grid_template_columns() {
  let container = ContainerNode {
    style: Style {
      width: 200.0.into(),
      height: 200.0.into(),
      display: Display::Grid,
      grid_template_columns: Some(vec![
        TrackSizingFunction::Single(GridTrackSize::Unit(Px(50.0))),
        TrackSizingFunction::Single(GridTrackSize::Unit(Px(100.0))),
      ]),
      background_color: Some(Color::Rgb(0, 0, 255).into()), // Blue background
      ..Default::default()
    },
    children: Some(vec![
      ContainerNode {
        style: Style {
          background_color: Some(Color::Rgb(255, 0, 0).into()), // Red
          ..Default::default()
        },
        children: None,
      }
      .into(),
      ContainerNode {
        style: Style {
          background_color: Some(Color::Rgb(0, 255, 0).into()), // Green
          ..Default::default()
        },
        children: None,
      }
      .into(),
    ]),
  };

  run_style_width_test(
    container.into(),
    "tests/fixtures/style_grid_template_columns.png",
  );
}
