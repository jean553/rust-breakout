//! The module that renders the cell.

use piston_window::G2d;

use graphics::rectangle::Rectangle;
use graphics::context::Context;

use display::Display;

#[derive(Copy, Clone)]
pub struct Cell {
    rectangle: Rectangle,
    horizontal_position: f64,
    vertical_position: f64,
}

const CELL_WIDTH: f64 = 49.0;
const CELL_HEIGHT: f64 = 20.0;

impl Cell {

    /// Initializes the cell
    pub fn new() -> Cell {

        const ORANGE_COLOR: [f32; 4] = [
            1.0,
            0.5,
            0.2,
            1.0,
        ];

        Cell {
            rectangle: Rectangle::new(ORANGE_COLOR),
            horizontal_position: 0.0,
            vertical_position: 0.0,
        }
    }

    /// Set the graphical position of a cell from its indices
    ///
    /// # Arguments:
    ///
    /// * `horizontal_index` - the horizontal address of the cell
    /// * `vertical_index` - the vertical address of the cell
    pub fn set_position_by_indices(
        &mut self,
        horizontal_index: u8,
        vertical_index: u8,
    ) {
        const DEFAULT_HORIZONTAL_POSITION: f64 = 301.0;
        self.horizontal_position = DEFAULT_HORIZONTAL_POSITION +
            horizontal_index as f64 * (CELL_WIDTH + 1.0);
        self.vertical_position = vertical_index as f64 * (CELL_HEIGHT + 1.0);
    }
}

impl Display for Cell {

    /// Displays the cell
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
                self.horizontal_position,
                self.vertical_position,
                CELL_WIDTH,
                CELL_HEIGHT,
            ],
            &context.draw_state,
            context.transform,
            graphics,
        );
    }
}
