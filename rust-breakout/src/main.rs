extern crate piston_window;

use piston_window::{
    PistonWindow,
    WindowSettings,
};

fn main() {

    let mut window: PistonWindow = WindowSettings::new(
        "Rust Breakout",
        [600, 400]
    )
    .build()
    .unwrap();

    while let Some(event) = window.next() {
    }
}
