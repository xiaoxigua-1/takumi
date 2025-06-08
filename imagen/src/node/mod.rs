pub mod draw;
pub mod measure;
pub mod style;

use std::fmt::Debug;
use std::sync::{Arc, OnceLock};

use async_trait::async_trait;
use dyn_clone::{DynClone, clone_trait_object};
use futures_util::future::join_all;
use image::{Rgba, RgbaImage};
use imageproc::{
  drawing::{Blend, draw_filled_rect_mut, draw_hollow_rect_mut},
  rect::Rect,
};
use serde::{Deserialize, Serialize};
use taffy::{AvailableSpace, Layout, NodeId, Size, TaffyError};

use crate::{
  color::Color,
  context::Context,
  node::{
    draw::{ImageState, draw_image, draw_text},
    measure::{measure_image, measure_text},
    style::{Background, Style},
  },
  render::TaffyTreeWithNodes,
};

#[typetag::serde(tag = "type")]
#[async_trait]
pub trait Node: Send + Sync + Debug + DynClone {
  fn get_style(&self) -> &Style;

  fn should_hydrate_async(&self) -> bool {
    false
  }

  async fn hydrate_async(&self, _context: &Context) {}

  fn measure(
    &self,
    _context: &Context,
    _available_space: Size<AvailableSpace>,
    _known_dimensions: Size<Option<f32>>,
  ) -> Size<f32> {
    Size::ZERO
  }

  fn draw_on_canvas(&self, _context: &Context, canvas: &mut Blend<RgbaImage>, layout: Layout) {
    if let Some(background) = &self.get_style().background {
      draw_background(background, canvas, layout);
    }
  }

  fn create_taffy_leaf(&self, taffy: &mut TaffyTreeWithNodes) -> Result<NodeId, TaffyError>;
}

clone_trait_object!(Node);

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ContainerNode {
  #[serde(default, flatten)]
  pub style: Style,
  pub children: Vec<Box<dyn Node>>,
}

#[async_trait]
#[typetag::serde(name = "container")]
impl Node for ContainerNode {
  fn get_style(&self) -> &Style {
    &self.style
  }

  fn should_hydrate_async(&self) -> bool {
    self
      .children
      .iter()
      .any(|child| child.should_hydrate_async())
  }

  async fn hydrate_async(&self, context: &Context) {
    let futures = self.children.iter().filter_map(|child| {
      if child.should_hydrate_async() {
        Some(child.hydrate_async(context))
      } else {
        None
      }
    });

    join_all(futures).await;
  }

  fn create_taffy_leaf(&self, taffy: &mut TaffyTreeWithNodes) -> Result<NodeId, TaffyError> {
    let taffy_style = self.style.clone().into();

    let children = self
      .children
      .iter()
      .map(|child| child.create_taffy_leaf(taffy))
      .collect::<Result<Vec<NodeId>, TaffyError>>()?;

    let container = taffy.new_with_children(taffy_style, children.as_slice())?;

    taffy.set_node_context(
      container,
      Some(Box::new(ContainerNode {
        style: self.style.clone(),
        children: vec![],
      })),
    )?;

    Ok(container)
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TextNode {
  #[serde(default, flatten)]
  pub style: Style,
  pub text: String,
}

#[typetag::serde(name = "text")]
impl Node for TextNode {
  fn get_style(&self) -> &Style {
    &self.style
  }

  fn create_taffy_leaf(&self, taffy: &mut TaffyTreeWithNodes) -> Result<NodeId, TaffyError> {
    taffy.new_leaf_with_context(self.style.clone().into(), Box::new(self.clone()))
  }

  fn draw_on_canvas(&self, context: &Context, canvas: &mut Blend<RgbaImage>, layout: Layout) {
    if let Some(background) = &self.style.background {
      draw_background(background, canvas, layout);
    }

    draw_text(
      &self.text,
      &(&self.style).into(),
      &context.font_context,
      canvas,
      layout,
    );
  }

  fn measure(
    &self,
    context: &Context,
    available_space: Size<AvailableSpace>,
    known_dimensions: Size<Option<f32>>,
  ) -> Size<f32> {
    measure_text(
      &context.font_context,
      &self.text,
      &(&self.style).into(),
      known_dimensions,
      available_space,
    )
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImageNode {
  #[serde(default, flatten)]
  pub style: Style,
  pub src: String,
  #[serde(skip)]
  pub image: Arc<OnceLock<Arc<ImageState>>>,
}

#[typetag::serde(name = "image")]
#[async_trait]
impl Node for ImageNode {
  fn get_style(&self) -> &Style {
    &self.style
  }

  fn create_taffy_leaf(&self, taffy: &mut TaffyTreeWithNodes) -> Result<NodeId, TaffyError> {
    taffy.new_leaf_with_context(self.style.clone().into(), Box::new(self.clone()))
  }

  fn should_hydrate_async(&self) -> bool {
    self.image.get().is_none()
  }

  async fn hydrate_async(&self, context: &Context) {
    let image = self.image.clone();
    let image_store = &context.image_store;

    if let Some(img) = image_store.get(&self.src) {
      image.set(img).unwrap();
      return;
    }

    let img = image_store.fetch_async(&self.src).await;

    image_store.insert(self.src.clone(), img.clone());
    image.set(img).unwrap();
  }

  fn measure(
    &self,
    _context: &Context,
    available_space: Size<AvailableSpace>,
    known_dimensions: Size<Option<f32>>,
  ) -> Size<f32> {
    let ImageState::Fetched(image) = self.image.get().unwrap().as_ref() else {
      return Size::ZERO;
    };

    let (width, height) = image.dimensions();

    measure_image(
      Size {
        width: width as f32,
        height: height as f32,
      },
      known_dimensions,
      available_space,
    )
  }

  fn draw_on_canvas(&self, _context: &Context, canvas: &mut Blend<RgbaImage>, layout: Layout) {
    if let Some(background) = &self.style.background {
      draw_background(background, canvas, layout);
    }

    let ImageState::Fetched(image) = self.image.get().unwrap().as_ref() else {
      return;
    };

    draw_image(image, &self.style, canvas, layout);
  }
}

pub fn draw_background(background: &Background, canvas: &mut Blend<RgbaImage>, layout: Layout) {
  match background {
    Background::Color(color) => draw_background_color(*color, canvas, layout),
    Background::Image(_src) => unimplemented!(),
  }
}

pub fn draw_background_color(color: Color, canvas: &mut Blend<RgbaImage>, layout: Layout) {
  let rect = Rect::at(layout.location.x as i32, layout.location.y as i32)
    .of_size(layout.size.width as u32, layout.size.height as u32);

  draw_filled_rect_mut(canvas, rect, color.into());
}

pub fn draw_debug_border(canvas: &mut Blend<RgbaImage>, layout: Layout) {
  let x = layout.content_box_x();
  let y = layout.content_box_y();
  let size = layout.content_box_size();

  draw_hollow_rect_mut(
    canvas,
    Rect::at(x as i32, y as i32).of_size(size.width as u32, size.height as u32),
    Rgba([255, 0, 0, 100]),
  );

  draw_hollow_rect_mut(
    canvas,
    Rect::at(layout.location.x as i32, layout.location.y as i32)
      .of_size(layout.size.width as u32, layout.size.height as u32),
    Rgba([0, 255, 0, 100]),
  );
}
