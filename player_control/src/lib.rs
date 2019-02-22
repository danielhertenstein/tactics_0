extern crate tcod;

use tcod::input::{Key, KeyCode};

use game_state::{Actor, GameState, PlayerState, Tile};

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

    let new_x = game_state.cursor.x + dx;
    let new_y = game_state.cursor.y + dy;

    if new_x < 0 || new_y < 0 || new_x == map_width || new_y == map_height {
        return
    }

    game_state.cursor.x = new_x;
    game_state.cursor.y = new_y;
}

fn select_tile(game_state: &mut GameState) {
    if let Some(actor) = game_state.actors
        .iter()
        .find(|actor| actor.x == game_state.cursor.x && actor.y == game_state.cursor.y) {
        show_actor_menu(actor);
    } else {
        let tile = &game_state.map[game_state.cursor.x as usize][game_state.cursor.y as usize];
        show_tile_menu(tile);
    }

}

fn show_actor_menu(actor: &Actor) {
    println!("Selected an Actor!")
}

fn show_tile_menu(tile: &Tile) {
    println!("Selected a Tile!")
}