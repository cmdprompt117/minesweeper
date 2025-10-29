use crossterm::{
    cursor::{
        MoveTo, SetCursorStyle   
    },
    event::{self, Event, KeyCode, KeyEventKind},
    execute
};
use rand::Rng;

use std::time::Duration;

fn main() -> Result<(), std::io::Error> {
    // Initial state
    let mut x: usize = 0;
    let mut y: usize = 0;
    let width: usize = 10;
    let height: usize = 10;
    let mut map: Vec<Vec<u16>> = vec![vec![0; width]; height];
    let mut checked_map: Vec<Vec<u16>> = vec![vec![0; width]; height];
    let mine_count: u16 = 20;

    // Terminal setup
    execute!(std::io::stdout(), SetCursorStyle::SteadyBlock).ok();

    // Generate mines randomly
    populate_mines(&mut map, mine_count);

    // Print board to the screen
    // _print_board(width, height);
    _print_board_mine_map(width, height, &map);
    // _print_board_mine_count(width, height, &map);

    // Position the cursor
    position_cursor(x, y);

    // Interaction loop
    loop {
        if event::poll(Duration::from_millis(250))? {
            match event::read().unwrap() {
                Event::Key(key_event) => {
                    if key_event.kind == KeyEventKind::Press {
                        match key_event.code {
                            KeyCode::Up => {
                                if y != 0 {
                                    y -= 1;
                                    position_cursor(x, y);
                                }
                            }
                            KeyCode::Down => {
                                if y != (height - 1) {
                                    y += 1;
                                    position_cursor(x, y);
                                }
                            }
                            KeyCode::Right => {
                                if x != (width - 1) {
                                    x += 1;
                                    position_cursor(x, y);
                                }
                            }
                            KeyCode::Left => {
                                if x != 0 {
                                    x -= 1;
                                    position_cursor(x, y);
                                }
                            }
                            KeyCode::Char(c) => {
                                match c {
                                    'q' => {
                                        // Quit
                                        print!("{}[2J", 27 as char);
                                        execute!(std::io::stdout(), MoveTo(0, 0)).ok();
                                        break;
                                    }
                                    'a' => {
                                        // Click
                                        let res = get_click_result(x, y, &map, &mut checked_map);
                                        if res == 1 {
                                            // Game is over, you lose!
                                            print!("{}[2J", 27 as char);
                                            execute!(std::io::stdout(), MoveTo(0, 1)).ok();
                                            println!("Game is over. You lose!");
                                            break;
                                        }
                                    }
                                    'd' => {
                                        // Flag
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}

///
/// Prints a blank board with no visual information.
/// Used when starting an actual game to set the initial scene
/// 
fn _print_board(width: usize, height: usize) {
    execute!(std::io::stdout(), MoveTo(0, 0)).ok();
    print!("{}[2J", 27 as char);

    print!("╔");
    for _ in 0..(width*3) {
        print!("═");
    }
    print!("╗\n");
    for _ in 0..height {
        print!("║");
        for _ in 0..(width) {
            print!("[#]");
        }
        print!("║\n");
    }
    print!("╚");
    for _ in 0..(width*3) {
        print!("═");
    }
    print!("╝\n");
}

///
/// Prints the board with a 0 and 1 map of the mines. 
/// Used for visualizing mine generation in testing.
/// 1 - mine, 0 - no mine
/// 
fn _print_board_mine_map(width: usize, height: usize, map: &Vec<Vec<u16>>) {
    execute!(std::io::stdout(), MoveTo(0, 0)).ok();
    print!("{}[2J", 27 as char);

    print!("╔");
    for _ in 0..(width*3) {
        print!("═");
    }
    print!("╗\n");
    for i in 0..height {
        print!("║");
        for j in 0..(width) {
            if map[i][j] == 1 {
                print!("[M]");
            } else {
                print!("[ ]");
            }
        }
        print!("║\n");
    }
    print!("╚");
    for _ in 0..(width*3) {
        print!("═");
    }
    print!("╝\n");
}

///
/// Prints the board with the calculated neighboring mine count of each position.
/// Used for testing the neighboring mine count algorithm
/// 
fn _print_board_mine_count(width: usize, height: usize, map: &Vec<Vec<u16>>) {
    execute!(std::io::stdout(), MoveTo(0, 0)).ok();
    print!("{}[2J", 27 as char);

    print!("╔");
    for _ in 0..(width*3) {
        print!("═");
    }
    print!("╗\n");
    for i in 0..height {
        print!("║");
        for j in 0..(width) {
            if map[i][j] == 1 {
                print!("[M]");
            } else {
                print!("[{}]", get_mine_count(j, i, map));
            }
        }
        print!("║\n");
    }
    print!("╚");
    for _ in 0..(width*3) {
        print!("═");
    }
    print!("╝\n");
}

///
/// Randomly populate board with mines.
/// TODO Get a better algorithm for placing mines
/// 
fn populate_mines(map: &mut Vec<Vec<u16>>, mine_count: u16) {
    let mut rng = rand::rng();
    let mut added: u16 = 0;
    while added < mine_count {
        let rand_y = rng.random_range(0..map.len());
        let rand_x = rng.random_range(0..map[rand_y].len());
        if map[rand_y][rand_x] != 1 {
            map[rand_y][rand_x] = 1;
            added += 1;
        }
    }
}

///
/// Position cursor relative to board position
/// 
fn position_cursor(x: usize, y: usize) {
    let mut new_x: u16 = 2;
    let mut new_y: u16 = 1;

    new_x += 3 * x as u16;
    new_y += y as u16;

    execute!(std::io::stdout(), MoveTo(new_x, new_y)).ok();
}

///
/// Get the mine count at a position on the board
/// 
fn get_mine_count(x: usize, y: usize, map: &Vec<Vec<u16>>) -> u16 {
    let x_left_edge: bool =  if x > 0 { false } else { true };
    let x_right_edge: bool = if x < map[y].len() - 1 { false } else { true };
    let y_top_edge: bool = if y > 0 { false } else { true };
    let y_bottom_edge: bool = if y < map.len() - 1 { false } else { true };

    let mine_count: u16;
    if x_left_edge {
        if y_top_edge {
            mine_count = map[y][x + 1] + map[y + 1][x] + map[y + 1][x + 1];
        } else if y_bottom_edge { 
            mine_count = map[y][x + 1] + map[y - 1][x] + map[y - 1][x + 1];
        } else {
            mine_count = map[y][x + 1] + map[y - 1][x] + map[y - 1][x + 1] + map[y + 1][x] + map[y + 1][x + 1];
        }
    } else if x_right_edge {
        if y_top_edge {
            mine_count = map[y][x - 1] + map[y + 1][x] + map[y + 1][x - 1];
        } else if y_bottom_edge { 
            mine_count = map[y][x - 1] + map[y - 1][x] + map[y - 1][x - 1];
        } else {
            mine_count = map[y][x - 1] + map[y - 1][x] + map[y - 1][x - 1] + map[y + 1][x] + map[y + 1][x - 1];
        }
    } else {
        if y_top_edge {
            mine_count = map[y][x - 1] + map[y][x + 1] + map[y + 1][x] + map[y + 1][x - 1] + map[y + 1][x + 1];
        } else if y_bottom_edge { 
            mine_count = map[y][x - 1] + map[y][x + 1] + map[y - 1][x] + map[y - 1][x - 1] + map[y - 1][x + 1];
        } else {
            mine_count = map[y][x - 1] + map[y][x + 1] + map[y - 1][x] + map[y - 1][x - 1] + map[y - 1][x + 1] + map[y + 1][x] + map[y + 1][x - 1] + map[y + 1][x + 1];
        }
    }
    return mine_count;
}

///
/// Performs the following actions after a click based on what is at the current position:
/// 1. If the position is a mine, you lose the game
/// 2. If the position is not a mine, runs a visual update to reveal mine count at that location
/// 
fn get_click_result(x: usize, y: usize, map: &Vec<Vec<u16>>, checked_map: &mut Vec<Vec<u16>>) -> u16 {
    if map[y][x] == 1 {
        return 1;    
    } else {
        visual_update(x, y, map, checked_map);
        position_cursor(x, y);
        return 0;
    }
}

///
/// Visually updates the space surrounding the current position, based on the mine count
/// 
fn visual_update(x: usize, y: usize, map: &Vec<Vec<u16>>, checked_map: &mut Vec<Vec<u16>>) {
    checked_map[x][y] = 1;
    let mine_count = get_mine_count(x, y, map);
    if mine_count != 0 {
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
        position_cursor(x, y); // Reposition after printing to re-align properly
    } else {
        print!(" ");
        position_cursor(x, y);
        // Update surrounding positions
        let x_left_edge: bool =  if x > 0 { false } else { true };
        let x_right_edge: bool = if x < map[y].len() - 1 { false } else { true };
        let y_top_edge: bool = if y > 0 { false } else { true };
        let y_bottom_edge: bool = if y < map.len() - 1 { false } else { true };

        let surrounding: Vec<(usize, usize)>;
        if x_left_edge {
            if y_top_edge {
                surrounding = vec![(x + 1, y), (x, y + 1), (x + 1, y + 1)];
            } else if y_bottom_edge { 
                surrounding = vec![(x + 1, y), (x, y - 1), (x + 1, y - 1)];
            } else {
                surrounding = vec![(x + 1, y), (x, y - 1), (x + 1, y - 1), (x, y + 1), (x + 1, y + 1)];
            }
        } else if x_right_edge {
            if y_top_edge {
                surrounding = vec![(x - 1, y), (x, y + 1), (x - 1, y + 1)];
            } else if y_bottom_edge { 
                surrounding = vec![(x - 1, y), (x, y - 1), (x - 1, y - 1)];
            } else {
                surrounding = vec![(x - 1, y), (x, y - 1), (x - 1, y - 1), (x, y + 1), (x - 1, y + 1)];
            }
        } else {
            if y_top_edge {
                surrounding = vec![(x - 1, y), (x + 1, y), (x, y + 1), (x - 1, y + 1), (x + 1, y + 1)];
            } else if y_bottom_edge { 
                surrounding = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x - 1, y - 1), (x + 1, y - 1)];
            } else {
                surrounding = vec![(x - 1, y), (x + 1, y), (x - 1, y - 1), (x, y - 1), (x + 1, y - 1), (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)];
            }
        }

        for pos in surrounding {
            if checked_map[pos.1][pos.0] != 1 {
                position_cursor(pos.0, pos.1);
                print!("C");
                // checked_map[pos.1][pos.0] = 1;
                // visual_update(pos.0, pos.1, map, checked_map);
            }
        }

        position_cursor(x, y);
    }
}