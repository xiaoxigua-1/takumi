pub mod draw;
pub mod measure;
pub mod properties;
pub mod style;

use futures_util::future::join_all;
use image::RgbaImage;
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use serde::Deserialize;
use taffy::{AvailableSpace, Layout, NodeId, Size, TaffyError, TaffyTree};

use crate::{
  context::Context,
  node::{
    draw::{draw_circle, draw_image, draw_rect, draw_text},
    measure::{measure_image, measure_text},
    properties::{
      CircleProperties, ContainerProperties, ImageProperties, RectProperties, TextProperties,
    },
    style::Style,
  },
};

#[derive(Debug, Clone, Deserialize)]
pub struct Node {
  #[serde(default)]
  pub style: Style,
  #[serde(flatten)]
  pub properties: NodeProperties,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum NodeProperties {
  Rect(RectProperties),
  Circle(CircleProperties),
  Text(TextProperties),
  Image(ImageProperties),
  Container(ContainerProperties),
  Space,
}

impl Node {
  pub fn create_taffy_leaf(self, taffy: &mut TaffyTree<Node>) -> Result<NodeId, TaffyError> {
    let style = self.style.clone().into();

    if let NodeProperties::Container(props) = self.properties {
      let children = props
        .children
        .into_iter()
        .map(|child| child.create_taffy_leaf(taffy))
        .collect::<Result<Vec<NodeId>, TaffyError>>()?;

      let container = taffy.new_with_children(style, children.as_slice())?;

      taffy.set_node_context(
        container,
        Some(Node {
          style: self.style,
          properties: NodeProperties::Space,
        }),
      )?;

      Ok(container)
    } else {
      taffy.new_leaf_with_context(
        style,
        Node {
          style: Default::default(),
          properties: self.properties,
        },
      )
    }
  }
}

impl Node {
  pub fn measure(
    &self,
    context: &Context,
    available_space: Size<AvailableSpace>,
    known_dimensions: Size<Option<f32>>,
  ) -> Size<f32> {
    match &self.properties {
      NodeProperties::Image(props) => {
        measure_image(context, props, known_dimensions, available_space)
      }
      NodeProperties::Text(props) => measure_text(context, props, available_space),
      _ => Size::ZERO,
    }
  }

  pub async fn hydrate(&self, context: &Context) {
    match &self.properties {
      NodeProperties::Image(props) => props.fetch_and_store(context).await,
      NodeProperties::Container(props) => {
        join_all(props.children.iter().map(|child| child.hydrate(context))).await;
      }
      _ => {}
    }
  }

  pub fn render(&self, context: &Context, canvas: &mut RgbaImage, layout: Layout) {
    if let Some(background_color) = self.style.background_color {
      let rect = Rect::at(layout.location.x as i32, layout.location.y as i32)
        .of_size(layout.size.width as u32, layout.size.height as u32);

      draw_filled_rect_mut(canvas, rect, background_color.into());
    }

    match &self.properties {
      NodeProperties::Rect(props) => draw_rect(props, canvas, layout),
      NodeProperties::Circle(props) => draw_circle(props, canvas, layout),
      NodeProperties::Text(props) => draw_text(props, context, canvas, layout),
      NodeProperties::Image(props) => draw_image(props, context, canvas, layout),
      _ => {}
    }
  }
}
