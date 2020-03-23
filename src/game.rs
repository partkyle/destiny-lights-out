use piston_window::{clear, rectangle, Button, Context, GenericEvent, Graphics, MouseButton};

pub enum GameState {
    Running,
    Quit,
}

pub struct Colors {
    background: [f32; 4],
    slot_color: [f32; 4],
}

impl Colors {
    pub fn new() -> Colors {
        Colors {
            // converting all of these from 0-255 values to ratios
            background: [132.0 / 255.0, 121.0 / 255.0, 115.0 / 255.0, 1.0],
            // using white to indicate that nothing is being drawn
            slot_color: [255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 1.0],
        }
    }
}

#[derive(Clone, Copy)]
pub struct GameConfig {
    board_size: [usize; 2],
}

impl GameConfig {
    pub fn default() -> GameConfig {
        GameConfig { board_size: [3, 3] }
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum State {
    Red,
    Green,
    Blue,
}

impl State {
    pub fn color(&self) -> [f32; 4] {
        match *self {
            State::Red => [1.0, 0.5, 0.5, 1.0],
            State::Green => [0.5, 1.0, 0.5, 1.0],
            State::Blue => [0.5, 0.5, 1.0, 1.0],
        }
    }

    pub fn next(&self) -> State {
        match *self {
            State::Red => State::Green,
            State::Green => State::Blue,
            State::Blue => State::Red,
        }
    }
}

#[derive(Debug)]
pub struct Piece {
    position: [usize; 2], // (x,y)
    rect: [f64; 4],
}

impl Piece {
    pub fn new(position: [usize; 2], rect: [f64; 4]) -> Piece {
        Piece {
            position: position,
            rect: rect,
        }
    }
}

pub struct Game {
    colors: Colors,
    game_config: GameConfig,
    board_config: BoardConfig,
    board: Vec<Vec<State>>,

    board_rectangles: Vec<Piece>,

    cursor: [f64; 2],
}

impl Game {
    pub fn new() -> Game {
        let game_config = GameConfig::default();

        // TODO: have the boards start in a random state.
        // are all states solvable?
        let mut board =
            vec![vec![State::Red; game_config.board_size[1]]; game_config.board_size[0]];

        // Archer's Line
        // https://www.youtube.com/watch?time_continue=338&v=9qoTcHQXbDc&feature=emb_logo
        board[0][1] = State::Green;

        let mut game = Game {
            colors: Colors::new(),
            game_config: game_config,
            board_config: BoardConfig::default(),
            board: board,
            board_rectangles: vec![],
            cursor: [0.0; 2],
        };

        game.init();

        game
    }

    fn init(&mut self) {
        self.set_size(self.board_config.width, self.board_config.height);
    }

    pub fn handle_event<E: GenericEvent>(&mut self, event: &E) -> GameState {
        // a bit of an odd behavior here
        // there is no cursor information on the button press, so we need to store the position
        // so we can reference it later.
        //
        // This means that if you click off the game, and click back on, you will get a mouse event,
        // but the cursor position with be (0,0). This is going to work for this game due to the padding,
        // but I should figure out what the intended behavior is here.
        if let Some(pos) = event.mouse_cursor_args() {
            self.cursor = pos;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            self.handle_click(self.cursor)
        }

        // TODO: is the events the best place to handling winning logic?

        if self.is_won() {
            return GameState::Quit;
        }

        GameState::Running
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

        for piece in self.board_rectangles.iter() {
            rectangle(
                self.colors.slot_color,
                piece.rect,
                context.transform,
                graphics,
            );
        }

        for piece in self.board_rectangles.iter() {
            rectangle(
                self.state(piece.position).color(),
                piece.rect,
                context.transform,
                graphics,
            );
        }
    }

    pub fn set_size(&mut self, width: f64, height: f64) {
        self.board_config.width = width;
        self.board_config.height = height;

        self.board_rectangles = self.create_board_rectangles();
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

    fn create_board_rectangles(&self) -> Vec<Piece> {
        let mut pieces = vec![];
        for y in 0..self.game_config.board_size[1] {
            for x in 0..self.game_config.board_size[0] {
                let position = [x, y];
                let rect = self.piece_rect(x, y);
                pieces.push(Piece::new(position, rect));
            }
        }

        pieces
    }

    fn piece_rect(&self, x: usize, y: usize) -> [f64; 4] {
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

    fn is_won(&self) -> bool {
        // TODO: look at other implementations. I just felt like learning how to use a hashmap.

        use std::collections::HashMap;

        let all_colors = [State::Red, State::Green, State::Blue];
        let board_size = self.game_config.board_size[0] * self.game_config.board_size[1];

        let mut counts = HashMap::new();
        for &c in all_colors.iter() {
            counts.insert(c, 0usize);
        }

        for row in self.board.iter() {
            for &state in row.iter() {
                let count = match counts.get(&state) {
                    None => 1,
                    Some(count) => count + 1,
                };
                counts.insert(state, count);
            }
        }

        let mut result = false;
        for c in all_colors.iter() {
            // TODO: figuring out the references is easy when using the compiler
            // but is there a better way to get to know them?
            result |= counts.get(c) == Some(&board_size);
        }

        result
    }

    fn state(&self, position: [usize; 2]) -> &State {
        let (x, y) = (position[0], position[1]);
        &self.board[x][y]
    }

    fn set_state(&mut self, position: [usize; 2]) {
        let (x, y) = (position[0], position[1]);
        self.board[x][y] = self.board[x][y].next();
    }

    fn handle_click(&mut self, position: [f64; 2]) {
        println!("x: {}, y: {}", position[0], position[1]);

        // check if the click collides with a rectangle
        if let Some(piece) = self.find_rect_at_position(position) {
            println!("gottem boys: {:?}", piece);

            // TODO: figure out why I can't move here. I should be able to call methods
            // within methods, and I don't know what I'm doing wrong here.
            for &position in get_affected_cells(self.game_config, piece.position).iter() {
                self.set_state(position);
            }
        }
    }

    fn find_rect_at_position(&self, position: [f64; 2]) -> Option<&Piece> {
        // rectangle collision
        // there is probably a method in Piston I can use so I don't have to implement it
        self.board_rectangles.iter().find(|&r| {
            position[0] > r.rect[0]
                && position[1] > r.rect[1]
                && position[0] < r.rect[0] + r.rect[2]
                && position[1] < r.rect[1] + r.rect[3]
        })
    }
}

fn get_affected_cells(game_config: GameConfig, position: [usize; 2]) -> Vec<[usize; 2]> {
    let mut result = vec![];

    // the clicked cell is affected
    result.push(position);

    // get all values of x with the same y posistion
    for x in 0..game_config.board_size[0] {
        // this is already accounted for above
        if x != position[0] {
            result.push([x, position[1]])
        }
    }

    // get all values of y with the same x posistion
    for y in 0..game_config.board_size[1] {
        // this is already accounted for above
        if y != position[1] {
            result.push([position[0], y])
        }
    }

    result
}
