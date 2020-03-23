extern crate piston_window;

mod game;

use piston_window::*;

use game::{Game, GameState};

fn main() {
    let window_size = [640, 480];
    let mut window: PistonWindow = WindowSettings::new("Lights Out", window_size)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new();
    while let Some(event) = window.next() {
        // update the size in the case some resizes have happened
        let size = window.size();
        game.set_size(size.width, size.height);

        // handle events
        if let GameState::Quit = game.handle_event(&event) {
            // TODO: should we just be quitting on win? that seems boring
            return;
        }

        // draw
        window.draw_2d(&event, |context, graphics, _device| {
            game.draw(context, graphics);
        });
    }
}
