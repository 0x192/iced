//! Write your own renderer.
//!
//! You will need to implement the `Renderer` trait first. It simply contains
//! an `Output` associated type.
//!
//! There is no common trait to draw all the widgets. Instead, every [`Widget`]
//! constrains its generic `Renderer` type as necessary.
//!
//! This approach is flexible and composable. For instance, the
//! [`Text`] widget only needs a [`text::Renderer`] while a [`Checkbox`] widget
//! needs both a [`text::Renderer`] and a [`checkbox::Renderer`], reusing logic.
//!
//! In the end, a __renderer__ satisfying all the constraints is
//! needed to build a [`UserInterface`].
//!
//! [`Widget`]: crate::Widget
//! [`UserInterface`]: crate::UserInterface
//! [`Text`]: crate::widget::Text
//! [`text::Renderer`]: crate::widget::text::Renderer
//! [`Checkbox`]: crate::widget::Checkbox
//! [`checkbox::Renderer`]: crate::widget::checkbox::Renderer

use crate::{layout, Color, Element, Rectangle};

/// A component that can take the state of a user interface and produce an
/// output for its users.
pub trait Renderer: Sized {
    /// Lays out the elements of a user interface.
    ///
    /// You should override this if you need to perform any operations before or
    /// after layouting. For instance, trimming the measurements cache.
    fn layout<'a, Message>(
        &mut self,
        element: &Element<'a, Message>,
        limits: &layout::Limits,
    ) -> layout::Node {
        element.layout(self, limits)
    }

    /// Overlays the `overlay` output with the given bounds on top of the `base`
    /// output.
    fn overlay(
        &mut self,
        // base: Self::Output,
        // overlay: Self::Output,
        overlay_bounds: Rectangle,
    );
}

/// Some default styling attributes.
#[derive(Debug, Clone, Copy)]
pub struct Defaults {
    /// Text styling
    pub text: Text,
}

impl Default for Defaults {
    fn default() -> Defaults {
        Defaults {
            text: Text::default(),
        }
    }
}

/// Some default text styling attributes.
#[derive(Debug, Clone, Copy)]
pub struct Text {
    /// The default color of text
    pub color: Color,
}

impl Default for Text {
    fn default() -> Text {
        Text {
            color: Color::BLACK,
        }
    }
}
