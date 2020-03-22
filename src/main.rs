extern crate piston_window;

mod board_view;

use piston_window::*;

fn main() {
    let window_size = [640, 480];
    let mut window: PistonWindow = WindowSettings::new("Lights Out", window_size)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let board_view = board_view::BoardView::new();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            board_view.draw(context, graphics);
        });
    }
}
