//! Trait for displayable content

use piston_window::G2d;

use graphics::context::Context;

pub trait Display {

    /// Displays the component
    fn display(
        &self,
        Context,
        &mut G2d,
    );
}
