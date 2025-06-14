use std::io::Cursor;

use criterion::{Criterion, criterion_group, criterion_main};
use image::{ImageFormat, RgbaImage};
use taffy::{Layout, Point, Rect, Size, prelude::FromLength};
use takumi::{
  color::{Color, ColorInput, Gradient},
  context::Context,
  node::{
    ContainerNode, Node, TextNode,
    draw::FastBlendImage,
    style::{FontWeight, InheritableStyle, Style, TextAlign},
  },
  render::ImageRenderer,
};

#[derive(Debug, Clone)]
enum NodeKind {
  Container(ContainerNode<NodeKind>),
  Text(TextNode),
}

impl Node<NodeKind> for NodeKind {
  fn get_style(&self) -> &Style {
    match self {
      NodeKind::Container(container) => &container.style,
      NodeKind::Text(text) => &text.style,
    }
  }

  fn get_style_mut(&mut self) -> &mut Style {
    match self {
      NodeKind::Container(container) => &mut container.style,
      NodeKind::Text(text) => &mut text.style,
    }
  }

  fn get_children(&self) -> Option<Vec<&Self>> {
    match self {
      NodeKind::Container(container) => container.get_children(),
      NodeKind::Text(text) => text.get_children(),
    }
  }
}

type ImageRendererNodes = ImageRenderer<NodeKind>;

