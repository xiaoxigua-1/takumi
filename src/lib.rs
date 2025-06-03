use ab_glyph::{FontArc, PxScale};
use async_trait::async_trait;
use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_filled_rect_mut, draw_text_mut};
use lru::LruCache;
use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap,
  io::Cursor,
  num::NonZeroUsize,
  sync::{Arc, Mutex},
};
use taffy::{
  TaffyTree,
  prelude::*,
  style::{AlignItems, AvailableSpace, Dimension, FlexDirection, JustifyContent, Style},
};

// LRU Cache for images
type ImageCache = Arc<Mutex<LruCache<String, DynamicImage>>>;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Position {
  Inherit,
  Fixed { x: f32, y: f32 },
}

impl Default for Position {
  fn default() -> Self {
    Position::Inherit
  }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

impl Default for Color {
  fn default() -> Self {
    Color {
      r: 0,
      g: 0,
      b: 0,
      a: 255,
    }
  }
}

impl From<Color> for Rgba<u8> {
  fn from(color: Color) -> Self {
    Rgba([color.r, color.g, color.b, color.a])
  }
}

// Trait for node rendering and sizing
#[async_trait]
pub trait NodeTrait {
  fn get_size(&self) -> (Option<f32>, Option<f32>);

  async fn render(
    &self,
    canvas: &mut RgbaImage,
    x: f32,
    y: f32,
    image_generator: &ImageGenerator,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct TextNode {
  pub content: String,
  pub font_size: Option<f32>,
  pub color: Option<Color>,
}

#[async_trait]
impl NodeTrait for TextNode {
  fn get_size(&self) -> (Option<f32>, Option<f32>) {
    let font_size = self.font_size.unwrap_or(16.0);
    let approx_width = self.content.len() as f32 * font_size * 0.6;
    let approx_height = font_size * 1.2;
    (Some(approx_width), Some(approx_height))
  }

  async fn render(
    &self,
    canvas: &mut RgbaImage,
    x: f32,
    y: f32,
    image_generator: &ImageGenerator,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let color = self.color.clone().unwrap_or_default();
    let font_size = self.font_size.unwrap_or(16.0);
    let scale = PxScale::from(font_size);

    draw_text_mut(
      canvas,
      color.into(),
      x as i32,
      y as i32,
      scale,
      &image_generator.font,
      &self.content,
    );
    Ok(())
  }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImageNode {
  pub src: String,
  pub width: Option<f32>,
  pub height: Option<f32>,
}

#[async_trait]
impl NodeTrait for ImageNode {
  fn get_size(&self) -> (Option<f32>, Option<f32>) {
    (self.width, self.height)
  }

  async fn render(
    &self,
    canvas: &mut RgbaImage,
    x: f32,
    y: f32,
    image_generator: &ImageGenerator,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Ok(image) = image_generator.load_image(&self.src).await {
      let resized = if let (Some(w), Some(h)) = (self.width, self.height) {
        image.resize_exact(w as u32, h as u32, image::imageops::FilterType::Lanczos3)
      } else {
        image
      };

      let rgba_img = resized.to_rgba8();
      for (dx, dy, pixel) in rgba_img.enumerate_pixels() {
        let canvas_x = x as u32 + dx;
        let canvas_y = y as u32 + dy;

        if canvas_x < canvas.width() && canvas_y < canvas.height() {
          canvas.put_pixel(canvas_x, canvas_y, *pixel);
        }
      }
    }
    Ok(())
  }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RectNode {
  pub width: f32,
  pub height: f32,
  pub color: Option<Color>,
}

#[async_trait]
impl NodeTrait for RectNode {
  fn get_size(&self) -> (Option<f32>, Option<f32>) {
    (Some(self.width), Some(self.height))
  }

  async fn render(
    &self,
    canvas: &mut RgbaImage,
    x: f32,
    y: f32,
    _image_generator: &ImageGenerator,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let color = self.color.clone().unwrap_or_default();
    let rect_bounds =
      imageproc::rect::Rect::at(x as i32, y as i32).of_size(self.width as u32, self.height as u32);
    draw_filled_rect_mut(canvas, rect_bounds, color.into());
    Ok(())
  }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CircleNode {
  pub radius: f32,
  pub color: Option<Color>,
}

#[async_trait]
impl NodeTrait for CircleNode {
  fn get_size(&self) -> (Option<f32>, Option<f32>) {
    let diameter = self.radius * 2.0;
    (Some(diameter), Some(diameter))
  }

  async fn render(
    &self,
    canvas: &mut RgbaImage,
    x: f32,
    y: f32,
    _image_generator: &ImageGenerator,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let color = self.color.clone().unwrap_or_default();
    let center_x = x + self.radius;
    let center_y = y + self.radius;

    draw_filled_circle_mut(
      canvas,
      (center_x as i32, center_y as i32),
      self.radius as i32,
      color.into(),
    );
    Ok(())
  }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SpaceNode {
  pub width: Option<f32>,
  pub height: Option<f32>,
}

#[async_trait]
impl NodeTrait for SpaceNode {
  fn get_size(&self) -> (Option<f32>, Option<f32>) {
    (self.width, self.height)
  }

  async fn render(
    &self,
    _canvas: &mut RgbaImage,
    _x: f32,
    _y: f32,
    _image_generator: &ImageGenerator,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Space nodes don't render anything
    Ok(())
  }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NodeType {
  Text(TextNode),
  Image(ImageNode),
  Rect(RectNode),
  Circle(CircleNode),
  Space(SpaceNode),
}

#[async_trait]
impl NodeTrait for NodeType {
  fn get_size(&self) -> (Option<f32>, Option<f32>) {
    match self {
      NodeType::Text(text) => text.get_size(),
      NodeType::Image(img) => img.get_size(),
      NodeType::Rect(rect) => rect.get_size(),
      NodeType::Circle(circle) => circle.get_size(),
      NodeType::Space(space) => space.get_size(),
    }
  }

  async fn render(
    &self,
    canvas: &mut RgbaImage,
    x: f32,
    y: f32,
    image_generator: &ImageGenerator,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match self {
      NodeType::Text(text) => text.render(canvas, x, y, image_generator).await,
      NodeType::Image(img) => img.render(canvas, x, y, image_generator).await,
      NodeType::Rect(rect) => rect.render(canvas, x, y, image_generator).await,
      NodeType::Circle(circle) => circle.render(canvas, x, y, image_generator).await,
      NodeType::Space(space) => space.render(canvas, x, y, image_generator).await,
    }
  }
}

#[derive(Debug, Clone, Deserialize)]
pub struct FlexStyle {
  pub direction: Option<String>,
  pub justify_content: Option<String>,
  pub align_items: Option<String>,
  pub gap: Option<f32>,
  pub padding: Option<f32>,
}

impl Default for FlexStyle {
  fn default() -> Self {
    FlexStyle {
      direction: Some("column".to_string()),
      justify_content: Some("flex-start".to_string()),
      align_items: Some("flex-start".to_string()),
      gap: None,
      padding: None,
    }
  }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Node {
  #[serde(flatten)]
  pub node_type: NodeType,
  pub position: Option<Position>,
  pub flex_style: Option<FlexStyle>,
  pub children: Option<Vec<Node>>,
}

#[derive(Debug, Deserialize)]
pub struct ImageRequest {
  pub nodes: Vec<Node>,
  pub width: u32,
  pub height: u32,
  pub background_color: Option<Color>,
}

#[derive(Debug, Serialize)]
pub struct ImageResponse {
  pub success: bool,
  pub message: String,
}

pub struct ImageGenerator {
  cache: ImageCache,
  client: reqwest::Client,
  font: FontArc,
}

impl ImageGenerator {
  pub fn new() -> Self {
    let cache = Arc::new(Mutex::new(LruCache::new(NonZeroUsize::new(100).unwrap())));
    let client = reqwest::Client::new();

    // Load a default font (you might want to load from file in production)
    let font_data =
      include_bytes!("../assets/noto-sans-tc-v36-chinese-traditional_latin-regular.woff2");
    let font = FontArc::try_from_slice(font_data).unwrap();

    ImageGenerator {
      cache,
      client,
      font,
    }
  }

  pub async fn generate_image(
    &self,
    request: ImageRequest,
  ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    let mut canvas = ImageBuffer::new(request.width, request.height);

    // Fill background
    let bg_color = request.background_color.unwrap_or_default();
    for pixel in canvas.pixels_mut() {
      *pixel = bg_color.into();
    }

    // Create layout engine
    let mut taffy = TaffyTree::new();
    let mut node_map = HashMap::new();

    // Build layout tree
    let root_style = Style {
      size: Size {
        width: Dimension::from_length(request.width as f32),
        height: Dimension::from_length(request.height as f32),
      },
      ..Default::default()
    };

    let root_node = taffy.new_leaf(root_style)?;

    // Process nodes
    for node in &request.nodes {
      self
        .process_node(&mut taffy, &mut node_map, root_node, node)
        .await?;
    }

    // Compute layout
    taffy.compute_layout(
      root_node,
      Size {
        width: AvailableSpace::Definite(request.width as f32),
        height: AvailableSpace::Definite(request.height as f32),
      },
    )?;

    // Render nodes
    for node in &request.nodes {
      self
        .render_node(&mut canvas, &taffy, &node_map, node, 0.0, 0.0)
        .await?;
    }

    // Convert to PNG bytes
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);

    DynamicImage::ImageRgba8(canvas).write_to(&mut cursor, ImageFormat::Png)?;

    Ok(buffer)
  }

  async fn process_node(
    &self,
    taffy: &mut TaffyTree,
    node_map: &mut HashMap<*const Node, taffy::NodeId>,
    parent: taffy::NodeId,
    node: &Node,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if matches!(node.position, Some(Position::Fixed { .. })) {
      // Skip layout processing for fixed position nodes
      return Ok(());
    }

    let style = self.create_taffy_style(node).await?;
    let mut children = Vec::new();

    // Process children first
    if let Some(child_nodes) = &node.children {
      for child in child_nodes {
        let child_taffy_node = taffy.new_leaf(Style::default())?;
        self
          .process_node(taffy, node_map, child_taffy_node, child)
          .await?;
        children.push(child_taffy_node);
      }
    }

    let taffy_node = if children.is_empty() {
      taffy.new_leaf(style)?
    } else {
      taffy.new_with_children(style, &children)?
    };

    node_map.insert(node as *const Node, taffy_node);

    Ok(())
  }

  async fn create_taffy_style(
    &self,
    node: &Node,
  ) -> Result<Style, Box<dyn std::error::Error + Send + Sync>> {
    let mut style = Style::default();

    // Set flex properties if available
    if let Some(flex_style) = &node.flex_style {
      if let Some(direction) = &flex_style.direction {
        style.flex_direction = match direction.as_str() {
          "row" => FlexDirection::Row,
          "column" => FlexDirection::Column,
          _ => FlexDirection::Column,
        };
      }

      if let Some(justify) = &flex_style.justify_content {
        style.justify_content = match justify.as_str() {
          "center" => Some(JustifyContent::Center),
          "flex-end" => Some(JustifyContent::FlexEnd),
          "space-between" => Some(JustifyContent::SpaceBetween),
          "space-around" => Some(JustifyContent::SpaceAround),
          _ => Some(JustifyContent::FlexStart),
        };
      }

      if let Some(align) = &flex_style.align_items {
        style.align_items = match align.as_str() {
          "center" => Some(AlignItems::Center),
          "flex-end" => Some(AlignItems::FlexEnd),
          "stretch" => Some(AlignItems::Stretch),
          _ => Some(AlignItems::FlexStart),
        };
      }

      // TODO: Add gap and padding support when taffy API is clarified
    }

    // Set size based on node type using the trait
    let (width, height) = node.node_type.get_size();

    style.size = Size {
      width: width.map(Dimension::Length).unwrap_or(Dimension::Auto),
      height: height.map(Dimension::Length).unwrap_or(Dimension::Auto),
    };

    Ok(style)
  }

  async fn render_node(
    &self,
    canvas: &mut RgbaImage,
    taffy: &TaffyTree,
    node_map: &HashMap<*const Node, taffy::NodeId>,
    node: &Node,
    parent_x: f32,
    parent_y: f32,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (x, y) = match &node.position {
      Some(Position::Fixed { x, y }) => (*x, *y),
      _ => {
        if let Some(&taffy_node) = node_map.get(&(node as *const Node)) {
          let layout = taffy.layout(taffy_node)?;
          (parent_x + layout.location.x, parent_y + layout.location.y)
        } else {
          (parent_x, parent_y)
        }
      }
    };

    // Use the trait method directly - no more match needed!
    node.node_type.render(canvas, x, y, self).await?;

    // Render children
    if let Some(children) = &node.children {
      for child in children {
        Box::pin(self.render_node(canvas, taffy, node_map, child, x, y)).await?;
      }
    }

    Ok(())
  }

  pub async fn load_image(
    &self,
    url: &str,
  ) -> Result<DynamicImage, Box<dyn std::error::Error + Send + Sync>> {
    // Check cache first
    {
      let mut cache = self.cache.lock().unwrap();
      if let Some(image) = cache.get(url) {
        return Ok(image.clone());
      }
    }

    // Download image
    let response = self.client.get(url).send().await?;
    let bytes = response.bytes().await?;
    let image = image::load_from_memory(&bytes)?;

    // Store in cache
    {
      let mut cache = self.cache.lock().unwrap();
      cache.put(url.to_string(), image.clone());
    }

    Ok(image)
  }
}
