use clap::Parser;
use mimalloc::MiMalloc;
use takumi::GlobalContext;
use tracing::Level;
use tracing_subscriber::fmt;

use takumi_server::{Args, run_server};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() {
  fmt().with_max_level(Level::INFO).init();

  let args = Args::parse();

  let context = GlobalContext {
    draw_debug_border: args.draw_debug_border,
    ..Default::default()
  };

  run_server(args, context).await;
}
