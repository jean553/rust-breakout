//! Renders the player

use piston_window::G2d;

use graphics::rectangle::Rectangle;
use graphics::context::Context;

use display::Display;

pub struct Player {
    rectangle: Rectangle,
    position: f64,
}

impl Player {

    /// Initializes the player
    pub fn new() -> Player {

        const GREY_COLOR: [f32; 4] = [
            0.3,
            0.3,
            0.3,
            1.0,
        ];

        const PLAYER_DEFAULT_POSITION: f64 = 850.0;

        Player {
            rectangle: Rectangle::new(GREY_COLOR),
            position: PLAYER_DEFAULT_POSITION,
        }
    }

    /// Setter of the mouse horizontal position (cursor horizontal position)
    ///
    /// # Arguments:
    ///
    /// * `position` - the horizontal position of the player
    pub fn set_position(
        &mut self,
        position: f64,
    ) {
        self.position = position;
    }

    /// Getter of the player position.
    ///
    /// # Returns:
    ///
    /// the horizontal position of the player.
    pub fn get_position(&self) -> f64 {
        self.position
    }
}

pub const PLAYER_VERTICAL_POSITION: f64 = 890.0;
pub const WIDTH: f64 = 100.0;
pub const HEIGHT: f64 = 10.0;

impl Display for Player {

    /// Displays the player at its position
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

        self.rectangle.draw(
            [
                self.position,
                PLAYER_VERTICAL_POSITION,
                WIDTH,
                HEIGHT,
            ],
            &context.draw_state,
            context.transform,
            graphics,
        );
    }
}
