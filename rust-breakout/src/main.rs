//! A simple breakout video game.

extern crate piston_window;
extern crate graphics;

mod separators;
mod player;
mod ball;
mod display;
mod cell;

use std::time::Instant;

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
use cell::Cell;

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

    const CELLS_AMOUNT: usize = 100;
    let mut cells = [
        Cell::new();
        CELLS_AMOUNT
    ];

    const CELLS_PER_LINE: u8 = 20;

    let mut vertical_index = 0;
    for (index, cell) in cells.iter_mut().enumerate() {

        let horizontal_index = index as u8 % CELLS_PER_LINE;

        if index != 0 && horizontal_index == 0 {
            vertical_index += 1;
        }

        cell.set_position_by_indices(
            horizontal_index,
            vertical_index,
        );
    }

    let mut last_time: u64 = 0;
    let timer = Instant::now();

    let mut last_ball_column = 0;
    let mut last_cell_index = 0;

    let mut update_column = true;

    let mut last_cell_vertical_position =
        cells[last_cell_index].get_vertical_position();
    let mut last_cell_bottom =
        last_cell_vertical_position + cell::HEIGHT * 2.0;

    while let Some(event) = window.next() {

        const ANIMATION_INTERVAL: u64 = 40;
        let current_time = get_elapsed_time(&timer);
        if ball.is_moving() && current_time - last_time > ANIMATION_INTERVAL {

            if ball.is_at_right_border() {
                ball.horizontally_invert_direction();
            }

            if ball.is_at_left_border() {
                ball.horizontally_invert_direction();
            }

            ball.update_position();

            let ball_horizontal_position = ball.get_horizontal_position() -
                separators::LEFT_SEPARATOR_HORIZONTAL_POSITION;

            const DISTANCE_BETWEEN_CELLS: f64 = 50.0;
            let ball_column =
                (ball_horizontal_position / DISTANCE_BETWEEN_CELLS) as usize;

            if ball_column != last_ball_column || update_column {

                last_cell_index = ball_column;

                /* FIXME: should handle the case when
                   there is no cell into the column */

                const CELLS_PER_COLUMN: usize = 5;
                for _ in 1..CELLS_PER_COLUMN {

                    let index = last_cell_index + CELLS_PER_LINE as usize;

                    if !cells[index].is_visible() {
                        break;
                    }

                    last_cell_index = index;
                }

                last_cell_vertical_position =
                    cells[last_cell_index].get_vertical_position();
                last_cell_bottom =
                    last_cell_vertical_position + cell::HEIGHT;

                update_column = false;
                last_ball_column = ball_column;
            }

            let ball_vertical_position = ball.get_vertical_position();
            let ball_horizontal_position = ball.get_horizontal_position();
            let player_position = player.get_position();

            if ball_vertical_position < last_cell_bottom {
                ball.set_vertical_position(last_cell_bottom);
                ball.vertically_invert_direction();

                cells[last_cell_index].hide();

                update_column = true;
            }
            else if
                ball_vertical_position + player::HEIGHT >
                    player::PLAYER_VERTICAL_POSITION &&
                ball_horizontal_position > player_position &&
                ball_horizontal_position < player_position + player::WIDTH
            {
                if ball_horizontal_position < player_position + 20.0 {
                    ball.set_horizontal_direction(-10.0);
                }
                else if
                    ball_horizontal_position >= player_position + 20.0 &&
                    ball_horizontal_position < player_position + 40.0
                {
                    ball.set_horizontal_direction(-5.0);
                }
                else if
                    ball_horizontal_position >= player_position + 40.0 &&
                    ball_horizontal_position < player_position + 60.0
                {
                    ball.set_horizontal_direction(0.0);
                }
                else if
                    ball_horizontal_position >= player_position + 60.0 &&
                    ball_horizontal_position < player_position + 80.0
                {
                    ball.set_horizontal_direction(5.0);
                } else {
                    ball.set_horizontal_direction(10.0);
                }

                ball.vertically_invert_direction();
            }

            last_time = current_time;
        }

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

                for cell in cells.iter() {

                    if !cell.is_visible() {
                        continue;
                    }

                    cell.display(
                        context,
                        graphics,
                    );
                }

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

/// Returns the elapsed time in milliseconds according to the given timer.
///
/// # Arguments:
///
/// * `timer` - the given timer started before the function call
///
/// # Returns:
///
/// The elapsed time in milliseconds.
fn get_elapsed_time(timer: &Instant) -> u64 {

    let current_timer = timer.elapsed();
    let current_second = current_timer.as_secs();
    let current_nanosecond = current_timer.subsec_nanos();

    const MILLISECONDS_IN_ONE_SECOND: u64 = 1000;
    const NANOSECONDS_IN_ONE_MILLISECOND: u64 = 1000000;

    current_second * MILLISECONDS_IN_ONE_SECOND +
        current_nanosecond as u64 / NANOSECONDS_IN_ONE_MILLISECOND
}
