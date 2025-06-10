use std::io::Cursor;

use criterion::{Criterion, criterion_group, criterion_main};
use image::{ImageFormat, RgbaImage};
use imagen::{
  color::Color,
  context::Context,
  node::{
    ContainerNode, Node, TextNode,
    style::{FontWeight, InheritableStyle, Length, Style, TextAlign, ValueOrAutoFull},
  },
  render::ImageRenderer,
};
use imageproc::drawing::Blend;
use taffy::{Layout, Point, Rect, Size, prelude::FromLength};

fn render_scenarios(c: &mut Criterion) {
  let mut group = c.benchmark_group("render_scenarios");

  // Empty container baseline
  group.bench_function("empty_container", |b| {
    b.iter(|| {
      let context = Context::default();
      let renderer = ImageRenderer::try_from(ContainerNode {
        style: Style {
          width: ValueOrAutoFull::SpecificValue(Length(1200.0)),
          height: ValueOrAutoFull::SpecificValue(Length(630.0)),
          ..Default::default()
        },
        children: vec![],
      })
      .unwrap();
      let (mut taffy, root_node_id) = renderer.create_taffy_tree();
      let mut buffer = Vec::new();
      let mut cursor = Cursor::new(&mut buffer);
      let image = renderer.draw(&context, &mut taffy, root_node_id);
      image.write_to(&mut cursor, ImageFormat::WebP).unwrap();
    })
  });

  // Text rendering
  group.bench_function("text_rendering", |b| {
    let node = TextNode {
      style: Style {
        inheritable_style: InheritableStyle {
          font_size: Some(48.0),
          color: Some(Color::Rgb(0, 0, 0)),
          text_align: Some(TextAlign::Center),
          font_weight: Some(FontWeight(700)),
          ..Default::default()
        },
        ..Default::default()
      },
      text: "Hello, World! This is a benchmark test for text rendering.".to_string(),
    };

    let layout = Layout {
      order: 0,
      location: Point::from_length(0.0),
      size: Size {
        width: 800.0,
        height: 600.0,
      },
      content_size: Size {
        width: 800.0,
        height: 600.0,
      },
      scrollbar_size: Size {
        width: 0.0,
        height: 0.0,
      },
      border: Rect::default(),
      padding: Rect::default(),
      margin: Rect::default(),
    };
    let context = Context::default();
    let mut canvas = Blend(RgbaImage::new(800, 600));

    b.iter(|| {
      node.draw_on_canvas(&context, &mut canvas, layout);
    })
  });

  // Nested containers
  group.bench_function("nested_containers", |b| {
    b.iter(|| {
      let context = Context::default();
      let renderer = ImageRenderer::try_from(ContainerNode {
        style: Style {
          width: ValueOrAutoFull::SpecificValue(Length(1200.0)),
          height: ValueOrAutoFull::SpecificValue(Length(630.0)),
          background: Some(Color::Rgb(240, 240, 240).into()),
          ..Default::default()
        },
        children: vec![Box::new(ContainerNode {
          style: Style {
            width: ValueOrAutoFull::SpecificValue(Length(400.0)),
            height: ValueOrAutoFull::SpecificValue(Length(400.0)),
            background: Some(Color::Rgb(200, 200, 200).into()),
            ..Default::default()
          },
          children: vec![Box::new(ContainerNode {
            style: Style {
              width: ValueOrAutoFull::SpecificValue(Length(200.0)),
              height: ValueOrAutoFull::SpecificValue(Length(200.0)),
              background: Some(Color::Rgb(160, 160, 160).into()),
              ..Default::default()
            },
            children: vec![],
          })],
        })],
      })
      .unwrap();
      let (mut taffy, root_node_id) = renderer.create_taffy_tree();
      let mut buffer = Vec::new();
      let mut cursor = Cursor::new(&mut buffer);
      let image = renderer.draw(&context, &mut taffy, root_node_id);
      image.write_to(&mut cursor, ImageFormat::WebP).unwrap();
    })
  });

  group.finish();
}

