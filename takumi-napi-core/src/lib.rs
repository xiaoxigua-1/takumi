use std::io::Cursor;

use napi::bindgen_prelude::*;
use napi_derive::napi;
use rayon::prelude::*;
use takumi::{
  context::GlobalContext,
  image::ImageFormat,
  node::{DefaultNodeKind, Node},
  render::{ImageRenderer, Viewport},
};

#[napi]
#[derive(Default)]
pub struct Renderer(GlobalContext);

pub struct RenderTask<'ctx> {
  pub node: Option<DefaultNodeKind>,
  pub context: &'ctx GlobalContext,
  pub viewport: Viewport,
}

impl<'ctx> Task for RenderTask<'ctx> {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let mut node = self.node.take().unwrap();

    node.inherit_style_for_children();
    node.hydrate(self.context);

    let mut render = ImageRenderer::new(self.viewport);

    render.construct_taffy_tree(node, self.context);
    render
      .draw(self.context)
      .map_err(|err| napi::Error::from_reason(format!("{err:?}")))?;

    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);

    let image = render.draw(self.context).unwrap();

    image.write_to(&mut cursor, ImageFormat::WebP).unwrap();

    Ok(buffer)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.into())
  }
}

pub struct PreloadImageTask<'ctx> {
  pub context: &'ctx GlobalContext,
  pub url: Option<String>,
}

impl<'ctx> Task for PreloadImageTask<'ctx> {
  type Output = bool;
  type JsValue = bool;

  fn compute(&mut self) -> Result<Self::Output> {
    let url = self.url.take().unwrap();

    let state = self.context.image_store.fetch(&url);
    let is_fetched = state.is_fetched();

    self.context.image_store.insert(url, state);

    Ok(is_fetched)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

pub struct LoadFontTask<'ctx> {
  pub context: &'ctx GlobalContext,
  pub buffers: Vec<Vec<u8>>,
}

impl<'ctx> Task for LoadFontTask<'ctx> {
  type Output = usize;
  type JsValue = u32;

  fn compute(&mut self) -> Result<Self::Output> {
    let buffers = std::mem::take(&mut self.buffers);
    
    let loaded_count = buffers
      .into_par_iter()
      .map(|buffer| {
        self.context.font_context.load_font(buffer).is_ok() as usize
      })
      .sum();

    Ok(loaded_count)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output as u32)
  }
}

#[napi]
impl Renderer {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self::default()
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
