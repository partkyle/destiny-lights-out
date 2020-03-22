use piston_window::{clear, rectangle, Context, Graphics};

pub struct Colors {
    background: [f32; 4],
    slot_color: [f32; 4],
}

impl Colors {
    pub fn new() -> Colors {
        Colors {
            // converting all of these from 0-255 values to ratios
            background: [132.0 / 255.0, 121.0 / 255.0, 115.0 / 255.0, 1.0],
            slot_color: [132.0 / 255.0, 255.0 / 255.0, 115.0 / 255.0, 1.0],
        }
    }
}

pub struct GameConfig {
    board_size: [u64; 2],
}

impl GameConfig {
    pub fn default() -> GameConfig {
        GameConfig { board_size: [4, 5] }
    }
}

pub struct BoardConfig {
    // visual settings
    width: f64,
    height: f64,
    line_width: f64,
}

impl BoardConfig {
    pub fn default() -> BoardConfig {
        BoardConfig {
            width: 640.0,
            height: 480.0,
            line_width: 10.0,
        }
    }
}

pub struct BoardView {
    colors: Colors,
    game_config: GameConfig,
    board_config: BoardConfig,
}

impl BoardView {
    pub fn new() -> BoardView {
        BoardView {
            colors: Colors::new(),
            game_config: GameConfig::default(),
            board_config: BoardConfig::default(),
        }
    }

    pub fn draw<G: Graphics>(&self, context: Context, graphics: &mut G) {
        // clear the board
        clear([1.0; 4], graphics);

        // draw background of board
        rectangle(
            self.colors.background,
            self.board_rect(),
            context.transform,
            graphics,
        );

        // draw individual rectangles for the board slots (for pieces)
        for y in 0..self.game_config.board_size[1] {
            for x in 0..self.game_config.board_size[0] {
                rectangle(
                    self.colors.slot_color,
                    self.piece_rect(x, y),
                    context.transform,
                    graphics,
                );
            }
        }
    }

    pub fn set_size(&mut self, width: f64, height: f64) {
        self.board_config.width = width;
        self.board_config.height = height;
    }

    fn board_rect(&self) -> [f64; 4] {
        [0.0, 0.0, self.calc_width(), self.calc_height()]
    }

    fn calc_width(&self) -> f64 {
        self.board_config.width
    }

    fn calc_height(&self) -> f64 {
        self.board_config.height
    }

    fn piece_rect(&self, x: u64, y: u64) -> [f64; 4] {
        // 640 / 4
        let piece_width = (self.board_config.width
            - ((self.game_config.board_size[0] + 1) as f64 * self.board_config.line_width))
            / self.game_config.board_size[0] as f64;
        let piece_height = (self.board_config.height
            - ((self.game_config.board_size[1] + 1) as f64 * self.board_config.line_width))
            / self.game_config.board_size[1] as f64;

        // xW + xL + L =  x(w+L) + L
        let piece_pos_x =
            x as f64 * (piece_width + self.board_config.line_width) + self.board_config.line_width;
        let piece_pos_y =
            y as f64 * (piece_height + self.board_config.line_width) + self.board_config.line_width;

        [piece_pos_x, piece_pos_y, piece_width, piece_height]
    }
}
