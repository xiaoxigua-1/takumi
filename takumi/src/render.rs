use image::{ImageBuffer, RgbaImage};
use taffy::{AvailableSpace, NodeId, Point, TaffyTree, geometry::Size};

use crate::{
  context::Context,
  node::{
    ContainerNode, Node,
    draw::{FastBlendImage, draw_debug_border},
    style::ValuePercentageAuto,
  },
};

/// Type alias for a TaffyTree that uses `Box<dyn Node>` as its node type
pub type TaffyTreeWithNodes = TaffyTree<Box<dyn Node>>;

/// A renderer for creating images from a container node with specified dimensions.
///
/// The renderer takes a root container node and uses Taffy for layout calculations
/// to render the final image with the specified content dimensions.
pub struct ImageRenderer {
  /// The root container node that defines the layout structure
  pub root_node: ContainerNode,
  /// The width of the output image in pixels
  content_width: u32,
  /// The height of the output image in pixels
  content_height: u32,
}

/// Errors that can occur during image rendering.
#[derive(Debug)]
pub enum ImageRendererError {
  /// The content size is invalid or not specified
  InvalidContentSize,
}

impl TryFrom<ContainerNode> for ImageRenderer {
  type Error = ImageRendererError;

  /// Attempts to create an ImageRenderer from a ContainerNode.
  ///
  /// # Arguments
  /// * `value` - The container node to use as the root
  ///
  /// # Returns
  /// * `Result<Self, ImageRendererError>` - The created renderer or an error if the content size is invalid
  fn try_from(value: ContainerNode) -> Result<Self, Self::Error> {
    let style = value.get_style();

    let ValuePercentageAuto::SpecificValue(width) = style.width else {
      return Err(ImageRendererError::InvalidContentSize);
    };

    let ValuePercentageAuto::SpecificValue(height) = style.height else {
      return Err(ImageRendererError::InvalidContentSize);
    };

    Ok(Self {
      root_node: value,
      content_width: width as u32,
      content_height: height as u32,
    })
  }
}

impl ImageRenderer {
  /// Creates a new TaffyTree with the root node and returns both the tree and root node ID.
  ///
  /// # Returns
  /// * `(TaffyTreeWithNodes, NodeId)` - The created TaffyTree and the ID of the root node
  pub fn create_taffy_tree(&self) -> (TaffyTreeWithNodes, NodeId) {
    let mut taffy = TaffyTree::new();

    let root_node_id = self.root_node.create_taffy_leaf(&mut taffy).unwrap();

    (taffy, root_node_id)
  }

  /// Renders the image using the provided context and TaffyTree.
  ///
  /// # Arguments
  /// * `context` - The rendering context containing font and image information
  /// * `taffy` - The TaffyTree containing the layout information
  /// * `root_node_id` - The ID of the root node in the TaffyTree
  ///
  /// # Returns
  /// * `RgbaImage` - The rendered image
  pub fn draw(
    &self,
    context: &Context,
    taffy: &mut TaffyTreeWithNodes,
    root_node_id: NodeId,
  ) -> RgbaImage {
    let mut canvas = FastBlendImage(ImageBuffer::new(self.content_width, self.content_height));

    let available_space = Size {
      width: AvailableSpace::Definite(self.content_width as f32),
      height: AvailableSpace::Definite(self.content_height as f32),
    };

    taffy
      .compute_layout_with_measure(
        root_node_id,
        available_space,
        |known_dimensions, available_space, _node_id, node_context, _style| {
          let Some(node) = node_context else {
            return Size::ZERO;
          };

          if let Size {
            width: Some(width),
            height: Some(height),
          } = known_dimensions
          {
            return Size { width, height };
          }

          node.measure(context, available_space, known_dimensions)
        },
      )
      .unwrap();

    if context.print_debug_tree {
      taffy.print_tree(root_node_id);
    }

    draw_from_node_id_with_layout(context, &mut canvas, taffy, root_node_id, Point::zero());

    canvas.0
  }
}

/// Recursively draws nodes from the TaffyTree onto the canvas.
///
/// # Arguments
/// * `context` - The rendering context
/// * `canvas` - The canvas to draw on
/// * `taffy` - The TaffyTree containing layout information
/// * `node_id` - The ID of the current node to draw
/// * `relative_offset` - The offset from the parent node's position
fn draw_from_node_id_with_layout(
  context: &Context,
  canvas: &mut FastBlendImage,
  taffy: &mut TaffyTreeWithNodes,
  node_id: NodeId,
  relative_offset: Point<f32>,
) {
  let mut node_layout = *taffy.layout(node_id).unwrap();

  node_layout.location.x += relative_offset.x;
  node_layout.location.y += relative_offset.y;

  let node_kind = taffy.get_node_context(node_id).unwrap();

  node_kind.draw_on_canvas(context, canvas, node_layout);

  if context.draw_debug_border {
    draw_debug_border(canvas, node_layout);
  }

  for child_id in taffy.children(node_id).unwrap() {
    draw_from_node_id_with_layout(context, canvas, taffy, child_id, node_layout.location);
  }
}
