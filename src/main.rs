use crossterm::{
    cursor::{
        MoveTo, SetCursorStyle   
    },
    event::{self, Event, KeyCode, KeyEventKind},
    execute
};
use rand::Rng;

use std::time::Duration;

///
/// Struct that acts as a game of minesweeper. Created / managed by the TUI
/// 
struct MinesweeperGame {
    // Info
    x: i8,           // Current x position
    y: i8,           // Current y position
    width: i8,       // Board width 
    height: i8,      // Board height
    m_count: i8,     // Number of mines on the board
    state: MSGState, // Whether or not the game is over

    // Visual
    flag_char: char,
    mine_char: char,
    tile_char: char,

    // Maps
    mine_map: Vec<Vec<i8>>,      // 0 = no mine, 1 = mine
    flag_map: Vec<Vec<i8>>,      // 0 = no flag, 1 = flag
    m_count_map: Vec<Vec<i8>>,   // Each space has the # of mines around it
    uncovered_map: Vec<Vec<i8>>, // 0 = covered, 1 = uncovered. Uncovered tiles cannot be flagged.
}

#[derive(PartialEq)]
enum MSGState {
    Starting,
    Running,
    Win,
    Loss
}

use std::fmt::Display;
use std::fmt::Formatter;
impl Display for MSGState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            &MSGState::Starting => {
                write!(f, "Starting")
            }
            &MSGState::Running => {
                write!(f, "Running")
            }
            &MSGState::Loss => {
                write!(f, "Loss")
            }
            &MSGState::Win => {
                write!(f, "Win")
            }
        }
    }
}

// Initialization
impl MinesweeperGame {
    ///
    /// Creates a new instance of the game
    /// 
    fn new(width: i8, height: i8, m_count: i8) -> MinesweeperGame {
        MinesweeperGame {
            x: 0,
            y: 0,
            width: width,
            height: height,
            m_count: m_count,
            state: MSGState::Starting,

            flag_char: '󰈿',
            mine_char: '󰷚',
            tile_char: '󰆢',

            mine_map: vec![vec![0; width as usize]; height as usize],
            flag_map: vec![vec![0; width as usize]; height as usize],
            m_count_map: vec![vec![0; width as usize]; height as usize],
            uncovered_map: vec![vec![0; width as usize]; height as usize],
        }
    }
    ///
    /// Populate the mines on the board by updating `mine_map`
    /// 
    fn populate_mine_map(&mut self) {
        for _ in 0..self.m_count {
            let mut rng = rand::rng();
            loop {
                let rand_y = rng.random_range(0..self.height);
                let rand_x = rng.random_range(0..self.width);
                // !(self.x == rand_x && self.y == rand_y)
                // Ensures that when generating the board we do not put a mine where the player chose to start 
                if self.mine_map[rand_y as usize][rand_x as usize] != 1 && !(self.x == rand_x && self.y == rand_y) {
                    self.mine_map[rand_y as usize][rand_x as usize] = 1;
                    break;
                }
            }
        }
    }
    /// 
    /// Populate the mine counts on the board by updating `m_count_map`
    /// 
    fn populate_m_count_map(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                let mine_count = Self::get_mine_count(self, j, i);
                self.m_count_map[i as usize][j as usize] = mine_count;
            }
        }
    }
    ///
    /// Get the number of mines surrounding the given position
    /// 
    fn get_mine_count(&mut self, x: i8, y: i8) -> i8 {
        let surrounding = vec![
            (x - 1, y - 1), // Top left
            (x, y - 1),     // Top
            (x + 1, y - 1), // Top right
            (x - 1, y),     // Left
            (x + 1, y),     // Right
            (x - 1, y + 1), // Bottom left
            (x, y + 1),     // Bottom
            (x + 1, y + 1)  // Bottom right
        ];
        let mut mine_count = 0;
        for coord in surrounding {
            if coord.0 >= 0 && coord.0 < self.width && coord.1 >= 0 && coord.1 < self.height {
                mine_count += self.mine_map[coord.1 as usize][coord.0 as usize];
            }
        }
        return mine_count;
    }
}

