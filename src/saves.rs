use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Save {
    // Statistics
    pub g_played: u32,       // Number of games played
    pub g_won: u32,          // Number of games won
    pub total_playtime: u64, // Number of seconds of game played
    pub total_clicks: u64,   // Total number of "check" / "chord" actions all time. This one is for fun
    // Settings
    // (ANSI color codes)
    pub border_fg: String,       // Foreground color of map borders
    pub border_bg: String,       // Background color of map borders
    pub inner_fg: String,        // Foreground color of mine character and surrounding brackets
    pub inner_highlight: String, // Foreground color for placed flags and mines exposed after loss
    pub inner_bg: String,        // Background color of inner 
    pub m_count_fg: Vec<String>, // Foreground color for all 8 mine counts (0 = blank)
    // (Characters)
    pub mine_char: String,
    pub flag_char: String,
    pub tile_char: String,
    // (Gamemode)
    // 0 - Vanilla
    // 1 - CMD's QOL
    // 2 - No Guessing
    pub gamemode: u8
}

impl Save {
    ///
    /// Reads save data from the file `save.json`.
    /// 
    pub fn read_save() -> Save {
        // Get file contents
        let save_path = std::env::current_exe().unwrap().parent().unwrap().to_str().unwrap().to_owned();
        let file = fs::read_to_string(format!("{}\\save.json", save_path));
        match file {
            Ok(_) => {}
            Err(e) => { 
                print!("Error while opening save file: {}\r\n", e);
                std::process::exit(1);
            }
        }
        let file_con: Result<Save, _> = serde_json::from_str(file.unwrap().as_str());
        match file_con {
            Ok(s) => {
                return s;
            }
            Err(e) => {
                print!("Error while opening save file: {}\r\n", e);
                std::process::exit(1);
            }
        }
    }
    ///
    /// Updates the stats of the Save object with those collected during the game
    /// 
    pub fn update_save(&mut self, won: bool, playtime: u64, clicks: u64) {
        self.g_played += 1;
        if won {
            self.g_won += 1;
        }
        self.total_playtime += playtime;
        self.total_clicks += clicks;
    }
    ///
    /// Stores the Save data back into the file `save.json`.
    /// 
    pub fn write_save(&mut self) {
        // TODO more error handling? It is a little pointless if the user <ctrl+c>'s 
        // TODO and write errors are few and far between since we make sure the file exists
        let new_save_data = serde_json::to_string(&self);
        match new_save_data {
            Ok(s) => {
                let save_path = std::env::current_exe().unwrap().parent().unwrap().to_str().unwrap().to_owned();
                fs::write(format!("{}\\save.json", save_path), s).ok();
            }
            Err(_) => {
                // Could not write save data
                return;
            }
        }
    }
}