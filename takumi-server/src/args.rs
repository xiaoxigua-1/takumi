use clap::{Parser, command};

/// Command line arguments for the image generator server.
///
/// This struct defines the configuration options that can be passed to the server
/// when starting it up. It uses the `clap` derive macro to automatically generate
/// command line argument parsing.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
  /// The port number on which the server will listen.
  ///
  /// Defaults to 3000 if not specified.
  #[arg(short, long, default_value_t = 3000)]
  pub port: u16,

  /// Enables printing of the debug tree structure.
  ///
  /// When enabled, the server will print the node tree structure
  /// during image generation for debugging purposes.
  #[arg(short, long, default_value_t = false)]
  pub print_debug_tree: bool,

  /// Enables drawing of debug borders around elements.
  ///
  /// When enabled, the server will draw borders around all elements
  /// in the generated image to help with layout debugging.
  #[arg(short, long, default_value_t = false)]
  pub draw_debug_border: bool,

  /// Glob pattern of font files to load into the server.
  #[arg(short, long)]
  pub font_glob: Option<String>,

  /// The HMAC key for integrity checking. Can be any valid UTF-8 string.
  #[cfg_attr(feature = "integrity", arg(long))]
  #[cfg(feature = "integrity")]
  pub hmac_key: Option<String>,
}
