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
  /// Example: `--port 8080` or `-p 8080`
  #[arg(short, long, default_value_t = 3000)]
  pub port: u16,

  /// Enables printing of the debug tree structure.
  ///
  /// When enabled, the server will print the node tree structure
  /// during image generation for debugging purposes.
  /// Example: `--print-debug-tree` or `-p`
  #[arg(short, long, default_value_t = false)]
  pub print_debug_tree: bool,

  /// Enables drawing of debug borders around elements.
  ///
  /// When enabled, the server will draw borders around all elements
  /// in the generated image to help with layout debugging.
  /// Example: `--draw-debug-border` or `-d`
  #[arg(short, long, default_value_t = false)]
  pub draw_debug_border: bool,

  /// List of font files to load into the server.
  ///
  /// These should be valid WOFF2 font files that will be loaded
  /// and made available for use in image generation.
  /// Example: `--fonts path/to/font1.woff2 path/to/font2.woff2` or `-f font1.woff2 -f font2.woff2`
  #[arg(short, long, value_parser)]
  pub fonts: Vec<String>,
}
