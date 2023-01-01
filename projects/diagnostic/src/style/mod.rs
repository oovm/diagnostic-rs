#[macro_use]
mod macros;

mod color;
mod paint;
mod style;
#[cfg(test)]
mod tests;
mod windows;

pub use color::Color;
pub use paint::Paint;
pub use style::Style;
