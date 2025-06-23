use image::{
  RgbaImage,
  imageops::{FilterType, crop_imm, resize},
};
use taffy::Layout;

use crate::{
  core::RenderContext,
  effects::{BorderRadius, apply_border_radius_antialiased},
  rendering::FastBlendImage,
  style::{ObjectFit, Style},
};

/// Draws an image on the canvas with the specified style and layout.
///
/// The image will be resized and positioned according to the object_fit style property.
/// Border radius will be applied if specified in the style.
pub fn draw_image(
  image: &RgbaImage,
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
  let image_width = image.width();
  let image_height = image.height();

  let (mut processed_image, offset_x, offset_y) = match style.object_fit {
    ObjectFit::Fill => {
      // Fill: stretch the image to fill the container exactly
      let resized = resize(
        image,
        container_width,
        container_height,
        FilterType::Lanczos3,
      );
      (resized, 0, 0)
    }
    ObjectFit::Contain => {
      // Contain: scale the image to fit within the container while preserving aspect ratio
      let scale_x = container_width as f32 / image_width as f32;
      let scale_y = container_height as f32 / image_height as f32;
      let scale = scale_x.min(scale_y);

      let new_width = (image_width as f32 * scale) as u32;
      let new_height = (image_height as f32 * scale) as u32;

      let resized = resize(image, new_width, new_height, FilterType::Lanczos3);
      let offset_x = (container_width.saturating_sub(new_width)) / 2;
      let offset_y = (container_height.saturating_sub(new_height)) / 2;

      (resized, offset_x, offset_y)
    }
    ObjectFit::Cover => {
      // Cover: scale the image to cover the entire container while preserving aspect ratio
      let scale_x = container_width as f32 / image_width as f32;
      let scale_y = container_height as f32 / image_height as f32;
      let scale = scale_x.max(scale_y);

      let new_width = (image_width as f32 * scale) as u32;
      let new_height = (image_height as f32 * scale) as u32;

      let resized = resize(image, new_width, new_height, FilterType::Lanczos3);

      // Crop to fit container
      let crop_x = (new_width.saturating_sub(container_width)) / 2;
      let crop_y = (new_height.saturating_sub(container_height)) / 2;

      let cropped =
        crop_imm(&resized, crop_x, crop_y, container_width, container_height).to_image();
      (cropped, 0, 0)
    }
    ObjectFit::ScaleDown => {
      // ScaleDown: same as contain, but never scale up
      let scale_x = container_width as f32 / image_width as f32;
      let scale_y = container_height as f32 / image_height as f32;
      let scale = scale_x.min(scale_y).min(1.0); // Never scale up

      let new_width = (image_width as f32 * scale) as u32;
      let new_height = (image_height as f32 * scale) as u32;

      let resized = if scale < 1.0 {
        resize(image, new_width, new_height, FilterType::Lanczos3)
      } else {
        image.clone()
      };

      let offset_x = (container_width.saturating_sub(new_width)) / 2;
      let offset_y = (container_height.saturating_sub(new_height)) / 2;

      (resized, offset_x, offset_y)
    }
    ObjectFit::None => {
      // None: display the image at its natural size, centered, but crop if too large
      if image_width <= container_width && image_height <= container_height {
        // Image fits within container, center it
        let offset_x = (container_width - image_width) / 2;
        let offset_y = (container_height - image_height) / 2;
        (image.clone(), offset_x, offset_y)
      } else {
        // Image is larger than container, crop from center
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

        let cropped = crop_imm(image, crop_x, crop_y, crop_width, crop_height).to_image();

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
  };

  // Apply border radius if specified
  if let Some(border_radius) = style.inheritable_style.border_radius {
    apply_border_radius_antialiased(
      &mut processed_image,
      BorderRadius::from_layout(context, &layout, border_radius.into()),
    );
  }

  canvas.overlay_image(&processed_image, offset_x + x as u32, offset_y + y as u32);
}
