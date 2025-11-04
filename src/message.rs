use crate::unist::{Point, Position};
use alloc::{boxed::Box, fmt, string::String};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Message {
    /// Place of message.
    #[cfg_attr(feature = "wasm", wasm_bindgen(skip))]
    pub place: Option<Box<Place>>,
    /// Reason for message (should use woodmarkdown).
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter_with_clone))]
    pub reason: String,
    /// Category of message.
    #[cfg_attr(feature = "wasm", wasm_bindgen(skip))]
    pub rule_id: Box<String>,
    /// Namespace of message.
    #[cfg_attr(feature = "wasm", wasm_bindgen(skip))]
    pub source: Box<String>,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref place) = self.place {
            write!(f, "{}: ", place)?;
        }

        write!(f, "{} ({}:{})", self.reason, self.source, self.rule_id)
    }
}

/// Somewhere.
#[derive(Clone, Debug, PartialEq)]
pub enum Place {
    /// Between two points.
    Position(Position),
    /// At a point.
    Point(Point),
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Place::Position(position) => write!(
                f,
                "{}:{}-{}:{}",
                position.start.line, position.start.column, position.end.line, position.end.column
            ),
            Place::Point(point) => write!(f, "{}:{}", point.line, point.column),
        }
    }
}
