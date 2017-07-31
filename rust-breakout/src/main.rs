//! Starting script of the program, contains the main loop.

extern crate piston_window;
extern crate graphics;

mod separators;
mod player;
mod ball;
mod display;

use piston_window::{
    PistonWindow,
    WindowSettings,
    MouseCursorEvent,
    PressEvent,
    clear,
    G2d,
    Button,
    MouseButton,
};

use separators::Separators;
use player::Player;
use ball::Ball;
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
    let mut ball = Ball::new();

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

                ball.display(
                    context,
                    graphics,
                );

                clear_screen(graphics);
            }
        );

        if let Some(position) = event.mouse_cursor_args() {

            const PLAYER_MIN_POSITION: f64 = 300.0;
            const PLAYER_MAX_POSITION: f64 = 1200.0;

            let current_position = position[0];
            let expected_position = if
                current_position <= PLAYER_MAX_POSITION &&
                current_position >= PLAYER_MIN_POSITION {
                current_position
            } else if current_position < PLAYER_MIN_POSITION {
                PLAYER_MIN_POSITION
            } else {
                PLAYER_MAX_POSITION
            };

            player.set_position(expected_position);

            if !ball.is_moving() {

                const BALL_ON_PLAYER_POSITION_OFFSET: f64 = 50.0;
                let ball_horizontal_position =
                    expected_position + BALL_ON_PLAYER_POSITION_OFFSET;
                ball.set_horizontal_position(ball_horizontal_position);
            }
        }

        if let Some(button) = event.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                ball.allow_move();
            }
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
