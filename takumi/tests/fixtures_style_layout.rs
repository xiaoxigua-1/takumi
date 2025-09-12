use takumi::layout::{
  node::ContainerNode,
  style::{
    Color, Display, FlexDirection, Gap, GridLengthUnit, GridTemplateComponent,
    GridTemplateComponents, GridTrackSize,
    LengthUnit::{Percentage, Px},
    StyleBuilder,
  },
};

mod test_utils;
use test_utils::run_style_width_test;

#[test]
fn test_style_flex_basis() {
  let container = ContainerNode {
    style: StyleBuilder::default()
      .width(Percentage(100.0))
      .height(Percentage(100.0))
      .display(Display::Flex)
      .flex_direction(FlexDirection::Row)
      .background_color(Some(Color([0, 0, 255, 255])))
      .build()
      .unwrap(),
    children: Some(vec![
      ContainerNode {
        style: StyleBuilder::default()
          .flex_basis(Px(100.0))
          .height(Px(50.0))
          .background_color(Some(Color([255, 0, 0, 255])))
          .build()
          .unwrap(),
        children: None,
      }
      .into(),
      ContainerNode {
        style: StyleBuilder::default()
          .flex_basis(Px(100.0))
          .height(Px(50.0))
          .background_color(Some(Color([0, 255, 0, 255])))
          .build()
          .unwrap(),
        children: None,
      }
      .into(),
      ContainerNode {
        style: StyleBuilder::default()
          .flex_basis(Px(100.0))
          .height(Px(50.0))
          .background_color(Some(Color([255, 255, 0, 255])))
          .build()
          .unwrap(),
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
    style: StyleBuilder::default()
      .width(Percentage(100.0))
      .height(Percentage(100.0))
      .display(Display::Flex)
      .flex_direction(FlexDirection::Column)
      .background_color(Some(Color([0, 0, 255, 255])))
      .build()
      .unwrap(),
    children: Some(vec![
      ContainerNode {
        style: StyleBuilder::default()
          .width(Px(50.0))
          .height(Px(50.0))
          .background_color(Some(Color([255, 0, 0, 255])))
          .build()
          .unwrap(),
        children: None,
      }
      .into(),
      ContainerNode {
        style: StyleBuilder::default()
          .width(Px(50.0))
          .height(Px(50.0))
          .background_color(Some(Color([0, 255, 0, 255])))
          .build()
          .unwrap(),
        children: None,
      }
      .into(),
      ContainerNode {
        style: StyleBuilder::default()
          .width(Px(50.0))
          .height(Px(50.0))
          .background_color(Some(Color([255, 255, 0, 255])))
          .build()
          .unwrap(),
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
    style: StyleBuilder::default()
      .width(Percentage(100.0))
      .height(Percentage(100.0))
      .display(Display::Flex)
      .gap(Gap(Px(20.0), Px(40.0)))
      .background_color(Some(Color([0, 0, 255, 255])))
      .build()
      .unwrap(),
    children: Some(vec![
      // First child
      ContainerNode {
        style: StyleBuilder::default()
          .width(Px(50.0))
          .height(Px(50.0))
          .background_color(Some(Color([255, 0, 0, 255])))
          .build()
          .unwrap(),
        children: None,
      }
      .into(),
      // Second child
      ContainerNode {
        style: StyleBuilder::default()
          .width(Px(50.0))
          .height(Px(50.0))
          .background_color(Some(Color([0, 255, 0, 255])))
          .build()
          .unwrap(),
        children: None,
      }
      .into(),
      // Third child
      ContainerNode {
        style: StyleBuilder::default()
          .width(Px(50.0))
          .height(Px(50.0))
          .background_color(Some(Color([255, 255, 0, 255])))
          .build()
          .unwrap(),
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
    style: StyleBuilder::default()
      .width(Px(200.0))
      .height(Px(200.0))
      .display(Display::Grid)
      .grid_template_columns(Some(GridTemplateComponents(vec![
        GridTemplateComponent::Single(GridTrackSize::Fixed(GridLengthUnit::Unit(Px(50.0)))),
        GridTemplateComponent::Single(GridTrackSize::Fixed(GridLengthUnit::Unit(Px(100.0)))),
      ])))
      .background_color(Some(Color([0, 0, 255, 255])))
      .build()
      .unwrap(),
    children: Some(vec![
      ContainerNode {
        style: StyleBuilder::default()
          .background_color(Some(Color([255, 0, 0, 255])))
          .build()
          .unwrap(),
        children: None,
      }
      .into(),
      ContainerNode {
        style: StyleBuilder::default()
          .background_color(Some(Color([0, 255, 0, 255])))
          .build()
          .unwrap(),
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
    style: StyleBuilder::default()
      .width(Px(200.0))
      .height(Px(200.0))
      .display(Display::Grid)
      .grid_template_rows(Some(GridTemplateComponents(vec![
        GridTemplateComponent::Single(GridTrackSize::Fixed(GridLengthUnit::Unit(Px(50.0)))),
        GridTemplateComponent::Single(GridTrackSize::Fixed(GridLengthUnit::Unit(Px(100.0)))),
      ])))
      .background_color(Some(Color([0, 0, 255, 255])))
      .build()
      .unwrap(),
    children: Some(vec![
      ContainerNode {
        style: StyleBuilder::default()
          .background_color(Some(Color([255, 0, 0, 255])))
          .build()
          .unwrap(),
        children: None,
      }
      .into(),
      ContainerNode {
        style: StyleBuilder::default()
          .background_color(Some(Color([0, 255, 0, 255])))
          .build()
          .unwrap(),
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
