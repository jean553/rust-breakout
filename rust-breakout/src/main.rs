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
    G2d,
};

use separators::Separators;
use player::Player;
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
    let mut player = Player::new();

    while let Some(event) = window.next() {

        window.draw_2d(
            &event,
            |context, graphics| {

                separators.display(
                    context,
                    graphics,
                );

                player.display(
                    context,
                    graphics,
                );

                clear_screen(graphics);
            }
        );

        if let Some(position) = event.mouse_cursor_args() {
            move_player(
                &mut player,
                position[0], // horizontal position only
            );
        }
    }
}

/// Turns the whole screen to black, used to update the rendered content.
///
/// # Arguments:
///
/// * `graphics` - 2D graphics from the piston window
fn clear_screen(graphics: &mut G2d) {

    const BLACK_COLOR: f32 = 0.0;
    const COLOR_COMPOSITE_AMOUNT: usize = 4;
    clear(
        [
            BLACK_COLOR;
            COLOR_COMPOSITE_AMOUNT
        ],
        graphics,
    );
}

/// Moves the player according to the current cursor horizontal position.
///
/// # Arguments:
///
/// * `player` - the player to move
/// * `cursor_position` - the cursor horizontal position
fn move_player(
    player: &mut Player,
    cursor_position: f64,
) {

    const PLAYER_MIN_POSITION: f64 = 300.0;
    const PLAYER_MAX_POSITION: f64 = 1200.0;

    let player_position = if
        cursor_position > PLAYER_MIN_POSITION &&
        cursor_position < PLAYER_MAX_POSITION {
        cursor_position
    } else if cursor_position < PLAYER_MIN_POSITION {
        PLAYER_MIN_POSITION
    } else {
        PLAYER_MAX_POSITION
    };

    player.set_position(player_position);
}
