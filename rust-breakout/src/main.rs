extern crate piston_window;
extern crate graphics;

use piston_window::{
    PistonWindow,
    WindowSettings,
    rectangle,
};

use graphics::rectangle::Rectangle;

fn main() {

    let mut window: PistonWindow = WindowSettings::new(
        "Rust Breakout",
        [600, 400]
    )
    .build()
    .unwrap();

    let player = Rectangle::new([0.3, 0.3, 0.3, 1.0]);

    while let Some(event) = window.next() {

        window.draw_2d(
            &event,
            |context, graphics| {

                player.draw(
                    [0.0, 390.0, 100.0, 10.0],
                    &context.draw_state,
                    context.transform,
                    graphics,
                );
            }
        );
    }
}
