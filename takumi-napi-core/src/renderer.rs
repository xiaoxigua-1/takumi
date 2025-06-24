use napi::bindgen_prelude::*;
use napi_derive::napi;
use takumi::{GlobalContext, ImageStore, Viewport};

use crate::{
  load_font_task::LoadFontTask, load_local_image_task::LoadLocalImageTask,
  preload_image_task::PreloadImageTask, render_task::RenderTask,
};

#[napi]
#[derive(Default)]
pub struct Renderer(GlobalContext);

#[napi]
impl Renderer {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self::default()
  }

  #[napi(ts_return_type = "Promise<void>")]
  pub fn load_local_image_async(
    &self,
    key: String,
    data: ArrayBuffer,
    signal: Option<AbortSignal>,
  ) -> AsyncTask<LoadLocalImageTask> {
    AsyncTask::with_optional_signal(
      LoadLocalImageTask {
        key: Some(key),
        context: &self.0,
        buffer: data.to_vec(),
      },
      signal,
    )
  }

  #[napi(ts_return_type = "Promise<number>")]
  pub fn load_font_async(
    &self,
    data: ArrayBuffer,
    signal: Option<AbortSignal>,
  ) -> AsyncTask<LoadFontTask> {
    AsyncTask::with_optional_signal(
      LoadFontTask {
        context: &self.0,
        buffers: vec![data.to_vec()],
      },
      signal,
    )
  }

  #[napi(ts_return_type = "Promise<number>")]
  pub fn load_fonts_async(
    &self,
    fonts: Vec<ArrayBuffer>,
    signal: Option<AbortSignal>,
  ) -> AsyncTask<LoadFontTask> {
    AsyncTask::with_optional_signal(
      LoadFontTask {
        context: &self.0,
        buffers: fonts.into_iter().map(|buf| buf.to_vec()).collect(),
      },
      signal,
    )
  }

  #[napi]
  pub fn clear_image_store(&self) {
    self.0.image_store.clear();
  }

  #[napi]
  pub fn clear_local_image_store(&self) {
    self.0.local_image_store.clear();
  }

  #[napi(ts_return_type = "Promise<void>")]
  pub fn preload_image_async(
    &self,
    url: String,
    signal: Option<AbortSignal>,
  ) -> AsyncTask<PreloadImageTask> {
    AsyncTask::with_optional_signal(
      PreloadImageTask {
        context: &self.0,
        url: Some(url),
      },
      signal,
    )
  }

  #[napi(
    ts_args_type = "source: { type: string }, width: number, height: number, signal?: AbortSignal",
    ts_return_type = "Promise<Buffer>"
  )]
  pub fn render_async(
    &self,
    env: &Env,
    source: Object,
    width: u32,
    height: u32,
    signal: Option<AbortSignal>,
  ) -> Result<AsyncTask<RenderTask>> {
    let node = env.from_js_value(source)?;

    Ok(AsyncTask::with_optional_signal(
      RenderTask {
        node: Some(node),
        context: &self.0,
        viewport: Viewport::new(width, height),
      },
      signal,
    ))
  }
}
