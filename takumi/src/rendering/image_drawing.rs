use image::imageops::crop_imm;
use image::{RgbaImage, imageops::FilterType};
use taffy::Layout;

use crate::layout::style::{ObjectFit, Style};
use crate::rendering::{BorderRadius, FastBlendImage, RenderContext};
use crate::resources::image::ImageSource;

/// Process an image according to the specified object-fit style.
///
/// This function handles resizing, cropping, and positioning of images
/// based on the ObjectFit property, returning the processed image and offset.
pub fn process_image_for_object_fit(
  image: &ImageSource,
  object_fit: ObjectFit,
  filter_type: FilterType,
  container_width: u32,
  container_height: u32,
) -> (RgbaImage, u32, u32) {
  let (image_width, image_height) = image.size();
  let image_width = image_width as u32;
  let image_height = image_height as u32;

  match object_fit {
    ObjectFit::Fill => (
      image.render_to_rgba_image(container_width, container_height, filter_type),
      0,
      0,
    ),
    ObjectFit::Contain => {
      let scale_x = container_width as f32 / image_width as f32;
      let scale_y = container_height as f32 / image_height as f32;
      let scale = scale_x.min(scale_y);

      let new_width = (image_width as f32 * scale) as u32;
      let new_height = (image_height as f32 * scale) as u32;

      let offset_x = container_width.saturating_sub(new_width) / 2;
      let offset_y = container_height.saturating_sub(new_height) / 2;

      (
        image.render_to_rgba_image(new_width, new_height, filter_type),
        offset_x,
        offset_y,
      )
    }
    ObjectFit::Cover => {
      let scale_x = container_width as f32 / image_width as f32;
      let scale_y = container_height as f32 / image_height as f32;
      let scale = scale_x.max(scale_y);

      let new_width = (image_width as f32 * scale) as u32;
      let new_height = (image_height as f32 * scale) as u32;

      let resized = image.render_to_rgba_image(new_width, new_height, filter_type);

      let crop_x = new_width.saturating_sub(container_width) / 2;
      let crop_y = new_height.saturating_sub(container_height) / 2;

      let cropped =
        crop_imm(&resized, crop_x, crop_y, container_width, container_height).to_image();

      (cropped, 0, 0)
    }
    ObjectFit::ScaleDown => {
      let scale_x = container_width as f32 / image_width as f32;
      let scale_y = container_height as f32 / image_height as f32;
      let scale = scale_x.min(scale_y).min(1.0);

      let new_width = (image_width as f32 * scale) as u32;
      let new_height = (image_height as f32 * scale) as u32;

      let processed_image = if scale < 1.0 {
        image.render_to_rgba_image(new_width, new_height, filter_type)
      } else {
        image.render_to_rgba_image(image_width, image_height, filter_type)
      };

      let offset_x = container_width.saturating_sub(new_width) / 2;
      let offset_y = container_height.saturating_sub(new_height) / 2;

      (processed_image, offset_x, offset_y)
    }
    ObjectFit::None => {
      if image_width <= container_width && image_height <= container_height {
        let offset_x = (container_width - image_width) / 2;
        let offset_y = (container_height - image_height) / 2;
        (
          image.render_to_rgba_image(image_width, image_height, filter_type),
          offset_x,
          offset_y,
        )
      } else {
        let crop_x = if image_width > container_width {
          (image_width - container_width) / 2
        } else {
          0
        };
        let crop_y = if image_height > container_height {
          (image_height - container_height) / 2
        } else {
          0
        };

        let crop_width = container_width.min(image_width);
        let crop_height = container_height.min(image_height);

        let source_image = image.render_to_rgba_image(image_width, image_height, filter_type);
        let cropped = crop_imm(&source_image, crop_x, crop_y, crop_width, crop_height).to_image();

        let offset_x = if crop_width < container_width {
          (container_width - crop_width) / 2
        } else {
          0
        };
        let offset_y = if crop_height < container_height {
          (container_height - crop_height) / 2
        } else {
          0
        };

        (cropped, offset_x, offset_y)
      }
    }
  }
}

/// Draws an image on the canvas with the specified style and layout.
///
/// The image will be resized and positioned according to the object_fit style property.
/// Border radius will be applied if specified in the style.
pub fn draw_image(
  image: &ImageSource,
  style: &Style,
  context: &RenderContext,
  canvas: &mut FastBlendImage,
  layout: Layout,
) {
  let content_box = layout.content_box_size();
  let x = layout.content_box_x();
  let y = layout.content_box_y();

  let container_width = content_box.width as u32;
  let container_height = content_box.height as u32;

  let (mut image, offset_x, offset_y) = process_image_for_object_fit(
    image,
    style.object_fit,
    style
      .inheritable_style
      .image_rendering
      .unwrap_or_default()
      .into(),
    container_width,
    container_height,
  );

  // Apply border radius if specified
  if let Some(border_radius) = style.inheritable_style.resolved_border_radius() {
    let radius = BorderRadius::from_layout(context, &layout, border_radius.into());
    radius.apply_to_image(&mut image);
  }

  canvas.overlay_image(&image, offset_x + x as u32, offset_y + y as u32);
}
