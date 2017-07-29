//! The module that renders the ball.

use std::f64::consts::PI;

use piston_window::G2d;

use graphics::circle_arc::CircleArc;
use graphics::context::Context;

use display::Display;

pub struct Ball {
    circle: CircleArc,
}

impl Ball {

    /// Initializes the ball.
    pub fn new() -> Ball {

        const BALL_GREY_COLOR: [f32; 4] = [
            0.3,
            0.3,
            0.3,
            1.0,
        ];
        const BALL_RADIUS: f64 = 1.0;
        const BALL_RADIUS_START: f64 = 0.0;
        const BALL_RADIUS_END: f64 = 2.0 * PI - 0.01;

        Ball {
            circle: CircleArc::new(
                BALL_GREY_COLOR,
                BALL_RADIUS,
                BALL_RADIUS_START,
                BALL_RADIUS_END,
            )
        }
    }
}

impl Display for Ball {

    /// Displays the ball.
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

        const BALL_HORIZONTAL_POSITION: f64 = 800.0;
        const BALL_VERTICAL_POSITION: f64 = 500.0;
        const BALL_WIDTH: f64 = 10.0;
        const BALL_HEIGHT: f64 = 10.0;

        self.circle.draw(
            [
                BALL_HORIZONTAL_POSITION,
                BALL_VERTICAL_POSITION,
                BALL_WIDTH,
                BALL_HEIGHT,
            ],
            &context.draw_state,
            context.transform,
            graphics,
        );
    }
}