fn render_performance_analysis(c: &mut Criterion) {
  let mut group = c.benchmark_group("performance_analysis");

  // Complex layout for detailed analysis
  let complex_layout = || ContainerNode {
    style: Style {
      width: ValueOrAutoFull::SpecificValue(Length(1200.0)),
      height: ValueOrAutoFull::SpecificValue(Length(630.0)),
      background: Some(Color::Rgb(240, 240, 240).into()),
      ..Default::default()
    },
    children: vec![
      Box::new(TextNode {
        style: Style {
          width: ValueOrAutoFull::SpecificValue(Length(800.0)),
          height: ValueOrAutoFull::SpecificValue(Length(100.0)),
          inheritable_style: InheritableStyle {
            font_size: Some(48.0),
            color: Some(Color::Rgb(0, 0, 0)),
            text_align: Some(TextAlign::Center),
            font_weight: Some(FontWeight(700)),
            ..Default::default()
          },
          ..Default::default()
        },
        text: "Complex Layout Benchmark".to_string(),
      }),
      Box::new(ContainerNode {
        style: Style {
          width: ValueOrAutoFull::SpecificValue(Length(1000.0)),
          height: ValueOrAutoFull::SpecificValue(Length(400.0)),
          background: Some(Color::Rgb(200, 200, 200).into()),
          ..Default::default()
        },
        children: vec![
          Box::new(TextNode {
            style: Style {
              width: ValueOrAutoFull::SpecificValue(Length(400.0)),
              height: ValueOrAutoFull::SpecificValue(Length(100.0)),
              inheritable_style: InheritableStyle {
                font_size: Some(24.0),
                color: Some(Color::Rgb(50, 50, 50)),
                text_align: Some(TextAlign::Left),
                ..Default::default()
              },
              ..Default::default()
            },
            text: "This is a complex layout with multiple nested elements and text nodes."
              .to_string(),
          }),
          Box::new(ContainerNode {
            style: Style {
              width: ValueOrAutoFull::SpecificValue(Length(200.0)),
              height: ValueOrAutoFull::SpecificValue(Length(200.0)),
              background: Some(Color::Rgb(160, 160, 160).into()),
              ..Default::default()
            },
            children: vec![],
          }),
        ],
      }),
    ],
  };

  // Full pipeline benchmark
  group.bench_function("full_pipeline", |b| {
    b.iter(|| {
      let context = Context::default();
      let renderer = ImageRenderer::try_from(complex_layout()).unwrap();
      let (mut taffy, root_node_id) = renderer.create_taffy_tree();
      let mut buffer = Vec::new();
      let mut cursor = Cursor::new(&mut buffer);
      let image = renderer.draw(&context, &mut taffy, root_node_id);
      image.write_to(&mut cursor, ImageFormat::WebP).unwrap();
    })
  });

  // Tree creation step
  group.bench_function("tree_creation", |b| {
    b.iter(|| {
      let renderer = ImageRenderer::try_from(complex_layout()).unwrap();
      renderer.create_taffy_tree()
    })
  });

  // Drawing step
  group.bench_function("image_drawing", |b| {
    let context = Context::default();
    let renderer = ImageRenderer::try_from(complex_layout()).unwrap();
    let (mut taffy, root_node_id) = renderer.create_taffy_tree();

    b.iter(|| renderer.draw(&context, &mut taffy, root_node_id))
  });

  // Encoding step
  group.bench_function("webp_encoding", |b| {
    let context = Context::default();
    let renderer = ImageRenderer::try_from(complex_layout()).unwrap();
    let (mut taffy, root_node_id) = renderer.create_taffy_tree();
    let image = renderer.draw(&context, &mut taffy, root_node_id);

    b.iter(|| {
      let mut buffer = Vec::new();
      let mut cursor = Cursor::new(&mut buffer);
      image.write_to(&mut cursor, ImageFormat::WebP).unwrap();
    })
  });

  group.finish();
}

criterion_group!(benches, render_scenarios, render_performance_analysis);
criterion_main!(benches);