fn render_scenarios(c: &mut Criterion) {
  let mut group = c.benchmark_group("render_scenarios");

  group.bench_function("gradient_background", |b| {
    b.iter(|| {
      let context = Context::default();
      let mut renderer = ImageRendererNodes::new(1200, 630);
      renderer.construct_taffy_tree(NodeKind::Container(ContainerNode {
        style: Style {
          width: 1200.0.into(),
          height: 630.0.into(),
          background_color: Some(ColorInput::Gradient(Gradient {
            stops: vec![Color::Rgb(242, 102, 223), Color::Rgb(157, 201, 106)],
            angle: 45.0,
          })),
          ..Default::default()
        },
        children: vec![],
      }));
      let image = renderer.draw(&context).unwrap();
      let mut buffer = Vec::new();
      let mut cursor = Cursor::new(&mut buffer);
      image.write_to(&mut cursor, ImageFormat::WebP).unwrap();
    })
  });

  // Empty container baseline
  group.bench_function("empty_container", |b| {
    b.iter(|| {
      let context = Context::default();
      let mut renderer = ImageRendererNodes::new(1200, 630);
      renderer.construct_taffy_tree(NodeKind::Container(ContainerNode {
        style: Style {
          width: 1200.0.into(),
          height: 630.0.into(),
          ..Default::default()
        },
        children: vec![],
      }));
      let image = renderer.draw(&context).unwrap();
      let mut buffer = Vec::new();
      let mut cursor = Cursor::new(&mut buffer);
      image.write_to(&mut cursor, ImageFormat::WebP).unwrap();
    })
  });

  // Text rendering
  group.bench_function("text_rendering", |b| {
    let node = TextNode {
      style: Style {
        inheritable_style: InheritableStyle {
          font_size: Some(48.0),
          color: Some(Color::Rgb(0, 0, 0).into()),
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
    let mut canvas = FastBlendImage(RgbaImage::new(800, 600));

    b.iter(|| {
      <TextNode as Node<NodeKind>>::draw_on_canvas(&node, &context, &mut canvas, layout);
    })
  });

  // Nested containers
  group.bench_function("nested_containers", |b| {
    b.iter(|| {
      let context = Context::default();
      let mut renderer = ImageRendererNodes::new(1200, 630);
      renderer.construct_taffy_tree(NodeKind::Container(ContainerNode {
        style: Style {
          width: 1200.0.into(),
          height: 630.0.into(),
          background_color: Some(Color::Rgb(240, 240, 240).into()),
          ..Default::default()
        },
        children: vec![NodeKind::Container(ContainerNode {
          style: Style {
            width: 400.0.into(),
            height: 400.0.into(),
            background_color: Some(Color::Rgb(200, 200, 200).into()),
            ..Default::default()
          },
          children: vec![NodeKind::Container(ContainerNode {
            style: Style {
              width: 200.0.into(),
              height: 200.0.into(),
              background_color: Some(Color::Rgb(160, 160, 160).into()),
              ..Default::default()
            },
            children: vec![],
          })],
        })],
      }));
      let image = renderer.draw(&context).unwrap();
      let mut buffer = Vec::new();
      let mut cursor = Cursor::new(&mut buffer);
      image.write_to(&mut cursor, ImageFormat::WebP).unwrap();
    })
  });

  group.finish();
}

fn render_parallel_optimization(c: &mut Criterion) {
  let mut group = c.benchmark_group("parallel_optimization");

  // Benchmark with many sibling nodes to test parallel benefits
  group.bench_function("many_siblings_parallel", |b| {
    b.iter(|| {
      let context = Context::default();
      let mut renderer = ImageRendererNodes::new(1200, 630);

      // Create a layout with many sibling containers to benefit from parallel processing
      let children: Vec<NodeKind> = (0..8)
        .map(|i| {
          NodeKind::Container(ContainerNode {
            style: Style {
              width: 100.0.into(),
              height: 100.0.into(),
              background_color: Some(Color::Rgb(100 + i * 20, 100, 100).into()),
              ..Default::default()
            },
            children: (0..4)
              .map(|j| {
                NodeKind::Text(TextNode {
                  style: Style {
                    width: 80.0.into(),
                    height: 20.0.into(),
                    inheritable_style: InheritableStyle {
                      font_size: Some(12.0),
                      color: Some(Color::Rgb(0, 0, 0).into()),
                      text_align: Some(TextAlign::Left),
                      ..Default::default()
                    },
                    ..Default::default()
                  },
                  text: format!("Child {} Text {}", i, j),
                })
              })
              .collect(),
          })
        })
        .collect();

      renderer.construct_taffy_tree(NodeKind::Container(ContainerNode {
        style: Style {
          width: 1200.0.into(),
          height: 630.0.into(),
          background_color: Some(Color::Rgb(240, 240, 240).into()),
          ..Default::default()
        },
        children,
      }));

      let image = renderer.draw(&context).unwrap();
      let mut buffer = Vec::new();
      let mut cursor = Cursor::new(&mut buffer);
      image.write_to(&mut cursor, ImageFormat::WebP).unwrap();
    })
  });

  group.finish();
}

fn render_performance_analysis(c: &mut Criterion) {
  let mut group = c.benchmark_group("performance_analysis");

  // Complex layout for detailed analysis
  let complex_layout = || {
    NodeKind::Container(ContainerNode {
      style: Style {
        width: 1200.0.into(),
        height: 630.0.into(),
        background_color: Some(Color::Rgb(240, 240, 240).into()),
        ..Default::default()
      },
      children: vec![
        NodeKind::Text(TextNode {
          style: Style {
            width: 800.0.into(),
            height: 100.0.into(),
            inheritable_style: InheritableStyle {
              font_size: Some(48.0),
              color: Some(Color::Rgb(0, 0, 0).into()),
              text_align: Some(TextAlign::Center),
              font_weight: Some(FontWeight(700)),
              ..Default::default()
            },
            ..Default::default()
          },
          text: "Complex Layout Benchmark".to_string(),
        }),
        NodeKind::Container(ContainerNode {
          style: Style {
            width: 1000.0.into(),
            height: 400.0.into(),
            background_color: Some(Color::Rgb(200, 200, 200).into()),
            ..Default::default()
          },
          children: vec![
            NodeKind::Text(TextNode {
              style: Style {
                width: 400.0.into(),
                height: 100.0.into(),
                inheritable_style: InheritableStyle {
                  font_size: Some(24.0),
                  color: Some(Color::Rgb(50, 50, 50).into()),
                  text_align: Some(TextAlign::Left),
                  ..Default::default()
                },
                ..Default::default()
              },
              text: "This is a complex layout with multiple nested elements and text nodes."
                .to_string(),
            }),
            NodeKind::Container(ContainerNode {
              style: Style {
                width: 200.0.into(),
                height: 200.0.into(),
                background_color: Some(Color::Rgb(160, 160, 160).into()),
                ..Default::default()
              },
              children: vec![],
            }),
          ],
        }),
      ],
    })
  };

  // Full pipeline benchmark
  group.bench_function("full_pipeline", |b| {
    b.iter(|| {
      let context = Context::default();
      let mut renderer = ImageRendererNodes::new(1200, 630);
      renderer.construct_taffy_tree(complex_layout());
      let image = renderer.draw(&context).unwrap();
      let mut buffer = Vec::new();
      let mut cursor = Cursor::new(&mut buffer);
      image.write_to(&mut cursor, ImageFormat::WebP).unwrap();
    })
  });

  // Tree creation step
  group.bench_function("tree_creation", |b| {
    b.iter(|| {
      let mut renderer = ImageRendererNodes::new(1200, 630);
      renderer.construct_taffy_tree(complex_layout());
    })
  });

  // Drawing step
  group.bench_function("image_drawing", |b| {
    let context = Context::default();
    let mut renderer = ImageRendererNodes::new(1200, 630);
    renderer.construct_taffy_tree(complex_layout());

    b.iter(|| renderer.draw(&context).unwrap())
  });

  // Encoding step
  group.bench_function("webp_encoding", |b| {
    let context = Context::default();
    let mut renderer = ImageRendererNodes::new(1200, 630);
    renderer.construct_taffy_tree(complex_layout());
    let image = renderer.draw(&context).unwrap();

    b.iter(|| {
      let mut buffer = Vec::new();
      let mut cursor = Cursor::new(&mut buffer);
      image.write_to(&mut cursor, ImageFormat::WebP).unwrap();
    })
  });

  group.finish();
}

criterion_group!(
  benches,
  render_scenarios,
  render_parallel_optimization,
  render_performance_analysis
);
criterion_main!(benches);
