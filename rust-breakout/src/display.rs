use piston_window::G2d;

use graphics::context::Context;

pub trait Display {

    fn display(
        &self,
        Context,
        &mut G2d,
    );
}
