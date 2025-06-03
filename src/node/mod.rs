pub mod draw;
pub mod properties;

use image::RgbaImage;
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use serde::Deserialize;
use taffy::{Layout, NodeId, TaffyError, TaffyTree, style::Style};

use crate::{
  color::Color,
  context::Context,
  node::{
    draw::{draw_circle, draw_image, draw_rect, draw_text},
    properties::{
      CircleProperties, ContainerProperties, ImageProperties, RectProperties, TextProperties,
    },
  },
};

#[derive(Debug, Clone, Deserialize)]
pub struct Node {
  pub background_color: Option<Color>,
  pub border_color: Option<Color>,
  pub style: Option<Style>,
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
    if let NodeProperties::Container(props) = self.properties {
      let children = props
        .children
        .into_iter()
        .map(|child| child.create_taffy_leaf(taffy))
        .collect::<Result<Vec<NodeId>, TaffyError>>()?;

      let container =
        taffy.new_with_children(self.style.unwrap_or_default(), children.as_slice())?;

      taffy.set_node_context(
        container,
        Some(Node {
          background_color: self.background_color,
          border_color: self.border_color,
          style: None,
          properties: NodeProperties::Space,
        }),
      )?;

      Ok(container)
    } else {
      taffy.new_leaf_with_context(
        self.style.unwrap_or_default(),
        Node {
          background_color: self.background_color,
          border_color: self.border_color,
          style: None,
          properties: self.properties,
        },
      )
    }
  }
}

impl Node {
  pub fn render(&self, context: &Context, canvas: &mut RgbaImage, layout: Layout) {
    if let Some(background_color) = self.background_color {
      let rect = Rect::at(layout.location.x as i32, layout.location.y as i32)
        .of_size(layout.size.width as u32, layout.size.height as u32);
      draw_filled_rect_mut(canvas, rect, background_color.into());
    }

    match &self.properties {
      NodeProperties::Rect(props) => draw_rect(props, canvas, layout),
      NodeProperties::Circle(props) => draw_circle(props, canvas, layout),
      NodeProperties::Text(props) => draw_text(props, context, canvas, layout),
      NodeProperties::Image(props) => draw_image(props, context, canvas, layout),
      NodeProperties::Container(_) | NodeProperties::Space => {}
    }
  }
}
