use std::num::NonZeroUsize;

use serde::Deserialize;

/// Configuration used by the formatter
#[derive(Clone, Copy, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    /// Indentation to use
    pub indentation: Indentation,

    /// Whether to put spaces around brackets (enabled by default)
    pub space_around_brackets: bool,

    /// Preferred maximum line width
    pub max_width: Option<NonZeroUsize>,

    /// Whether to align trailing line comments
    pub align_trailing_comments: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            indentation: Indentation::default(),
            space_around_brackets: true,
            max_width: None,
            align_trailing_comments: false,
        }
    }
}

#[derive(Clone, Copy, Default, Deserialize)]
/// Indentation options
pub enum Indentation {
    /// Four spaces
    FourSpaces,
    /// Tabs
    Tabs,
    #[default]
    /// Two spaces
    TwoSpaces,
}
