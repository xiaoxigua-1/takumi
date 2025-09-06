use std::borrow::Cow;
use std::sync::Arc;

use image::imageops::crop_imm;
use image::{RgbaImage, imageops::FilterType};
use taffy::{Layout, Point, Size};

use crate::layout::style::{Affine, ObjectFit, Style};
use crate::rendering::{Canvas, RenderContext};
use crate::resources::image::ImageSource;

/// Process an image according to the specified object-fit style.
///
/// This function handles resizing, cropping, and positioning of images
/// based on the ObjectFit property, returning the processed image and offset.
pub fn process_image_for_object_fit<'i>(
  image: &'i ImageSource,
  object_fit: ObjectFit,
  filter_type: FilterType,
  container_width: f32,
  container_height: f32,
) -> (Cow<'i, RgbaImage>, f32, f32) {
  let (image_width, image_height) = image.size();

  match object_fit {
    ObjectFit::Fill => (
      image.render_to_rgba_image(container_width as u32, container_height as u32, filter_type),
      0.0,
      0.0,
    ),
    ObjectFit::Contain => {
      let scale_x = container_width / image_width;
      let scale_y = container_height / image_height;
      let scale = scale_x.min(scale_y);

      let new_width = image_width * scale;
      let new_height = image_height * scale;

      let offset_x = (container_width - new_width) / 2.0;
      let offset_y = (container_height - new_height) / 2.0;

      (
        image.render_to_rgba_image(new_width as u32, new_height as u32, filter_type),
        offset_x,
        offset_y,
      )
    }
    ObjectFit::Cover => {
      let scale_x = container_width / image_width;
      let scale_y = container_height / image_height;
      let scale = scale_x.max(scale_y);

      let new_width = image_width * scale;
      let new_height = image_height * scale;

      let resized = image.render_to_rgba_image(new_width as u32, new_height as u32, filter_type);

      let crop_x = (new_width - container_width) / 2.0;
      let crop_y = (new_height - container_height) / 2.0;

      let cropped = crop_imm(
        resized.as_ref(),
        crop_x as u32,
        crop_y as u32,
        container_width as u32,
        container_height as u32,
      )
      .to_image();

      (Cow::Owned(cropped), 0.0, 0.0)
    }
    ObjectFit::ScaleDown => {
      let scale_x = container_width / image_width;
      let scale_y = container_height / image_height;
      let scale = scale_x.min(scale_y).min(1.0);

      let new_width = image_width * scale;
      let new_height = image_height * scale;

      let processed_image = if scale < 1.0 {
        image.render_to_rgba_image(new_width as u32, new_height as u32, filter_type)
      } else {
        image.render_to_rgba_image(image_width as u32, image_height as u32, filter_type)
      };

      let offset_x = (container_width - new_width) / 2.0;
      let offset_y = (container_height - new_height) / 2.0;

      (processed_image, offset_x, offset_y)
    }
    ObjectFit::None => {
      if image_width <= container_width && image_height <= container_height {
        let offset_x = (container_width - image_width) / 2.0;
        let offset_y = (container_height - image_height) / 2.0;
        (
          image.render_to_rgba_image(image_width as u32, image_height as u32, filter_type),
          offset_x,
          offset_y,
        )
      } else {
        let crop_x = if image_width > container_width {
          (image_width - container_width) / 2.0
        } else {
          0.0
        };
        let crop_y = if image_height > container_height {
          (image_height - container_height) / 2.0
        } else {
          0.0
        };

        let crop_width = container_width.min(image_width);
        let crop_height = container_height.min(image_height);

        let source_image =
          image.render_to_rgba_image(image_width as u32, image_height as u32, filter_type);
        let cropped = crop_imm(
          source_image.as_ref(),
          crop_x as u32,
          crop_y as u32,
          crop_width as u32,
          crop_height as u32,
        )
        .to_image();

        let offset_x = if crop_width < container_width {
          (container_width - crop_width) / 2.0
        } else {
          0.0
        };
        let offset_y = if crop_height < container_height {
          (container_height - crop_height) / 2.0
        } else {
          0.0
        };

        (Cow::Owned(cropped), offset_x, offset_y)
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
  canvas: &Canvas,
  layout: Layout,
) {
  let content_box = layout.content_box_size();

  let (image, offset_x, offset_y) = process_image_for_object_fit(
    image,
    style.object_fit,
    style
      .inheritable_style
      .image_rendering
      .unwrap_or_default()
      .into(),
    content_box.width,
    content_box.height,
  );

  // manually apply the border and padding to ensure rotation with origin is applied correctly
  let transform_offset_x = layout.border.left + layout.padding.left;
  let transform_offset_y = layout.border.top + layout.padding.top;

  let transform_with_content_offset = Affine::translation(Size {
    width: transform_offset_x,
    height: transform_offset_y,
  }) * context.transform;

  canvas.overlay_image(
    Arc::new(image.into_owned()),
    Point {
      x: offset_x as i32 + layout.location.x as i32,
      y: offset_y as i32 + layout.location.y as i32,
    },
    style
      .create_border_radius(&layout, context)
      .inset_by_border_width(),
    transform_with_content_offset,
    style.inheritable_style.image_rendering.unwrap_or_default(),
  );
}
