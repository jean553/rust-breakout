//! Declaration of trait `Display`
//!
//! This trait is implemented by every game-level defined
//! (and not library-level) displayable components.
//!
//! TODO: this trait is only used by one item for now;
//! it should be implemented for the player surface as well.

use piston_window::G2d;

use graphics::context::Context;

pub trait Display {

    /// Displays the component.
    fn display(
        &self,
        Context,
        &mut G2d,
    );
}
