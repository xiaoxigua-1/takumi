use criterion::{BatchSize, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use takumi::{
  GlobalContext,
  layout::Viewport,
  layout::style::{
    Angle, BackgroundImage, Color, GradientStop, LengthUnit, LinearGradient, StopPosition,
  },
  rendering::{RenderContext, render_gradient_tile},
};

fn bench_sizes() -> Vec<(u32, u32)> {
  vec![(256, 256), (512, 512), (1200, 630), (1920, 1080)]
}

fn sample_gradient() -> LinearGradient {
  LinearGradient {
    angle: Angle::new(45.0),
    stops: vec![
      GradientStop::ColorHint {
        color: Color([255, 0, 0, 255]),
        hint: Some(StopPosition(LengthUnit::Percentage(0.0))),
      },
      GradientStop::ColorHint {
        color: Color([0, 255, 0, 255]),
        hint: Some(StopPosition(LengthUnit::Percentage(50.0))),
      },
      GradientStop::ColorHint {
        color: Color([0, 0, 255, 255]),
        hint: Some(StopPosition(LengthUnit::Percentage(100.0))),
      },
    ],
  }
}

fn bench_takumi(c: &mut Criterion) {
  let mut group = c.benchmark_group("linear_gradient");
  let global = GlobalContext::default();

  for (w, h) in bench_sizes() {
    group.throughput(Throughput::Bytes((w as u64) * (h as u64) * 4));
    group.bench_with_input(
      BenchmarkId::new("takumi", format!("{}x{}", w, h)),
      &(w, h),
      |b, &(w, h)| {
        b.iter_batched(
          || {
            let viewport = Viewport::new(w, h);
            let context = RenderContext {
              global: &global,
              viewport,
              parent_font_size: viewport.font_size,
            };
            (sample_gradient(), w, h, context)
          },
          |(gradient, w, h, context)| {
            render_gradient_tile(&BackgroundImage::Linear(gradient), w, h, &context);
          },
          BatchSize::SmallInput,
        )
      },
    );
  }

  group.finish();
}

fn bench_tiny_skia(c: &mut Criterion) {
  use tiny_skia::GradientStop as SkStop;
  use tiny_skia::{
    Color as SkColor, LinearGradient as SkLinearGradient, Paint, Pixmap, Point, SpreadMode,
    Transform,
  };

  let mut group = c.benchmark_group("linear_gradient");

  for (w, h) in bench_sizes() {
    group.throughput(Throughput::Bytes((w as u64) * (h as u64) * 4));
    group.bench_with_input(
      BenchmarkId::new("tiny_skia", format!("{}x{}", w, h)),
      &(w, h),
      |b, &(w, h)| {
        b.iter_batched(
          || {
            let pixmap = Pixmap::new(w, h).unwrap();
            let paint = {
              let stops = vec![
                SkStop::new(0.0, SkColor::from_rgba8(255, 0, 0, 255)),
                SkStop::new(0.5, SkColor::from_rgba8(0, 255, 0, 255)),
                SkStop::new(1.0, SkColor::from_rgba8(0, 0, 255, 255)),
              ];
              let start = Point::from_xy(0.0, 0.0);
              let end = Point::from_xy(w as f32, h as f32);
              let shader =
                SkLinearGradient::new(start, end, stops, SpreadMode::Pad, Transform::identity())
                  .unwrap();

              Paint {
                shader,
                ..Default::default()
              }
            };
            (pixmap, paint)
          },
          |(mut pixmap, paint)| {
            let rect = tiny_skia::Rect::from_xywh(0.0, 0.0, w as f32, h as f32).unwrap();
            pixmap.fill_rect(rect, &paint, Transform::identity(), None);
            std::hint::black_box(pixmap);
          },
          BatchSize::SmallInput,
        )
      },
    );
  }

  group.finish();
}

pub fn benches(c: &mut Criterion) {
  bench_takumi(c);
  bench_tiny_skia(c);
}

criterion_group!(name = gradient_benches; config = Criterion::default(); targets = benches);
criterion_main!(gradient_benches);
