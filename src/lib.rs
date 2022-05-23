//! A Rust library that generates PGFPlots code to `\input` into LaTeX documents.

use crate::axis::Axis;
use std::fmt;

/// Axis environment inside a [`Picture`].
pub mod axis;

/// Ti*k*Z options passed to the [`Picture`] environment.
///
/// The most commonly used key-value pairs are variants of the [`PictureKey`]
/// enum. The [`PictureKey::Custom`] variant is provided to add unimplemented
/// keys and will be written verbatim in the options of the [`Picture`]
/// environment.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PictureKey {
    /// Custom key-value pairs that have not been implemented. These will be
    /// appended verbatim to the options of the [`Picture`].
    Custom(String),
}

impl fmt::Display for PictureKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PictureKey::Custom(key) => write!(f, "{key}"),
        }
    }
}

/// Picture environment.
///
/// Creating a [`Picture`] is equivalent to the Ti*k*Z graphics environment:
///
/// ```text
/// \begin{tikzpicture}[PictureKeys]
///     % contents
/// \end{tikzpicture}
/// ```
#[derive(Clone, Debug, Default)]
pub struct Picture {
    keys: Vec<PictureKey>,
    pub axes: Vec<Axis>,
}

impl fmt::Display for Picture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\\begin{{tikzpicture}}")?;
        // If there are keys, print one per line. It makes it easier for a
        // human later to find keys if they are divided by lines.
        if !self.keys.is_empty() {
            writeln!(f, "[")?;
            for key in self.keys.iter() {
                writeln!(f, "\t{key},")?;
            }
            write!(f, "]")?;
        }
        writeln!(f)?;

        for axis in self.axes.iter() {
            writeln!(f, "{axis}")?;
        }

        write!(f, "\\end{{tikzpicture}}")?;

        Ok(())
    }
}

impl Picture {
    /// Create a new, empty picture environment.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::Picture;
    ///
    /// let mut picture = Picture::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }
    /// Add a key to control the appearance of the picture. This will overwrite
    /// any previous mutually exclusive key.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::{Picture, PictureKey};
    ///
    /// let mut picture = Picture::new();
    ///
    /// picture.add_key(PictureKey::Custom(String::from("baseline")));
    /// ```
    pub fn add_key(&mut self, key: PictureKey) {
        match key {
            PictureKey::Custom(_) => (),
            // If/whenever another variant is added, handle it the same way as
            // Axis::add_key and Plot2D::add_key
        }
        self.keys.push(key);
    }
}

#[cfg(test)]
mod tests;
