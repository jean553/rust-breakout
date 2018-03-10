//! The module that handles the two vertical separators
//! located at both side of the playable area.

use piston_window::G2d;

use graphics::rectangle::Rectangle;
use graphics::context::Context;

use display::Display;

/// Contains the left and right separators surfaces
pub struct Separators {
    left: Rectangle,
    right: Rectangle,
}

pub const LEFT_SEPARATOR_HORIZONTAL_POSITION: f64 = 300.0;

impl Separators {

    /// Initializes the two separators
    pub fn new() -> Separators {

        const SEPARATOR_WHITE_COLOR: [f32; 4] = [
            1.0,
            1.0,
            1.0,
            1.0,
        ];

        Separators {
            left: Rectangle::new(SEPARATOR_WHITE_COLOR),
            right: Rectangle::new(SEPARATOR_WHITE_COLOR),
        }
    }
}

impl Display for Separators {

    /// Displays the two separators
    ///
    /// # Arguments:
    ///
    /// * `context` - graphical context from the piston window
    /// * `graphics` - 2D graphics from the piston window
    fn display(
        &self,
        context: Context,
        graphics: &mut G2d,
    ) {

        const SEPARATOR_VERTICAL_POSITION: f64 = 0.0;
        const SEPARATOR_WIDTH: f64 = 1.0;
        const SEPARATOR_HEIGHT: f64 = 1600.0;

        self.left.draw(
            [
                LEFT_SEPARATOR_HORIZONTAL_POSITION,
                SEPARATOR_VERTICAL_POSITION,
                SEPARATOR_WIDTH,
                SEPARATOR_HEIGHT,
            ],
            &context.draw_state,
            context.transform,
            graphics,
        );

        const RIGHT_SEPARATOR_HORIZONTAL_POSITION: f64 = 1300.0;

        self.right.draw(
            [
                RIGHT_SEPARATOR_HORIZONTAL_POSITION,
                SEPARATOR_VERTICAL_POSITION,
                SEPARATOR_WIDTH,
                SEPARATOR_HEIGHT,
            ],
            &context.draw_state,
            context.transform,
            graphics,
        );
    }
}
