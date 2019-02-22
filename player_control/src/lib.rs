extern crate tcod;

use tcod::input::{Key, KeyCode};

use game_state::{GameState, PlayerState};

pub fn player_control_system(input_state: Key, game_state: &mut GameState) {
    match &game_state.player_state {
        PlayerState::MovingCursor => handle_moving_cursor(input_state, game_state),
        PlayerState::UnitSelected => {}
    }
}

fn handle_moving_cursor(input_state: Key, game_state: &mut GameState) {
    match input_state {
        Key { code: KeyCode::Up, .. } => move_cursor(0, -1, game_state),
        Key { code: KeyCode::Down, .. } => move_cursor(0, 1, game_state),
        Key { code: KeyCode::Left, .. } => move_cursor(-1, 0, game_state),
        Key { code: KeyCode::Right, .. } => move_cursor(1, 0, game_state),
        Key { code: KeyCode::Enter, .. } => select_tile(game_state),
        _ => {},
    }
}

fn move_cursor(dx: i32, dy: i32, game_state: &mut GameState) {
    let map_width = game_state.map.len() as i32;
    let map_height = game_state.map[0].len() as i32;
    for y in 0..map_height {
        for x in 0..map_width {
            let new_x = x + dx;
            let new_y = y + dy;

            if new_x < 0 || new_y < 0 || new_x == map_width || new_y == map_height {
                continue
            }

            if game_state.map[x as usize][y as usize].cursored {
                game_state.map[x as usize][y as usize].cursored = false;
                game_state.map[new_x as usize][new_y as usize].cursored = true;
                return;
            }
        }
    }
}

fn select_tile(game_state: &mut GameState) {
}