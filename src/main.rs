use crossterm::{
    cursor::{
        MoveTo, SetCursorStyle, Hide, Show
    },
    event::{self, Event, KeyCode, KeyEventKind},
    execute
};
use rand::Rng;

use std::time::{Duration, Instant};
use std::io::Write;

///
/// Struct that acts as a game of minesweeper. Created / managed by the TUI
/// 
struct MinesweeperGame {
    // Info
    x: i16,           // Current x position
    y: i16,           // Current y position
    width: i16,       // Board width 
    height: i16,      // Board height
    m_count: i16,     // Number of mines on the board
    f_count: i16,     // Number of flags on the board
    state: MSGState,  // Whether or not the game is over
    reset: bool,      // Whether or not to reset the game
    time: Instant,    // Represents the instant that the game started, for getting game length

    // Visual
    flag_char: char,
    mine_char: char,
    tile_char: char,

    // Maps
    mine_map: Vec<Vec<i16>>,      // 0 = no mine, 1 = mine
    flag_map: Vec<Vec<i16>>,      // 0 = no flag, 1 = flag
    m_count_map: Vec<Vec<i16>>,   // Each space has the # of mines around it
    uncovered_map: Vec<Vec<i16>>, // 0 = covered, 1 = uncovered. Uncovered tiles cannot be flagged.
}

#[derive(PartialEq)]
enum MSGState {
    Starting,
    Running,
    Win,
    Loss,
    Done
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
            &MSGState::Done => {
                write!(f, "Done")
            }
        }
    }
}

