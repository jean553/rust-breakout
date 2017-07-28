//! The module that renders the player.

use graphics::rectangle::Rectangle;

pub struct Player {
    rectangle: Rectangle,
    position: f32,
}

impl Player {

    /// Initializes the player.
    pub fn new() -> Player {

        const GREY_COLOR: [f32; 4] = [
            0.3,
            0.3,
            0.3,
            1.0,
        ];

        const PLAYER_DEFAULT_POSITION: f32 = 850.0;

        Player {
            rectangle: Rectangle::new(GREY_COLOR),
            position: PLAYER_DEFAULT_POSITION,
        }
    }
}
