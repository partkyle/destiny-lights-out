extern crate piston_window;

mod game;

use piston_window::*;

use game::Game;

fn main() {
    let window_size = [640, 480];
    let mut window: PistonWindow = WindowSettings::new("Lights Out", window_size)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new();
    while let Some(event) = window.next() {
        let size = window.size();
        game.set_size(size.width, size.height);
        window.draw_2d(&event, |context, graphics, _device| {
            game.draw(context, graphics);
        });
    }
}
