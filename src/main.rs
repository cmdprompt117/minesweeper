pub(crate) mod logic;
pub(crate) mod saves;

use saves::Save;
use logic::MinesweeperGame;

use crossterm::{
    cursor::{
        MoveTo, SetCursorStyle, Hide, Show
    },
    event::{self, Event, KeyCode, KeyEventKind},
    execute
};

use std::time::Duration;
use std::io::Write;
use std::fs;

fn do_splash_text() {
    //? Shoutout Patrick Gillespie: https://patorjk.com/software/taag
    execute!(std::io::stdout(), MoveTo(0, 0)).ok();
    print!("{}[2J", 27 as char);
    print!(" _____    _____               _____                           \r\n");
    print!("| | | |  |_   _|___ ___ _____|   __|_ _ _ ___ ___ ___ ___ ___ \r\n");
    print!("|-   -|    | | | -_|  _|     |__   | | | | -_| -_| . | -_|  _|\r\n");
    print!("|_|_|_|    |_| |___|_| |_|_|_|_____|_____|___|___|  _|___|_|  \r\n");
    print!("                                                 |_|          \r\n\r\n");

    print!("1. Beginner (9x9, 10 mines)\r\n");
    print!("2. Intermediate (16x16, 40 mines)\r\n");
    print!("3. Expert (30x16, 99 mines)\r\n");
    print!("4. Custom\r\n");
    print!("5. Exit\r\n");

    let save = Save::read_save();
    print!("\x1b[0;90m\r\nGames Played: {}\r\nGames Won: {}\r\nWin %: {}\r\nMinutes played: {}\r\nClicks: {}\x1b[0m\r\n", save.g_played, save.g_won, (save.g_won as f32 / save.g_played as f32) * 100., save.total_playtime / 60, save.total_clicks);
}

fn main() -> Result<(), std::io::Error> {
    // Terminal setup
    execute!(std::io::stdout(), SetCursorStyle::SteadyBlock).ok();
    execute!(std::io::stdout(), Hide).ok();
    // Check for save file and make sure it exists
    let save_path = std::env::current_exe().unwrap().parent().unwrap().to_str().unwrap().to_owned();
    if !fs::exists(format!("{}\\save.json", save_path)).unwrap() {
        match fs::write(format!("{}\\save.json", save_path), 
        "{\"g_played\": 0, \"g_won\": 0, \"total_playtime\": 0, \"total_clicks\": 0,
        \"inner_fg\": \"37\", \"inner_bg\": \"100\", \"border_fg\": \"37\", \"border_bg\": \"40\",
        \"inner_highlight\": \"97\", \"flag_char\": \"󰈿\", \"mine_char\": \"󰷚\", \"tile_char\": \"󰆢\",
        \"m_count_fg\": [\"34\", \"32\", \"31\", \"35\", \"33\", \"36\", \"37\", \"30\"],
        \"gamemode\": 0}"
    )   {
            Ok(_) => {}
            Err(e) => {
                print!("Error creating save file: {}\r\n", e);
                return Ok(())
            }
        }
    }
    // Show start text and begin input loop
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
                                print!("\r\n> Width: "); std::io::stdout().flush()?;
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
                                    print!("\r\nX Error while reading input\r\n");
                                    print!("{:?}\r\n{:?}\r\n{:?}\r\n\r\n", width_n, height_n, mines_n);
                                    continue;
                                }
                                if width_n.clone().unwrap() < 0 || height_n.clone().unwrap() < 0 || mines_n.clone().unwrap() < 0 {
                                    print!("\r\nX Please enter valid positive numbers\r\n");
                                    continue;
                                }
                                // Check (by numerical constraints) if it is valid
                                let space_n = width_n.clone().unwrap() * height_n.clone().unwrap();
                                if mines_n.clone().unwrap() >= space_n - 1 {
                                    print!("\r\nX Too many mines for the given space count ({} mines in {} spaces)\r\n", mines_n.clone().unwrap(), space_n);
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
    execute!(std::io::stdout(), Show).ok();
    print!("{}[2J", 27 as char);
    Ok(())
}
