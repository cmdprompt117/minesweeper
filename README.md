# minesweeper
Rust implementation of minesweeper on the terminal

## Roadmap

- ~~Save files for data like win ratio, avg. game time per mode, settings~~
- ~~View save information (display by default on menu?)~~
- Settings (color choices, custom flag/mine/tile characters, border chars, etc.)
- QOL modes (check four corners until good start, no guessing, etc.)
- UI overhaul (ratatui?)

### Potential later additions

Any ideas below this header are complete maybes.

- Challenges, levels, XP
- "AI" mode: Start up some games and watch an algorithm solve them. 
- Step-through mode for the algorithm solving to watch how it goes slowly for learning

## Usage

Only tested on Windows 11. Currently no pre-packaged binaries have been generated for this repo, so you will need to compile the project manually with `cargo build`. Any files that save game information will be stored in the directory that the binary is kept in.

## Special Thanks

Shoutout to <https://minesweeper.online/> for being a great minesweeper website and fueling my complete minesweeping addiction. If you want something more fleshed out and have a mouse, definitely check it out.