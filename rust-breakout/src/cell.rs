//! The module that renders the cell.

use graphics::rectangle::Rectangle;

pub struct Cell {
    rectangle: Rectangle,
    horizontal_position: f64,
    vertical_position: f64,
}

impl Cell {

    /// Initializes the cell
    pub fn new(
        horizontal_position: f64,
        vertical_position: f64,
    ) -> Cell {

        const ORANGE_COLOR: [f32; 4] = [
            0.3,
            0.3,
            0.3,
            1.0,
        ];

        Cell {
            rectangle: Rectangle::new(ORANGE_COLOR),
            horizontal_position: horizontal_position,
            vertical_position: vertical_position,
        }
    }
}
