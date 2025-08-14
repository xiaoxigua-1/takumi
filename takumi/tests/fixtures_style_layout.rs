use takumi::{
  Color, ContainerNode, Display, FlexDirection, Gap, GridLengthUnit, GridTemplateComponent,
  GridTrackSize, InheritableStyle,
  LengthUnit::{Percentage, Px},
  Style,
};

mod test_utils;
use test_utils::run_style_width_test;

#[test]
fn test_style_flex_basis() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      display: Display::Flex,
      flex_direction: FlexDirection::Row,
      background_color: Some(Color([0, 0, 255, 255])), // Blue background to show container
      ..Default::default()
    },
    children: Some(vec![
      ContainerNode {
        style: Style {
          flex_basis: Px(100.0), // Set flex basis to 100px
          height: Px(50.0),
          background_color: Some(Color([255, 0, 0, 255])), // Red child
          inheritable_style: InheritableStyle {
            ..Default::default()
          },
          ..Default::default()
        },
        children: None,
      }
      .into(),
      ContainerNode {
        style: Style {
          flex_basis: Px(100.0), // Set flex basis to 100px
          height: Px(50.0),
          background_color: Some(Color([0, 255, 0, 255])), // Green child
          inheritable_style: InheritableStyle {
            ..Default::default()
          },
          ..Default::default()
        },
        children: None,
      }
      .into(),
      ContainerNode {
        style: Style {
          flex_basis: Px(100.0), // Set flex basis to 100px
          height: Px(50.0),
          background_color: Some(Color([255, 255, 0, 255])), // Yellow child
          inheritable_style: InheritableStyle {
            ..Default::default()
          },
          ..Default::default()
        },
        children: None,
      }
      .into(),
    ]),
  };

  run_style_width_test(container.into(), "tests/fixtures/style_flex_basis.png");
}

#[test]
fn test_style_flex_direction() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      display: Display::Flex,
      flex_direction: FlexDirection::Column,
      background_color: Some(Color([0, 0, 255, 255])), // Blue background to show container
      ..Default::default()
    },
    children: Some(vec![
      ContainerNode {
        style: Style {
          width: Px(50.0),
          height: Px(50.0),
          background_color: Some(Color([255, 0, 0, 255])), // Red child
          inheritable_style: InheritableStyle {
            ..Default::default()
          },
          ..Default::default()
        },
        children: None,
      }
      .into(),
      ContainerNode {
        style: Style {
          width: Px(50.0),
          height: Px(50.0),
          background_color: Some(Color([0, 255, 0, 255])), // Green child
          inheritable_style: InheritableStyle {
            ..Default::default()
          },
          ..Default::default()
        },
        children: None,
      }
      .into(),
      ContainerNode {
        style: Style {
          width: Px(50.0),
          height: Px(50.0),
          background_color: Some(Color([255, 255, 0, 255])), // Yellow child
          inheritable_style: InheritableStyle {
            ..Default::default()
          },
          ..Default::default()
        },
        children: None,
      }
      .into(),
    ]),
  };

  run_style_width_test(container.into(), "tests/fixtures/style_flex_direction.png");
}

#[test]
fn test_style_gap() {
  let container = ContainerNode {
    style: Style {
      width: Percentage(100.0),
      height: Percentage(100.0),
      display: Display::Flex, // Enable flexbox layout to demonstrate gap
      gap: Gap::Array(Px(20.0), Px(20.0)), // Create spacing between children
      background_color: Some(Color([0, 0, 255, 255])), // Blue background to show container
      ..Default::default()
    },
    children: Some(vec![
      // First child
      ContainerNode {
        style: Style {
          width: Px(50.0),                               // Fixed width
          height: Px(50.0),                              // Fixed height
          background_color: Some(Color([255, 0, 0, 255])), // Red child
          inheritable_style: InheritableStyle {
            ..Default::default()
          },
          ..Default::default()
        },
        children: None,
      }
      .into(),
      // Second child
      ContainerNode {
        style: Style {
          width: Px(50.0),                               // Fixed width
          height: Px(50.0),                              // Fixed height
          background_color: Some(Color([0, 255, 0, 255])), // Green child
          inheritable_style: InheritableStyle {
            ..Default::default()
          },
          ..Default::default()
        },
        children: None,
      }
      .into(),
      // Third child
      ContainerNode {
        style: Style {
          width: Px(50.0),                                 // Fixed width
          height: Px(50.0),                                // Fixed height
          background_color: Some(Color([255, 255, 0, 255])), // Yellow child
          inheritable_style: InheritableStyle {
            ..Default::default()
          },
          ..Default::default()
        },
        children: None,
      }
      .into(),
    ]),
  };

  run_style_width_test(container.into(), "tests/fixtures/style_gap.png");
}

#[test]
fn test_style_grid_template_columns() {
  let container = ContainerNode {
    style: Style {
      width: 200.0.into(),
      height: 200.0.into(),
      display: Display::Grid,
      grid_template_columns: Some(vec![
        GridTemplateComponent::Single(GridTrackSize::Fixed(GridLengthUnit::Unit(Px(50.0)))),
        GridTemplateComponent::Single(GridTrackSize::Fixed(GridLengthUnit::Unit(Px(100.0)))),
      ]),
      background_color: Some(Color([0, 0, 255, 255])), // Blue background
      ..Default::default()
    },
    children: Some(vec![
      ContainerNode {
        style: Style {
          background_color: Some(Color([255, 0, 0, 255])), // Red
          ..Default::default()
        },
        children: None,
      }
      .into(),
      ContainerNode {
        style: Style {
          background_color: Some(Color([0, 255, 0, 255])), // Green
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

#[test]
fn test_style_grid_template_rows() {
  let container = ContainerNode {
    style: Style {
      width: 200.0.into(),
      height: 200.0.into(),
      display: Display::Grid,
      grid_template_rows: Some(vec![
        GridTemplateComponent::Single(GridTrackSize::Fixed(GridLengthUnit::Unit(Px(50.0)))),
        GridTemplateComponent::Single(GridTrackSize::Fixed(GridLengthUnit::Unit(Px(100.0)))),
      ]),
      background_color: Some(Color([0, 0, 255, 255])), // Blue background
      ..Default::default()
    },
    children: Some(vec![
      ContainerNode {
        style: Style {
          background_color: Some(Color([255, 0, 0, 255])), // Red
          ..Default::default()
        },
        children: None,
      }
      .into(),
      ContainerNode {
        style: Style {
          background_color: Some(Color([0, 255, 0, 255])), // Green
          ..Default::default()
        },
        children: None,
      }
      .into(),
    ]),
  };

  run_style_width_test(
    container.into(),
    "tests/fixtures/style_grid_template_rows.png",
  );
}