// Visualization
impl MinesweeperGame {
    ///
    /// Prints the board with a map of the mines. 
    /// Used for visualizing mine generation in testing.
    /// "M" = mine, " " = no mine
    /// 
    fn print_board_mine_map(&self) {
        execute!(std::io::stdout(), MoveTo(0, 0)).ok();
        print!("{}[2J", 27 as char);

        print!("╔");
        for _ in 0..(self.width*3) {
            print!("═");
        }
        print!("╗\n");
        for i in 0..self.height {
            print!("║");
            for j in 0..(self.width) {
                if self.mine_map[i as usize][j as usize] == 1 {
                    print!("[{}]", self.mine_char);
                } else {
                    print!("[{}]", self.tile_char);
                }
            }
            print!("║\n");
        }
        print!("╚");
        for _ in 0..(self.width*3) {
            print!("═");
        }
        print!("╝\n");
    }
    ///
    /// Prints the board with the calculated neighboring mine count of each position.
    /// If a position contains a mine, it prints "M" instead.
    /// Used for testing the `get_mine_count` algorithm
    ///
    fn print_board_m_count_map(&self) {
        execute!(std::io::stdout(), MoveTo(0, 0)).ok();
        print!("{}[2J", 27 as char);

        print!("╔");
        for _ in 0..(self.width*3) {
            print!("═");
        }
        print!("╗\n");
        for i in 0..self.height {
            print!("║");
            for j in 0..(self.width) {
                if self.mine_map[i as usize][j as usize] == 1 {
                    print!("[{}]", self.mine_char);
                } else {
                    print!("[{}]", self.m_count_map[i as usize][j as usize]);
                }
            }
            print!("║\n");
        }
        print!("╚");
        for _ in 0..(self.width*3) {
            print!("═");
        }
        print!("╝\n");
    }
    ///
    /// Prints a blank board with no visual information.
    /// Used when starting an actual game to set the initial scene
    /// 
    fn print_board_normal(&self) {
        execute!(std::io::stdout(), MoveTo(0, 0)).ok();
        print!("{}[2J", 27 as char);

        print!("╔");
        for _ in 0..(self.width*3) {
            print!("═");
        }
        print!("╗\n");
        for _ in 0..self.height {
            print!("║");
            for _ in 0..(self.width) {
                print!("\x1b[0;90m[{}]\x1b[0m", self.tile_char);
            }
            print!("║\n");
        }
        print!("╚");
        for _ in 0..(self.width*3) {
            print!("═");
        }
        print!("╝\n");
    }
    ///
    /// Prints the mine count at a position with a color based on the count
    /// 
    fn print_colored_count(&self, mine_count: i8) {
        match mine_count {
            1 => {
                print!("\x1b[1;34m");
            }
            2 => {
                print!("\x1b[1;32m");
            }
            3 => {
                print!("\x1b[1;31m");
            }
            4 => {
                print!("\x1b[1;35m");
            }
            5 => {
                print!("\x1b[1;33m");
            }
            6 => {
                print!("\x1b[1;36m");
            }
            7 => {
                print!("\x1b[1;37m");
            }
            8 => {
                print!("\x1b[1;30m");
            }
            _ => {} // Impossible cases
        }
        print!("{}\x1b[0m", mine_count);
    }
}

