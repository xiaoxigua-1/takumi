use std::fmt::Debug;

use cssparser::{BasicParseError, BasicParseErrorKind, ParseError, Parser, ParserInput};
use serde::{Deserialize, Deserializer, Serialize};
use ts_rs::TS;

use crate::{
  FromCss,
  length_unit::LengthUnit,
  properties::{ParseResult, color::Color},
};

/// Represents a box shadow with all its properties.
///
/// Box shadows can be either outset (default) or inset, and consist of:
/// - Horizontal and vertical offsets
/// - Blur radius (optional, defaults to 0)
/// - Spread radius (optional, defaults to 0)
/// - Color (optional, defaults to transparent)
#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize, TS)]
#[ts(as = "BoxShadowValue")]
#[serde(try_from = "BoxShadowValue")]
pub struct BoxShadow {
  /// Whether the shadow is inset (inside the element) or outset (outside the element).
  pub inset: bool,
  /// Horizontal offset of the shadow.
  pub offset_x: LengthUnit,
  /// Vertical offset of the shadow.
  pub offset_y: LengthUnit,
  /// Blur radius of the shadow. Higher values create a more blurred shadow.
  pub blur_radius: LengthUnit,
  /// Spread radius of the shadow. Positive values expand the shadow, negative values shrink it.
  pub spread_radius: LengthUnit,
  /// Color of the shadow.
  pub color: Color,
}

/// Proxy type for `BoxShadow` Css deserialization.
#[derive(Debug, Clone, PartialEq, TS, Deserialize)]
#[serde(untagged)]
pub enum BoxShadowValue {
  /// Represents a structured box shadow.
  #[serde(rename_all = "camelCase")]
  Structured {
    /// Whether the shadow is inset (inside the element) or outset (outside the element).
    inset: bool,
    /// Horizontal offset of the shadow.
    offset_x: LengthUnit,
    /// Vertical offset of the shadow.
    offset_y: LengthUnit,
    /// Blur radius of the shadow. Higher values create a more blurred shadow.
    blur_radius: LengthUnit,
    /// Spread radius of the shadow. Positive values expand the shadow, negative values shrink it.
    spread_radius: LengthUnit,
    /// Color of the shadow.
    color: Color,
  },
  /// Represents a CSS string.
  Css(String),
}

impl TryFrom<BoxShadowValue> for BoxShadow {
  type Error = &'static str;

  fn try_from(value: BoxShadowValue) -> Result<Self, Self::Error> {
    match value {
      BoxShadowValue::Structured {
        inset,
        offset_x,
        offset_y,
        blur_radius,
        spread_radius,
        color,
      } => Ok(BoxShadow {
        inset,
        offset_x,
        offset_y,
        blur_radius,
        spread_radius,
        color,
      }),
      BoxShadowValue::Css(css) => {
        let mut input = ParserInput::new(&css);
        let mut parser = Parser::new(&mut input);

        BoxShadow::from_css(&mut parser).map_err(|_| "Failed to parse box-shadow")
      }
    }
  }
}

/// Represents a collection of box shadows, have custom `FromCss` implementation for comma-separated values.
#[derive(Debug, Clone, PartialEq, TS, Serialize)]
pub struct BoxShadows(pub Vec<BoxShadow>);

impl<'de> Deserialize<'de> for BoxShadows {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;

    let mut input = ParserInput::new(&s);
    let mut parser = Parser::new(&mut input);

    let mut shadows = Vec::new();

    loop {
      let shadow = BoxShadow::from_css(&mut parser)
        .map_err(|_| serde::de::Error::custom("Failed to parse box-shadow"))?;

      shadows.push(shadow);

      if parser.expect_comma().is_err() {
        break; // No more shadows, exit loop
      }
    }

    Ok(BoxShadows(shadows))
  }
}

impl<'i> FromCss<'i> for BoxShadow {
  /// Parses a box-shadow value from CSS input.
  ///
  /// The box-shadow syntax allows for the following components in any order:
  /// - inset keyword (optional)
  /// - Two length values for horizontal and vertical offsets (required)
  /// - Two optional length values for blur radius and spread radius
  /// - A color value (optional)
  ///
  /// Examples:
  /// - `box-shadow: 2px 4px;`
  /// - `box-shadow: 2px 4px 6px;`
  /// - `box-shadow: 2px 4px 6px 8px;`
  /// - `box-shadow: 2px 4px red;`
  /// - `box-shadow: inset 2px 4px 6px red;`
  fn from_css(input: &mut Parser<'i, '_>) -> ParseResult<'i, BoxShadow> {
    let mut color = None;
    let mut lengths = None;
    let mut inset = false;

    // Parse all components in a loop, as they can appear in any order
    loop {
      // Try to parse the "inset" keyword if not already found
      if !inset
        && input
          .try_parse(|input| input.expect_ident_matching("inset"))
          .is_ok()
      {
        inset = true;
        continue;
      }

      // Try to parse length values (offsets, blur radius, spread radius)
      if lengths.is_none() {
        let value = input.try_parse::<_, _, ParseError<BasicParseError<'i>>>(|input| {
          // Parse the required horizontal and vertical offsets
          let horizontal = LengthUnit::from_css(input)?;
          let vertical = LengthUnit::from_css(input)?;

          // Parse optional blur radius (defaults to 0)
          let blur = input
            .try_parse(LengthUnit::from_css)
            .unwrap_or(LengthUnit::zero());

          // Parse optional spread radius (defaults to 0)
          let spread = input
            .try_parse(LengthUnit::from_css)
            .unwrap_or(LengthUnit::zero());

          Ok((horizontal, vertical, blur, spread))
        });

