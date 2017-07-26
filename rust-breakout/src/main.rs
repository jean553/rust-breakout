extern crate piston_window;

use piston_window::{
    PistonWindow,
    WindowSettings,
    rectangle,
};

fn main() {

    let mut window: PistonWindow = WindowSettings::new(
        "Rust Breakout",
        [600, 400]
    )
    .build()
    .unwrap();

    while let Some(event) = window.next() {

        window.draw_2d(
            &event,
            |context, graphics| {
                rectangle(
                    [0.3, 0.3, 0.3, 1.0],
                    [0.0, 390.0, 100.0, 10.0],
                    context.transform,
                    graphics,
                );
            }
        );
    }
}
