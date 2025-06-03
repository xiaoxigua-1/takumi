use ab_glyph::{Font as _, PxScale};
use async_trait::async_trait;
use image::{DynamicImage, Rgba, RgbaImage, imageproc::drawing::draw_filled_rect_mut};
use taffy::{
  prelude::*,
  style::{
    AvailableSpace, Dimension, Display, FlexDirection, LengthPercentage, LengthPercentageAuto,
    Position, Style,
  },
};

use crate::{Color, ImageGenerator, TaffyRect};

#[async_trait]
pub trait NodeTrait: std::fmt::Debug + Send + Sync {
  /// Get the size of the node
  fn get_size(&self) -> Size<Dimension> {
    Size {
      width: Dimension::Length(0.0),
      height: Dimension::Length(0.0),
    }
  }

  /// Get the style for the node
  fn get_style(&self) -> Style {
    Style::default()
  }

  /// Render the node onto the canvas
  async fn render(
    &self,
    canvas: &mut RgbaImage,
    generator: &ImageGenerator,
    x: f32,
    y: f32,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
impl NodeTrait for super::TextNode {
  fn get_size(&self) -> Size<Dimension> {
    // Approximate text size
    let font_size = self.font_size.unwrap_or(16.0);
    let approx_width = self.content.len() as f32 * font_size * 0.6;
    let approx_height = font_size * 1.2;

    Size {
      width: Dimension::Length(approx_width),
      height: Dimension::Length(approx_height),
    }
  }

  fn get_style(&self) -> Style {
    Style {
      display: Display::Flex,
      size: self.get_size(),
      ..Default::default()
    }
  }

  async fn render(
    &self,
    _generator: &ImageGenerator,
    canvas: &mut RgbaImage,
    layout: &TaffyRect,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Implementation for text rendering
    Ok(())
  }

  fn get_style(&self) -> Style {
    Style::default()
  }

  async fn render(
    &self,
    canvas: &mut RgbaImage,
    generator: &ImageGenerator,
    x: f32,
    y: f32,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use imageproc::drawing::draw_text_mut;
    use imageproc::pixelops::interpolate;

    let color = self.color.clone().unwrap_or_default();
    let font_size = self.font_size.unwrap_or(16.0);
    let scale = ab_glyph::PxScale::from(font_size);

    draw_text_mut(
      canvas,
      color.into(),
      x as i32,
      y as i32,
      scale,
      &generator.font,
      &self.content,
    );

    Ok(())
  }
}

#[async_trait]
impl NodeTrait for super::RectNode {
  fn get_size(&self) -> Size<Dimension> {
    Size {
      width: Dimension::Length(self.width),
      height: Dimension::Length(self.height),
    }
  }

  fn get_style(&self) -> Style {
    Style {
      display: Display::Flex,
      size: self.get_size(),
      ..Default::default()
    }
  }

  async fn render(
    &self,
    _generator: &ImageGenerator,
    canvas: &mut RgbaImage,
    layout: &TaffyRect,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Implementation for rectangle rendering
    let color = self.color.unwrap_or(Color {
      r: 0,
      g: 0,
      b: 0,
      a: 255,
    });
    let rect = imageproc::rect::Rect::at(layout.location.x as i32, layout.location.y as i32)
      .of_size(layout.size.width as u32, layout.size.height as u32);
    draw_filled_rect_mut(canvas, rect, Rgba([color.r, color.g, color.b, color.a]));
    Ok(())
  }

  fn get_style(&self) -> Style {
    Style::default()
  }

  async fn render(
    &self,
    canvas: &mut RgbaImage,
    _generator: &ImageGenerator,
    x: f32,
    y: f32,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use imageproc::drawing::draw_filled_rect_mut;
    use imageproc::rect::Rect;

    if let Some(color) = &self.color {
      let rect = Rect::at(x as i32, y as i32).of_size(self.width as u32, self.height as u32);
      draw_filled_rect_mut(canvas, rect, (*color).into());
    }

    Ok(())
  }
}

#[async_trait]
impl NodeTrait for super::CircleNode {
  fn get_size(&self) -> Size<Dimension> {
    let diameter = self.radius * 2.0;
    Size {
      width: Dimension::Length(diameter),
      height: Dimension::Length(diameter),
    }
  }

  fn get_style(&self) -> Style {
    Style {
      display: Display::Flex,
      size: self.get_size(),
      ..Default::default()
    }
  }

  async fn render(
    &self,
    _generator: &ImageGenerator,
    canvas: &mut RgbaImage,
    layout: &TaffyRect,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Implementation for circle rendering
    let color = self.color.unwrap_or(Color {
      r: 0,
      g: 0,
      b: 0,
      a: 255,
    });
    let center = (
      (layout.location.x + layout.size.width / 2.0) as i32,
      (layout.location.y + layout.size.height / 2.0) as i32,
    );
    let radius = self.radius as i32;

    // Draw circle using midpoint circle algorithm
    let mut x = radius - 1;
    let mut y = 0;
    let mut dx = 1;
    let mut dy = 1;
    let mut err = dx - (radius << 1);

    while x >= y {
      for px in [center.0 + x, center.0 - x] {
        for py in [center.1 + y, center.1 - y] {
          if px >= 0 && py >= 0 && (px as u32) < canvas.width() && (py as u32) < canvas.height() {
            canvas.put_pixel(
              px as u32,
              py as u32,
              Rgba([color.r, color.g, color.b, color.a]),
            );
          }
        }
      }

      if x != y {
        for px in [center.0 + y, center.0 - y] {
          for py in [center.1 + x, center.1 - x] {
            if px >= 0 && py >= 0 && (px as u32) < canvas.width() && (py as u32) < canvas.height() {
              canvas.put_pixel(
                px as u32,
                py as u32,
                Rgba([color.r, color.g, color.b, color.a]),
              );
            }
          }
        }
      }

      if err <= 0 {
        y += 1;
        err += dy;
        dy += 2;
      }

      if err > 0 {
        x -= 1;
        dx += 2;
        err += dx - (radius << 1);
      }
    }

    Ok(())
  }

  fn get_style(&self) -> Style {
    Style::default()
  }

  async fn render(
    &self,
    canvas: &mut RgbaImage,
    _generator: &ImageGenerator,
    x: f32,
    y: f32,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use imageproc::drawing::draw_filled_circle_mut;

    if let Some(color) = &self.color {
      let center_x = (x + self.radius) as i32;
      let center_y = (y + self.radius) as i32;
      draw_filled_circle_mut(
        canvas,
        (center_x, center_y),
        self.radius as i32,
        (*color).into(),
      );
    }

    Ok(())
  }
}

#[async_trait]
impl NodeTrait for super::ImageNode {
  fn get_size(&self) -> Size<Dimension> {
    Size {
      width: self.width.map(Dimension::Points).unwrap_or(Dimension::Auto),
      height: self
        .height
        .map(Dimension::Points)
        .unwrap_or(Dimension::Auto),
    }
  }

  async fn render(
    &self,
    canvas: &mut RgbaImage,
    generator: &ImageGenerator,
    x: f32,
    y: f32,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Ok(image) = generator.load_image(&self.src).await {
      let resized = match (self.width, self.height) {
        (Some(w), Some(h)) => {
          image.resize_exact(w as u32, h as u32, image::imageops::FilterType::Lanczos3)
        }
        _ => image,
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

#[async_trait]
impl NodeTrait for super::SpaceNode {
  fn get_size(&self) -> Size<Dimension> {
    Size {
      width: self.width.map(Dimension::Length).unwrap_or(Dimension::Auto),
      height: self
        .height
        .map(Dimension::Length)
        .unwrap_or(Dimension::Auto),
    }
  }

  fn get_style(&self) -> Style {
    Style {
      display: Display::Flex,
      size: self.get_size(),
      ..Default::default()
    }
  }

  async fn render(
    &self,
    _generator: &ImageGenerator,
    _canvas: &mut RgbaImage,
    _layout: &TaffyRect,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Space nodes don't render anything
    Ok(())
  }

  fn get_style(&self) -> Style {
    Style::default()
  }

  async fn render(
    &self,
    _canvas: &mut RgbaImage,
    _generator: &ImageGenerator,
    _x: f32,
    _y: f32,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Space nodes don't render anything
    Ok(())
  }
}