        if let Ok(value) = value {
          lengths = Some(value);
          continue;
        }
      }

      // Try to parse a color value if not already found
      if color.is_none() {
        if let Ok(value) = input.try_parse(Color::from_css) {
          color = Some(value);
          continue;
        }
      }

      // If we can't parse anything else, break out of the loop
      break;
    }

    // At minimum, we need the two required length values (offsets)
    let lengths = lengths.ok_or(input.new_error(BasicParseErrorKind::QualifiedRuleInvalid))?;

    // Construct the BoxShadow with parsed values or defaults
    Ok(BoxShadow {
      // Use parsed color or default to transparent
      color: color.unwrap_or_else(Color::transparent),
      offset_x: lengths.0,
      offset_y: lengths.1,
      blur_radius: lengths.2,
      spread_radius: lengths.3,
      inset,
    })
  }
}

#[cfg(test)]
mod tests {
  use cssparser::{Parser, ParserInput};

  use super::*;
  use crate::properties::color::Color;
  use LengthUnit::Px;

  /// Helper function to parse box-shadow strings for testing
  fn parse_box_shadow_str(input: &str) -> ParseResult<'_, BoxShadow> {
    let mut parser_input = ParserInput::new(input);
    let mut parser = Parser::new(&mut parser_input);
    BoxShadow::from_css(&mut parser)
  }

  #[test]
  fn test_parse_simple_box_shadow() {
    // Test parsing a simple box-shadow with just offsets
    let result = parse_box_shadow_str("2px 4px").unwrap();
    assert_eq!(result.offset_x, Px(2.0));
    assert_eq!(result.offset_y, Px(4.0));
    assert_eq!(result.blur_radius, LengthUnit::zero());
    assert_eq!(result.spread_radius, LengthUnit::zero());
    assert_eq!(result.color, Color::transparent());
    assert!(!result.inset);
  }

  #[test]
  fn test_parse_box_shadow_with_blur() {
    // Test parsing box-shadow with blur radius
    let result = parse_box_shadow_str("2px 4px 6px").unwrap();
    assert_eq!(result.offset_x, Px(2.0));
    assert_eq!(result.offset_y, Px(4.0));
    assert_eq!(result.blur_radius, Px(6.0));
    assert_eq!(result.spread_radius, LengthUnit::zero());
    assert_eq!(result.color, Color::transparent());
    assert!(!result.inset);
  }

  #[test]
  fn test_parse_box_shadow_with_spread() {
    // Test parsing box-shadow with blur and spread radius
    let result = parse_box_shadow_str("2px 4px 6px 8px").unwrap();
    assert_eq!(result.offset_x, Px(2.0));
    assert_eq!(result.offset_y, Px(4.0));
    assert_eq!(result.blur_radius, Px(6.0));
    assert_eq!(result.spread_radius, Px(8.0));
    assert_eq!(result.color, Color::transparent());
    assert!(!result.inset);
  }

  #[test]
  fn test_parse_box_shadow_with_color() {
    // Test parsing box-shadow with color
    let result = parse_box_shadow_str("2px 4px red").unwrap();
    assert_eq!(result.offset_x, Px(2.0));
    assert_eq!(result.offset_y, Px(4.0));
    assert_eq!(result.blur_radius, LengthUnit::zero());
    assert_eq!(result.spread_radius, LengthUnit::zero());
    assert_eq!(result.color, Color([255, 0, 0, 255]));
    assert!(!result.inset);
  }

  #[test]
  fn test_parse_inset_box_shadow() {
    // Test parsing inset box-shadow
    let result = parse_box_shadow_str("inset 2px 4px").unwrap();
    assert_eq!(result.offset_x, Px(2.0));
    assert_eq!(result.offset_y, Px(4.0));
    assert_eq!(result.blur_radius, LengthUnit::zero());
    assert_eq!(result.spread_radius, LengthUnit::zero());
    assert_eq!(result.color, Color::transparent());
    assert!(result.inset);
  }

  #[test]
  fn test_parse_box_shadow_different_order() {
    // Test parsing box-shadow with components in different order
    let result = parse_box_shadow_str("red 2px 4px").unwrap();
    assert_eq!(result.offset_x, Px(2.0));
    assert_eq!(result.offset_y, Px(4.0));
    assert_eq!(result.color, Color([255, 0, 0, 255]));

    let result = parse_box_shadow_str("2px 4px inset red").unwrap();
    assert_eq!(result.offset_x, Px(2.0));
    assert_eq!(result.offset_y, Px(4.0));
    assert_eq!(result.color, Color([255, 0, 0, 255]));
    assert!(result.inset);
  }

  #[test]
  fn test_parse_box_shadow_hex_color() {
    // Test parsing box-shadow with hex color
    let result = parse_box_shadow_str("2px 4px #ff0000").unwrap();
    assert_eq!(result.offset_x, Px(2.0));
    assert_eq!(result.offset_y, Px(4.0));
    assert_eq!(result.color, Color([255, 0, 0, 255]));
  }

  #[test]
  fn test_parse_box_shadow_rgba_color() {
    // Test parsing box-shadow with rgba color
    let result = parse_box_shadow_str("2px 4px rgba(255, 0, 0, 0.5)").unwrap();
    assert_eq!(result.offset_x, Px(2.0));
    assert_eq!(result.offset_y, Px(4.0));
    assert_eq!(result.color, Color([255, 0, 0, 128])); // 0.5 * 255 = 128
  }

  #[test]
  fn test_parse_box_shadow_invalid() {
    // Test parsing invalid box-shadow (missing required offsets)
    let result = parse_box_shadow_str("2px");
    assert!(result.is_err());

    // Test parsing invalid box-shadow (no values)
    let result = parse_box_shadow_str("");
    assert!(result.is_err());
  }
}
