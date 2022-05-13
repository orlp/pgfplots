use std::fmt;

/// PGFPlots options passed to the [`Axis`] environment.
///
/// The most commonly used key-value pairs are variants of the [`AxisKey`] enum.
/// The [`AxisKey::Custom`] variant is provided to add unimplemented keys and 
/// will be written verbatim in the options of the [`Axis`] environment.
#[derive(Clone, Debug)]
pub enum AxisKey {
    /// Custom key-value pairs that have not been implemented. These will be
    /// appended verbatim to the options of the [`Axis`].
    Custom(String),
    /// Control the scaling of the *x* axis.
    XMode(XMode),
    /// Control the scaling of the *y* axis.
    YMode(YMode),
}

impl fmt::Display for AxisKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AxisKey::Custom(key) => write!(f, "{key}"),
            AxisKey::XMode(value) => write!(f, "xmode={value}"),
            AxisKey::YMode(value) => write!(f, "ymode={value}"),
        }
    }
}

/// Axis environment inside a [`Picture`].
///
/// Adding an [`Axis`] to a [`Picture`] environment is equivalent to the PGFPlots 
/// axis environment:
///
/// ```text
/// \begin{axis}[AxisKeys]
///     % contents
/// \end{axis}
/// ```
#[derive(Clone, Debug)]
pub struct Axis {
    keys: Vec<AxisKey>,
}

impl fmt::Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\\begin{{axis}}")?;
        // If there are keys, print one per line. It makes it easier for a 
        // human to find individual keys later.
        if !self.keys.is_empty() {
            write!(f, "[\n")?;
            for key in self.keys.iter() {
                write!(f, "\t{key},\n")?;
            }
            write!(f, "]")?;
        }
        write!(f, "\n")?;

        // Need to implement Display for each addplot
        todo!();

        write!(f, "\\end{{axis}}")?;

        Ok(())
    }
}

/// Control the scaling of the *x* axis.
#[derive(Clone, Copy, Debug)]
pub enum XMode {
    /// Logarithmic scaling i.e. apply the natural logarithm to each *x* 
    /// coordinate.
    Log,
    /// Linear scaling of the *x* coordinates.
    Normal,
}
impl fmt::Display for XMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XMode::Log => write!(f, "log"),
            XMode::Normal => write!(f, "normal"),
        }
    }
}

/// Control the scaling of the *y* axis.
#[derive(Clone, Copy, Debug)]
pub enum YMode {
    /// Logarithmic scaling i.e. apply the natural logarithm to each *y* 
    /// coordinate.
    Log,
    /// Linear scaling of the *y* coordinates.
    Normal,
}
impl fmt::Display for YMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            YMode::Log => write!(f, "log"),
            YMode::Normal => write!(f, "normal"),
        }
    }
}