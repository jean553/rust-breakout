//! Starting script of the program, contains the main loop.

extern crate piston_window;
extern crate graphics;

mod separators;
mod player;
mod display;

use piston_window::{
    PistonWindow,
    WindowSettings,
    MouseCursorEvent,
    clear,
};

use graphics::rectangle::Rectangle;

use separators::Separators;
use display::Display;

fn main() {

    const WINDOW_WIDTH: u32 = 1600;
    const WINDOW_HEIGHT: u32 = 900;

    let mut window: PistonWindow = WindowSettings::new(
        "Rust Breakout",
        [
            WINDOW_WIDTH,
            WINDOW_HEIGHT
        ]
    )
    .fullscreen(true)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let separators = Separators::new();

    while let Some(event) = window.next() {

        window.draw_2d(
            &event,
            |context, graphics| {

                separators.display(
                    context,
                    graphics,
                );

                const BLACK_COLOR: f32 = 0.0;
                const COLOR_COMPOSITE_AMOUNT: usize = 4;
                clear(
                    [
                        BLACK_COLOR;
                        COLOR_COMPOSITE_AMOUNT
                    ],
                    graphics,
                );

                const PLAYER_VERTICAL_POSITION: f64 = 890.0;
                const PLAYER_WIDTH: f64 = 100.0;
                const PLAYER_HEIGHT: f64 = 10.0;
                player.draw(
                    [
                        player_position,
                        PLAYER_VERTICAL_POSITION,
                        PLAYER_WIDTH,
                        PLAYER_HEIGHT,
                    ],
                    &context.draw_state,
                    context.transform,
                    graphics,
                );
            }
        );

        if let Some(position) = event.mouse_cursor_args() {
            player_position = position[0];
        }
    }
}
