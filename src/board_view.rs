pub struct BoardView {}

use piston_window::{clear, rectangle, Context, Graphics};

impl BoardView {
    pub fn new() -> BoardView {
        BoardView {}
    }

    pub fn draw<G: Graphics>(&self, context: Context, graphics: &mut G) {
        clear([1.0; 4], graphics);
        rectangle(
            [1.0, 0.0, 0.0, 1.0], // red
            [0.0, 0.0, 100.0, 100.0],
            context.transform,
            graphics,
        );
    }
}