// Game logic
impl MinesweeperGame {
    ///
    /// Handle the start of the game, in which the player has to get a check in before the mines can generate, in order to avoid random start losses 
    /// 
    fn handle_start(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Up => {
                if self.y > 0 {
                    self.y -= 1;
                    self.position_cursor(self.x, self.y);
                }
            }
            KeyCode::Down => {
                if self.y < self.height - 1 {
                    self.y += 1;
                    self.position_cursor(self.x, self.y);
                }
            }
            KeyCode::Left => {
                if self.x > 0 {
                    self.x -= 1;
                    self.position_cursor(self.x, self.y);
                }
            }
            KeyCode::Right => {
                if self.x < self.width - 1{
                    self.x += 1;
                    self.position_cursor(self.x, self.y);
                }
            }
            KeyCode::Char('a') => {
                // Generate the board, update the game state, and check
                self.populate_mine_map();
                self.populate_m_count_map();
                self.state = MSGState::Running;
                self.check();
            }
            _ => {}
        }
    }
    ///
    /// Handle user input for things like checking, flagging, movement, etc.
    /// 
    fn handle_input(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Up => {
                if self.y > 0 {
                    self.y -= 1;
                    self.position_cursor(self.x, self.y);
                }
            }
            KeyCode::Down => {
                if self.y < self.height - 1 {
                    self.y += 1;
                    self.position_cursor(self.x, self.y);
                }
            }
            KeyCode::Left => {
                if self.x > 0 {
                    self.x -= 1;
                    self.position_cursor(self.x, self.y);
                }
            }
            KeyCode::Right => {
                if self.x < self.width - 1 {
                    self.x += 1;
                    self.position_cursor(self.x, self.y);
                }
            }
            KeyCode::Char('a') => {
                // Check
                if self.flag_map[self.y as usize][self.x as usize] != 1 {
                    self.check();
                }
            }
            KeyCode::Char('d') => {
                // Flag
                if self.uncovered_map[self.y as usize][self.x as usize] == 0 {
                    if self.flag_map[self.y as usize][self.x as usize] == 0 {
                        self.flag_map[self.y as usize][self.x as usize] = 1;
                        print!("{}", self.flag_char);
                        self.position_cursor(self.x, self.y);
                    } else {
                        self.flag_map[self.y as usize][self.x as usize] = 0;
                        print!("\x1b[0;90m{}\x1b[0m", self.tile_char);
                        self.position_cursor(self.x, self.y);
                    }
                }
            }
            KeyCode::Char('q') => {
                // Quit the game
                execute!(std::io::stdout(), MoveTo(0, 0)).ok();
                print!("{}[2J", 27 as char);
                self.state = MSGState::Loss;
            }
            _ => {}
        }
    }
    ///
    /// Handle the checking action
    /// 
    fn check(&mut self) {
        let temp_x = self.x;
        let temp_y = self.y;
        // See if there is a mine where we checked. If so, we lose.
        if self.mine_map[self.y as usize][self.x as usize] == 1 {
            self.state = MSGState::Loss;
        }
        // If there is not a mine, check the mine count on the current space
        let current_mine_count = self.m_count_map[self.y as usize][self.x as usize];
        // Mark the uncovered map so that we know we have checked this spot already
        self.uncovered_map[self.y as usize][self.x as usize] = 1;
        // 1. If the mine count != 0, show mine count
        if current_mine_count != 0 {
            self.print_colored_count(current_mine_count);
            self.position_cursor(self.x, self.y);
        }
        // 2. If the mine count == 0, show mine count and check the surrounding spaces as well
        else if current_mine_count == 0 {
            print!(" ");
            self.position_cursor(self.x, self.y);
            // Get surrounding spaces
            let surrounding = self.get_surrounding(self.x, self.y);
            for space in surrounding {
                if self.uncovered_map[space.1 as usize][space.0 as usize] != 1 {
                    if self.m_count_map[space.1 as usize][space.0 as usize] == 0 {
                        self.position_cursor(space.0, space.1);
                        print!(" ");
                        self.position_cursor(space.0, space.1);
                        self.x = space.0;
                        self.y = space.1;
                        self.check();
                    } else {
                        self.position_cursor(space.0, space.1);
                        self.print_colored_count(self.m_count_map[space.1 as usize][space.0 as usize]);
                        self.position_cursor(self.x, self.y);
                    }
                }
            }
        }
        self.position_cursor(temp_x, temp_y);
        self.x = temp_x;
        self.y = temp_y;
    }
    ///
    /// Gets the surrounding spaces of a given coordinate as a `Vec<(i8, i8)>`
    /// 
    fn get_surrounding(&self, x: i8, y: i8) -> Vec<(i8, i8)> {
        // TODO make this more efficient?
        let mut surroundings: Vec<(i8, i8)> = vec![];
        // Left space
        if x > 0 {
            surroundings.push((x - 1, y));
        }
        // Right space
        if x < self.width - 1 {
            surroundings.push((x + 1, y));
        }
        // Top space
        if y > 0 {
            surroundings.push((x, y - 1));
        }
        // Bottom space
        if y < self.height - 1 {
            surroundings.push((x, y + 1));
        }
        // Top left
        if x > 0 && y > 0 {
            surroundings.push((x - 1, y - 1));
        }
        // Top right
        if x < self.width - 1 && y > 0 {
            surroundings.push((x + 1, y - 1));
        }
        // Bottom left
        if x > 0 && y < self.height - 1 {
            surroundings.push((x - 1, y + 1));
        }
        // Bottom right
        if x < self.width - 1 && y < self.height - 1 {
            surroundings.push((x + 1, y + 1));
        }
        return surroundings;
    }
    ///
    /// Position cursor relative to board position
    /// 
    fn position_cursor(&self, x: i8, y: i8) {
        let mut new_x: u16 = 2;
        let mut new_y: u16 = 1;

        new_x += 3 * x as u16;
        new_y += y as u16;

        execute!(std::io::stdout(), MoveTo(new_x, new_y)).ok();
    }
}

