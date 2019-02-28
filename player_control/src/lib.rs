extern crate tcod;

use tcod::input::{Key, KeyCode};

use game_state::{GameState, PlayerState, MenuOption};

pub fn clock_tick_system(game_state: &mut GameState) {
    let turn_ready_for = game_state.charge_times
        .iter()
        .zip(game_state.actors.iter().map(|a| &a.speed))
        .enumerate()
        .filter(|&(_i, (&c, _s))| c >= 100)
        .max_by(|&(_i1, (&c1, &s1)), &(_i2, (&c2, &s2))| {
            if c1 == c2 {
                s1.cmp(&s2)
            } else {
                c1.cmp(&c2)
            }
        })
        .map(|(i, (_c, _s))| i);

    match turn_ready_for {
        Some(index) => game_state.active_actor_index = Some(index),
        None => {
            for i in 0..game_state.charge_times.len() {
                game_state.charge_times[i] += game_state.actors[i].speed;
            }

        },
    }
}

pub fn player_control_system(input_state: Key, game_state: &mut GameState) {
    match game_state.player_state {
        PlayerState::MovingCursor => handle_moving_cursor(input_state, game_state),
        PlayerState::UnitSelected => handle_unit_selected(input_state, game_state),
        PlayerState::MovingActor => handle_moving_actor(input_state, game_state),
        PlayerState::ActorAttacking => handle_actor_attacking(input_state, game_state),
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
    // TODO: Can only select units controlled by the player
    let cursor_x = game_state.cursor.x;
    let cursor_y = game_state.cursor.y;

    let actor = game_state.actors
        .iter_mut()
        .find(|actor| actor.x == cursor_x && actor.y == cursor_y);

    if let Some(actor) = actor {
        actor.selected = true;
        game_state.current_menu = Some(actor.selected_menu.clone());
        game_state.current_menu_option = Some(0);
    } else {
        let tile = &mut game_state.map[game_state.cursor.x as usize][game_state.cursor.y as usize];
        tile.selected = true;
    }

    game_state.player_state = PlayerState::UnitSelected;
}

fn handle_unit_selected(input_state: Key, game_state: &mut GameState) {
    match input_state {
        Key { code: KeyCode::Escape, .. } => deselect_tile(game_state),
        Key { code: KeyCode::Up, .. } => menu_option_up(game_state),
        Key { code: KeyCode::Down, .. } => menu_option_down(game_state),
        Key { code: KeyCode::Enter, .. } => menu_option_select(game_state),
        _ => {}
    }
}

fn deselect_tile(game_state: &mut GameState) {
    let cursor_x = game_state.cursor.x;
    let cursor_y = game_state.cursor.y;

    let actor = game_state.actors
        .iter_mut()
        .find(|actor| actor.x == cursor_x && actor.y == cursor_y);

    if let Some(actor) = actor {
        actor.selected = false;
        game_state.current_menu = None;
        game_state.current_menu_option = None;
    } else {
        let tile = &mut game_state.map[game_state.cursor.x as usize][game_state.cursor.y as usize];
        tile.selected = false;
    }

    game_state.player_state = PlayerState::MovingCursor;
}

fn menu_option_up(game_state: &mut GameState) {
    let current_menu_option = game_state.current_menu_option.take();
    match current_menu_option {
        Some(menu_option) => {
            if menu_option > 0 {
                game_state.current_menu_option = Some(menu_option - 1);
            } else {
                game_state.current_menu_option = Some(menu_option);
            }
        }
        None => {}
    }
}

fn menu_option_down(game_state: &mut GameState) {
    let current_menu_option = game_state.current_menu_option.take();
    match current_menu_option {
        Some(menu_option) => {
            if menu_option < game_state.current_menu.as_ref().unwrap().len() - 1 {
                game_state.current_menu_option = Some(menu_option + 1);
            } else {
                game_state.current_menu_option = Some(menu_option);
            }
        }
        None => {}
    }
}

fn menu_option_select(game_state: &mut GameState) {
    if let Some(current_menu) = &game_state.current_menu {
        let current_menu_option = game_state.current_menu_option.unwrap();
        let menu_option = &current_menu[current_menu_option];
        match menu_option {
            MenuOption::Move => game_state.player_state = PlayerState::MovingActor,
            MenuOption::Attack => game_state.player_state = PlayerState::ActorAttacking,
        }

    }
}

fn handle_moving_actor(input_state: Key, game_state: &mut GameState) {
    match input_state {
        Key { code: KeyCode::Up, .. } => move_cursor(0, -1, game_state),
        Key { code: KeyCode::Down, .. } => move_cursor(0, 1, game_state),
        Key { code: KeyCode::Left, .. } => move_cursor(-1, 0, game_state),
        Key { code: KeyCode::Right, .. } => move_cursor(1, 0, game_state),
        Key { code: KeyCode::Enter, .. } => move_actor(game_state),
        Key { code: KeyCode::Escape, .. } => cancel_actor_action(game_state),
        _ => {},
    }
}

fn move_actor(game_state: &mut GameState) {
    let cursor_x = game_state.cursor.x;
    let cursor_y = game_state.cursor.y;

    let other_actor_under_cursor = game_state.actors
        .iter()
        .filter(|actor| !actor.selected)
        .find(|actor| actor.x == cursor_x && actor.y == cursor_y)
        .is_some();

    let actor = game_state.actors
        .iter_mut()
        .find(|actor| actor.selected)
        .unwrap();

    let cursor_distance_from_actor = (actor.x - cursor_x).abs() + (actor.y - cursor_y).abs();

    if cursor_distance_from_actor <= actor.move_range && !other_actor_under_cursor {
        actor.x = cursor_x;
        actor.y = cursor_y;

        game_state.player_state = PlayerState::UnitSelected;
    }

}

fn cancel_actor_action(game_state: &mut GameState) {
    let actor = game_state.actors
        .iter()
        .find(|actor| actor.selected)
        .unwrap();
    game_state.cursor.x = actor.x;
    game_state.cursor.y = actor.y;

    game_state.player_state = PlayerState::UnitSelected;
}

fn handle_actor_attacking(input_state: Key, game_state: &mut GameState) {
    match input_state {
        Key { code: KeyCode::Up, .. } => move_cursor(0, -1, game_state),
        Key { code: KeyCode::Down, .. } => move_cursor(0, 1, game_state),
        Key { code: KeyCode::Left, .. } => move_cursor(-1, 0, game_state),
        Key { code: KeyCode::Right, .. } => move_cursor(1, 0, game_state),
        Key { code: KeyCode::Enter, .. } => attack(game_state),
        Key { code: KeyCode::Escape, .. } => cancel_actor_action(game_state),
        _ => {},
    }
}

fn attack(game_state: &mut GameState) {
}