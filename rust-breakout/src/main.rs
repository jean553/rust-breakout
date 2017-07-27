extern crate piston_window;
extern crate graphics;

use piston_window::{
    PistonWindow,
    WindowSettings,
    MouseCursorEvent,
    clear,
};

use graphics::rectangle::Rectangle;

fn main() {

    let mut window: PistonWindow = WindowSettings::new(
        "Rust Breakout",
        [1600, 900]
    )
    .fullscreen(true)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let player = Rectangle::new([0.3, 0.3, 0.3, 1.0]);
    let mut player_position: f64 = 0.0;

    while let Some(event) = window.next() {

        window.draw_2d(
            &event,
            |context, graphics| {

                clear(
                    [0.0; 4],
                    graphics,
                );

                player.draw(
                    [
                        player_position,
                        890.0,
                        100.0,
                        10.0
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