fn main() -> Result<(), std::io::Error> {
    // Terminal setup
    execute!(std::io::stdout(), SetCursorStyle::SteadyBlock).ok();

    // Create game object and run game
    let height: i8 = 10;
    let width: i8 = 10;
    let mines: i8 = 10;
    let mut msg = MinesweeperGame::new(width, height, mines);

    // Populate the mines
    msg.populate_mine_map();
    msg.populate_m_count_map();

    // Print board to the screen
    msg.print_board_normal();

    // Position the cursor
    msg.position_cursor(msg.x, msg.y);

    // Ensure that the user gets a click in before generating the board
    while msg.state == MSGState::Starting {
        if event::poll(Duration::from_millis(250))? {
            match event::read().unwrap() {
                Event::Key(key_event) => {
                    if key_event.kind == KeyEventKind::Press {
                        msg.handle_start(key_event.code);
                    }
                }
                _ => {}
            }
        }
    }

    // TODO remove these. Debug
    execute!(std::io::stdout(), MoveTo(0,0)).ok();
    print!("{}[2J", 27 as char);
    // msg.print_board_m_count_map();
    // msg.print_board_mine_map();
    msg.print_board_normal();
    // We have already checked the position we started at so make sure to check it when we move there
    msg.position_cursor(msg.x, msg.y);
    msg.check();

    // Main game loop
    while msg.state == MSGState::Running {
        if event::poll(Duration::from_millis(250))? {
            match event::read().unwrap() {
                Event::Key(key_event) => {
                    if key_event.kind == KeyEventKind::Press {
                        msg.handle_input(key_event.code);
                    }
                }
                _ => {}
            }
        }
    }

    execute!(std::io::stdout(), MoveTo(0, 0)).ok();
    print!("{}[2J", 27 as char);
    match msg.state {
        MSGState::Win => {
            println!("Congratulations, you won!");
        }
        MSGState::Loss => {
            println!("Yikes... you lost.");
        }
        _ => {
            // Something has gone terribly wrong to get here
            panic!("Error: Game ended with invalid state: {}", msg.state);
        }
    }

    // TODO show game stats?
    
    Ok(())
}