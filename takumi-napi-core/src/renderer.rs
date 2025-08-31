use napi::bindgen_prelude::*;
use napi_derive::napi;
use takumi::{
  GlobalContext,
  layout::Viewport,
  rendering::ImageOutputFormat,
  resources::{font::FontContext, image::load_image_source_from_bytes},
};

use crate::{
  load_font_task::LoadFontTask, put_persistent_image_task::PutPersistentImageTask,
  render_task::RenderTask,
};
use std::sync::Arc;

#[napi]
#[derive(Default)]
pub struct Renderer(Arc<GlobalContext>);

#[napi(object)]
pub struct RenderOptions {
  pub width: u32,
  pub height: u32,
  pub format: Option<OutputFormat>,
  pub quality: Option<u8>,
}

#[napi(string_enum)]
#[allow(non_camel_case_types)]
pub enum OutputFormat {
  webp,
  avif,
  png,
  jpeg,
  WebP,
  Avif,
  Jpeg,
  Png,
}

impl From<OutputFormat> for ImageOutputFormat {
  fn from(format: OutputFormat) -> Self {
    match format {
      OutputFormat::WebP => ImageOutputFormat::WebP,
      OutputFormat::Avif => ImageOutputFormat::Avif,
      OutputFormat::Jpeg => ImageOutputFormat::Jpeg,
      OutputFormat::Png => ImageOutputFormat::Png,
      OutputFormat::png => ImageOutputFormat::Png,
      OutputFormat::jpeg => ImageOutputFormat::Jpeg,
      OutputFormat::webp => ImageOutputFormat::WebP,
      OutputFormat::avif => ImageOutputFormat::Avif,
    }
  }
}

#[napi(object)]
pub struct PersistentImage<'ctx> {
  pub src: String,
  pub data: BufferSlice<'ctx>,
}

#[napi(object)]
#[derive(Default)]
pub struct ConstructRendererOptions<'ctx> {
  pub debug: Option<bool>,
  pub persistent_images: Option<Vec<PersistentImage<'ctx>>>,
  pub fonts: Option<Vec<BufferSlice<'ctx>>>,
  pub load_default_fonts: Option<bool>,
}

#[napi]
impl Renderer {
  #[napi(constructor)]
  pub fn new(options: Option<ConstructRendererOptions>) -> Self {
    let options = options.unwrap_or_default();

    let renderer = Self(Arc::new(GlobalContext {
      draw_debug_border: options.debug.unwrap_or_default(),
      font_context: FontContext::new(
        options
          .load_default_fonts
          .unwrap_or_else(|| options.fonts.is_none()),
      ),
      ..Default::default()
    }));

    if let Some(images) = options.persistent_images {
      for image in images {
        let image_source = load_image_source_from_bytes(&image.data).unwrap();

        renderer
          .0
          .persistent_image_store
          .insert(&image.src, image_source);
      }
    }

    if let Some(fonts) = options.fonts {
      for font in fonts {
        renderer.0.font_context.load_and_store(&font).unwrap();
      }
    }

    renderer
  }

  #[napi]
  pub fn purge_font_cache(&self) {
    self.0.font_context.purge_cache();
  }

  #[napi(ts_return_type = "Promise<void>")]
  pub fn put_persistent_image_async(
    &self,
    src: String,
    data: Buffer,
    signal: Option<AbortSignal>,
  ) -> AsyncTask<PutPersistentImageTask> {
    AsyncTask::with_optional_signal(
      PutPersistentImageTask {
        src: Some(src),
        context: Arc::clone(&self.0),
        buffer: data,
      },
      signal,
    )
  }

  #[napi(ts_return_type = "Promise<number>")]
  pub fn load_font_async(
    &self,
    data: Buffer,
    signal: Option<AbortSignal>,
  ) -> AsyncTask<LoadFontTask> {
    AsyncTask::with_optional_signal(
      LoadFontTask {
        context: Arc::clone(&self.0),
        buffers: vec![data],
      },
      signal,
    )
  }

  #[napi(ts_return_type = "Promise<number>")]
  pub fn load_fonts_async(
    &self,
    fonts: Vec<Buffer>,
    signal: Option<AbortSignal>,
  ) -> AsyncTask<LoadFontTask> {
    AsyncTask::with_optional_signal(
      LoadFontTask {
        context: Arc::clone(&self.0),
        buffers: fonts,
      },
      signal,
    )
  }

  #[napi]
  pub fn clear_image_store(&self) {
    self.0.persistent_image_store.clear();
  }

  #[napi(
    ts_args_type = "source: { type: string }, options: RenderOptions, signal?: AbortSignal",
    ts_return_type = "Promise<Buffer>"
  )]
  pub fn render_async(
    &self,
    env: Env,
    source: Object,
    options: RenderOptions,
    signal: Option<AbortSignal>,
  ) -> Result<AsyncTask<RenderTask>> {
    let node = env.from_js_value(source)?;

    Ok(AsyncTask::with_optional_signal(
      RenderTask {
        node: Some(node),
        context: Arc::clone(&self.0),
        viewport: Viewport::new(options.width, options.height),
        format: options.format.unwrap_or(OutputFormat::png).into(),
        quality: options.quality,
      },
      signal,
    ))
  }
}