// Initialization
impl MinesweeperGame {
    ///
    /// Creates a new instance of the game
    /// 
    fn new(width: i16, height: i16, m_count: i16) -> MinesweeperGame {
        MinesweeperGame {
            x: 0,
            y: 0,
            width: width,
            height: height,
            m_count: m_count,
            f_count: 0,
            state: MSGState::Starting,
            reset: false,
            time: Instant::now(),

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
    fn get_mine_count(&mut self, x: i16, y: i16) -> i16 {
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
    fn _print_board_mine_map(&self) {
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
    fn _print_board_m_count_map(&self) {
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
        println!("q - check | w - flag | r - reset | m - menu");
        println!("FLAGS LEFT: {}", self.m_count);
        print!("╔");
        for _ in 0..(self.width*3) {
            print!("═");
        }
        print!("╗\n");
        for _ in 0..self.height {
            print!("║");
            for _ in 0..(self.width) {
                print!("\x1b[0;37;100m[{}]\x1b[0m", self.tile_char);
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
    fn print_colored_count(&self, mine_count: i16) {
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
    ///
    /// Used to visually update the colors of an entire square after checking
    /// 
    fn visual_update_space(&self, x: i16, y: i16, mine_count: i16) {
        // 0. Get the canon position
        let pos = self.get_canon_pos(x, y);
        // 1. Move to the character before it on the x-axis
        execute!(std::io::stdout(), MoveTo((pos.0 - 1) as u16, (pos.1) as u16)).ok();
        // 2. Print space info based on mine count
        if mine_count == 0 {
            // Empty space
            print!("\x1b[0;30m[ ]\x1b[0m");
        } else if mine_count == -1 {
            // Mine
            print!("\x1b[0;30;100m[{}]\x1b[0m", self.mine_char);
        } else if mine_count == -2 {
            // Flag
            print!("\x1b[0;37;100m[\x1b[0;100m{}\x1b[0;37;100m]\x1b[0m", self.flag_char);
        } else {
            // Space with mine count
            print!("\x1b[0;30m[\x1b[0m");
            self.print_colored_count(mine_count);
            print!("\x1b[0;30m]\x1b[0m");
        }
    }
    ///
    /// Shows all of the mine locations. Used for showing mines after a loss
    /// 
    fn show_mines(&self) {
        for i in 0..self.height {
            for j in 0..self.width {
                if self.mine_map[i as usize][j as usize] == 1 {
                    if self.flag_map[i as usize][j as usize] != 1 {
                        self.visual_update_space(j, i, -1);
                    }
                }
            }
        }
    }
    ///
    /// Update the "mines left counter" when a flag is placed
    ///
    fn visual_update_f_count(&self) {
        // Jump to where it is printed and update it
        execute!(std::io::stdout(), MoveTo(0, 1)).ok();
        print!("FLAGS LEFT: {}     ", self.m_count - self.f_count);
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
            KeyCode::Char('q') => {
                // Generate the board, update the game state, and check
                self.populate_mine_map();
                self.populate_m_count_map();
                self.state = MSGState::Running;
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
            KeyCode::Char('q') => {
                if self.state != MSGState::Win && self.state != MSGState::Loss {
                    // Chord
                    if self.flag_map[self.y as usize][self.x as usize] != 1 {
                        self.chord();
                    }
                    // Check for win condition
                    self.check_win_condition();
                }
            }
            KeyCode::Char('w') => {
                if self.state != MSGState::Win && self.state != MSGState::Loss {
                    // Flag
                    if self.uncovered_map[self.y as usize][self.x as usize] == 0 {
                        if self.flag_map[self.y as usize][self.x as usize] == 0 && self.f_count < (self.m_count) {
                            self.flag_map[self.y as usize][self.x as usize] = 1;
                            print!("\x1b[0;100m{}\x1b[0m", self.flag_char);
                            self.f_count += 1;
                            self.visual_update_f_count();
                            self.position_cursor(self.x, self.y);
                        } else if self.flag_map[self.y as usize][self.x as usize] == 1 {
                            self.flag_map[self.y as usize][self.x as usize] = 0;
                            print!("\x1b[0;37;100m{}\x1b[0m", self.tile_char);
                            self.f_count -= 1;
                            self.visual_update_f_count();
                            self.position_cursor(self.x, self.y);
                        }
                    }
                }
            }
            KeyCode::Char('r') => {
                // Reset the game
                self.reset = true;
                self.state = MSGState::Done;
            }
            KeyCode::Char('m') => {
                // Quit to main menu
                self.state = MSGState::Done;
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
            execute!(std::io::stdout(), MoveTo(0, (self.height + 4) as u16)).ok();
            execute!(std::io::stdout(), Hide).ok();
            println!("Sorry! You lose.");
            println!("Game time: {}s", self.time.elapsed().as_secs());
            self.show_mines();
            return;
        }
        // If there is not a mine, check the mine count on the current space
        let current_mine_count = self.m_count_map[self.y as usize][self.x as usize];
        // Mark the uncovered map so that we know we have checked this spot already
        self.uncovered_map[self.y as usize][self.x as usize] = 1;
        // 1. If the mine count != 0, show mine count
        if current_mine_count != 0 {
            self.visual_update_space(self.x, self.y, current_mine_count);
            self.position_cursor(self.x, self.y);
        }
        // 2. If the mine count == 0, show mine count and check the surrounding spaces as well
        else if current_mine_count == 0 {
            self.visual_update_space(self.x, self.y, current_mine_count);
            self.position_cursor(self.x, self.y);
            // Get surrounding spaces
            let surrounding = self.get_surrounding(self.x, self.y);
            let mut to_check: Vec<(i16, i16)> = vec![];
            for space in surrounding {
                if self.uncovered_map[space.1 as usize][space.0 as usize] != 1 {
                    if self.m_count_map[space.1 as usize][space.0 as usize] == 0 {
                        self.visual_update_space(self.x, self.y, current_mine_count);
                        self.position_cursor(space.0, space.1);
                        to_check.push((space.0, space.1));
                    } else {
                        self.position_cursor(space.0, space.1);
                        self.visual_update_space(space.0, space.1, self.m_count_map[space.1 as usize][space.0 as usize]);
                        self.position_cursor(self.x, self.y);
                    }
                    self.uncovered_map[space.1 as usize][space.0 as usize] = 1;
                }
            }
            // If we found any zeroes, check them as well
            for space in to_check {
                self.x = space.0;
                self.y = space.1;
                self.check();
            }
        }
        self.position_cursor(temp_x, temp_y);
        self.x = temp_x;
        self.y = temp_y;
    }
    ///
    /// Handle the chording action
    /// 
    fn chord(&mut self) {
        // If we are trying to chord on an unchecked space, jk jk, just check
        if self.uncovered_map[self.y as usize][self.x as usize] == 0 {
            self.check();
            return;
        }
        // Get all the surrounding and make a list of those which are flagged
        let surrounding = self.get_surrounding(self.x, self.y);
        let mut num_flagged: i16 = 0;
        let mut flagged: Vec<(i16, i16)> = vec![];
        for space in &surrounding {
            if self.flag_map[space.1 as usize][space.0 as usize] == 1 {
                num_flagged += 1;
                flagged.push(*space);
            }
        }
        let temp_x = self.x;
        let temp_y = self.y;
        // If the number of flags matches the number of surrounding mines, we can chord.
        if num_flagged == self.m_count_map[self.y as usize][self.x as usize] {
            for space in surrounding {
                if !flagged.contains(&space) {
                    self.x = space.0;
                    self.y = space.1;
                    self.check();
                }
            }
        }
        // Reset to initial position
        self.x = temp_x;
        self.y = temp_y;
        self.position_cursor(self.x, self.y);
    }
    ///
    /// Gets the surrounding spaces of a given coordinate as a `Vec<(i16, i16)>`
    /// 
    fn get_surrounding(&self, x: i16, y: i16) -> Vec<(i16, i16)> {
        // TODO make this more efficient?
        let mut surroundings: Vec<(i16, i16)> = vec![];
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
    fn position_cursor(&self, x: i16, y: i16) {
        let coord = self.get_canon_pos(x, y);
        execute!(std::io::stdout(), MoveTo(coord.0 as u16, coord.1 as u16)).ok();
    }
    ///
    /// Gets the cursor location that position_cursor will place the cursor at.
    /// This is split into a different function so it can be used also to fix background colors on space check
    /// 
    fn get_canon_pos(&self, x: i16, y: i16) -> (i16, i16) {
        return ((3 * x) + 2, y + 3);
    }
    ///
    /// Check win condition after clearing a space
    /// 
    fn check_win_condition(&mut self) {
        // Win condition is defined as:
        // Every position that does NOT have a mine is checked
        let mut has_won: bool = true;
        for i in 0..self.height {
            for j in 0..self.width {
                if self.mine_map[i as usize][j as usize] == 0 {
                    if self.uncovered_map[i as usize][j as usize] != 1 {
                        has_won = false;
                    }
                }
            }
        }
        // If we got all the way through the maps and has_won is still true, we won!
        if has_won {
            self.state = MSGState::Win;
            // Update the board to have flags over the remaining mines
            for i in 0..self.height {
                for j in 0..self.width {
                    if self.mine_map[i as usize][j as usize] == 1 {
                        if self.flag_map[i as usize][j as usize] != 1 {
                            self.visual_update_space(j, i, -2);
                        }
                    }
                }
            }
            self.f_count = self.m_count;
            self.visual_update_f_count();
            // Display win message
            // TODO reconfigure this 4 to be a non-magic number
            execute!(std::io::stdout(), MoveTo(0, (self.height + 4) as u16)).ok();
            execute!(std::io::stdout(), Hide).ok();
            println!("Congrats! You won!");
            println!("Game time: {}s", self.time.elapsed().as_secs());
        }
    }
}

// Game controller
impl MinesweeperGame {
    fn run_game(width: i16, height: i16, mine_count: i16) -> Result<(), std::io::Error> {
        // Create game object
        execute!(std::io::stdout(), Show).ok();
        let mut msg = MinesweeperGame::new(width, height, mine_count);
        // Display board size
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
        // Reset board visually
        execute!(std::io::stdout(), MoveTo(0,0)).ok();
        msg.print_board_normal();
        // We have already checked the position we started at so make sure to check it when we move there
        msg.position_cursor(msg.x, msg.y);
        msg.check();
        // Main game loop
        while msg.state != MSGState::Done {
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
        // Reset if need be
        if msg.reset {
            MinesweeperGame::run_game(width, height, mine_count)?;
        }
        Ok(())
    }
}

fn do_splash_text() {
    //? Shoutout Patrick Gillespie: https://patorjk.com/software/taag
    execute!(std::io::stdout(), MoveTo(0, 0)).ok();
    print!("{}[2J", 27 as char);
    println!(" _____    _____               _____                           ");
    println!("| | | |  |_   _|___ ___ _____|   __|_ _ _ ___ ___ ___ ___ ___ ");
    println!("|-   -|    | | | -_|  _|     |__   | | | | -_| -_| . | -_|  _|");
    println!("|_|_|_|    |_| |___|_| |_|_|_|_____|_____|___|___|  _|___|_|  ");
    println!("                                                 |_|          \n");

    println!("1. Beginner (9x9, 10 mines)");
    println!("2. Intermediate (16x16, 40 mines)");
    println!("3. Expert (30x16, 99 mines)");
    println!("4. Custom");
    println!("5. Exit");
}

fn main() -> Result<(), std::io::Error> {
    // Terminal setup
    execute!(std::io::stdout(), SetCursorStyle::SteadyBlock).ok();
    execute!(std::io::stdout(), Hide).ok();

    do_splash_text();

    loop {
        if event::poll(Duration::from_millis(500))? {
            match event::read().unwrap() {
                Event::Key(key_event) => {
                    if key_event.kind == KeyEventKind::Press {
                        match key_event.code {
                            KeyCode::Char('1') => {
                                MinesweeperGame::run_game(9, 9, 10)?;
                            }
                            KeyCode::Char('2') => {
                                MinesweeperGame::run_game(16, 16, 40)?;
                            }
                            KeyCode::Char('3') => {
                                MinesweeperGame::run_game(30, 16, 99)?;
                            }
                            KeyCode::Char('4') => {
                                execute!(std::io::stdout(), Show).ok();
                                // Get user input
                                let mut width: String = String::new();
                                let mut height: String = String::new();
                                let mut mines: String = String::new();
                                print!("\n> Width: "); std::io::stdout().flush()?;
                                std::io::stdin().read_line(&mut width)?;
                                print!("> Height: "); std::io::stdout().flush()?;
                                std::io::stdin().read_line(&mut height)?;
                                print!("> Mines: "); std::io::stdout().flush()?;
                                std::io::stdin().read_line(&mut mines)?;
                                // Check if it is valid
                                let width_n = width.trim().parse::<i16>();
                                let height_n = height.trim().parse::<i16>();
                                let mines_n = mines.trim().parse::<i16>();
                                if width_n.is_err() || height_n.is_err() || mines_n.is_err() {
                                    println!("\nX Error while reading input");
                                    println!("{:?}\n{:?}\n{:?}\n", width_n, height_n, mines_n);
                                    continue;
                                }
                                if width_n.clone().unwrap() < 0 || height_n.clone().unwrap() < 0 || mines_n.clone().unwrap() < 0 {
                                    println!("\nX Please enter valid positive numbers");
                                    continue;
                                }
                                // Check (by numerical constraints) if it is valid
                                let space_n = width_n.clone().unwrap() * height_n.clone().unwrap();
                                if mines_n.clone().unwrap() >= space_n - 1 {
                                    println!("\nX Too many mines for the given space count ({} mines in {} spaces)", mines_n.clone().unwrap(), space_n);
                                    continue;
                                }

                                // If valid, run the game
                                MinesweeperGame::run_game(width_n.unwrap(), height_n.unwrap(), mines_n.unwrap())?;
                            }
                            KeyCode::Char('5') => {
                                break;
                            }
                            _ => {}
                        }
                        do_splash_text();
                    }
                }
                _ => {}
            }
        }
    }
    execute!(std::io::stdout(), MoveTo(0,0)).ok();
    print!("{}[2J", 27 as char);
    Ok(())
}