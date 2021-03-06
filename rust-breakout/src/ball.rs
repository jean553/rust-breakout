//! The module that renders the ball.

use std::f64::consts::PI;

use piston_window::G2d;

use graphics::circle_arc::CircleArc;
use graphics::context::Context;

use display::Display;

const RIGHT_BORDER_HORIZONTAL_POSITION: f64 = 1300.0;
const LEFT_BORDER_HORIZONTAL_POSITION: f64 = 300.0;

pub struct Ball {
    circle: CircleArc,
    horizontal_position: f64,
    vertical_position: f64,
    horizontal_direction: f64,
    vertical_direction: f64,
    moving: bool,
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

        const DEFAULT_HORIZONTAL_POSITION: f64 = 800.0;
        const DEFAULT_VERTICAL_POSITION: f64 = 876.0;

        const DEFAULT_HORIZONTAL_DIRECTION: f64 = 8.0;
        const DEFAULT_VERTICAL_DIRECTION: f64 = -6.0;

        Ball {
            circle: CircleArc::new(
                BALL_GREY_COLOR,
                BALL_RADIUS,
                BALL_RADIUS_START,
                BALL_RADIUS_END,
            ),
            horizontal_position: DEFAULT_HORIZONTAL_POSITION,
            vertical_position: DEFAULT_VERTICAL_POSITION,
            horizontal_direction: DEFAULT_HORIZONTAL_DIRECTION,
            vertical_direction: DEFAULT_VERTICAL_DIRECTION,
            moving: false,
        }
    }

    /// Setter for the horizontal position of the ball.
    ///
    /// # Arguments:
    ///
    /// * `position` - the horizontal position to set
    pub fn set_horizontal_position(
        &mut self,
        position: f64,
    ) {
        self.horizontal_position = position;
    }

    /// Getter for the horizontal position of the cell.
    ///
    /// # Returns:
    ///
    /// the horizontal position
    pub fn get_horizontal_position(&self) -> f64 {
        self.horizontal_position
    }

    /// Setter for the vertical position of the ball.
    ///
    /// # Arguments:
    ///
    /// * `position` - the vertical position to set
    pub fn set_vertical_position(
        &mut self,
        position: f64,
    ) {
        self.vertical_position = position;
    }

    /// Getter for the vertical position of the cell.
    ///
    /// # Returns:
    ///
    /// the vertical position
    pub fn get_vertical_position(&self) -> f64 {
        self.vertical_position
    }

    /// Setter of the ball horizontal direction.
    ///
    /// # Arguments:
    ///
    /// `direction` - the update horizontal direction
    pub fn set_horizontal_direction(
        &mut self,
        direction: f64,
    ) {
        self.horizontal_direction = direction;
    }

    /// Indicates if the ball is moving.
    ///
    /// # Returns:
    ///
    /// True if the ball is moving, False if the ball is attached to the player
    pub fn is_moving(&self) -> bool {
        self.moving
    }

    /// Allows the ball to move, the ball is not attached to the player anymore
    pub fn allow_move(&mut self) {
        self.moving = true;
    }

    /// Updates the positions of the ball according to the directions
    pub fn update_position(&mut self) {
        self.horizontal_position += self.horizontal_direction;
        self.vertical_position += self.vertical_direction;
    }

    /// Indicates if the ball is touching the right border of the game area
    ///
    /// # Returns:
    ///
    /// True if the ball is touching a border,
    /// False if the ball is not touching any border
    pub fn is_at_right_border(&self) -> bool {

        const BALL_WIDTH: f64 = 20.0;
        let right_horizontal_position = self.horizontal_position + BALL_WIDTH;

        if right_horizontal_position > RIGHT_BORDER_HORIZONTAL_POSITION {
            return true;
        }

        false
    }

    /// Indicates if the ball is touching the left border of the game area
    ///
    /// # Returns:
    ///
    /// True if the ball is touching a border,
    /// False if the ball is not touching any border
    pub fn is_at_left_border(&self) -> bool {

        if self.horizontal_position - 5.0 < LEFT_BORDER_HORIZONTAL_POSITION {
            return true;
        }

        false
    }

    /// Horizontally inverts the ball direction
    pub fn horizontally_invert_direction(&mut self) {
        const INVERT: f64 = -1.0;
        self.horizontal_direction *= INVERT;
    }

    /// Vertically inverts the ball direction
    pub fn vertically_invert_direction(&mut self) {
        const INVERT: f64 = -1.0;
        self.vertical_direction *= INVERT;
    }
}

impl Display for Ball {

    /// Displays the ball
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

        const BALL_WIDTH: f64 = 10.0;
        const BALL_HEIGHT: f64 = 10.0;

        self.circle.draw(
            [
                self.horizontal_position,
                self.vertical_position,
                BALL_WIDTH,
                BALL_HEIGHT,
            ],
            &context.draw_state,
            context.transform,
            graphics,
        );
    }
}
