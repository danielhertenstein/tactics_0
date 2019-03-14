extern crate tcod;

use tcod::input::{Key, KeyCode};

use game_state::{GameState, PlayerState, MenuOption, Menu, Actor, Turn};

pub fn player_control_system(input_state: Key, game_state: &mut GameState) {
    match game_state.player_state {
        PlayerState::WaitingForTurn => {},
        PlayerState::TurnReady => handle_start_of_turn(game_state),
        PlayerState::MovingCursor => handle_moving_cursor(input_state, game_state),
        PlayerState::UnitSelected => handle_unit_selected(input_state, game_state),
        PlayerState::MovingActor => handle_moving_actor(input_state, game_state),
        PlayerState::ActorAttacking => handle_actor_attacking(input_state, game_state),
    }
}

fn handle_start_of_turn(game_state: &mut GameState) {
    game_state.turn = Some(Turn::new());
    select_tile(game_state);
}

fn handle_moving_cursor(input_state: Key, game_state: &mut GameState) {
    match input_state {
        Key { code: KeyCode::Up, .. } => move_cursor(0, -1, game_state),
        Key { code: KeyCode::Down, .. } => move_cursor(0, 1, game_state),
        Key { code: KeyCode::Left, .. } => move_cursor(-1, 0, game_state),
        Key { code: KeyCode::Right, .. } => move_cursor(1, 0, game_state),
        Key { code: KeyCode::Enter, .. } => select_tile(game_state),
        // TODO: Might want to bring up game menu if cursor already on active unit
        Key { code: KeyCode::Escape, .. } => return_to_active_unit(game_state),
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
    match game_state.positions
        .iter()
        .position(|position| position == &game_state.cursor) {
        Some(index) => {
            if index == game_state.active_actor_index.unwrap() {
                let actor = &game_state.actors[index];
                if actor.player_controlled {
                    // TODO: I don't like this unwrap
                    game_state.menu = Some(create_battle_menu(actor, game_state.turn.as_ref().unwrap()));
                }
            }
            game_state.player_state = PlayerState::UnitSelected;
        },
        None => {}
    }
}

fn create_battle_menu(actor: &Actor, turn: &Turn) -> Menu {
    let mut options = Vec::new();
    if actor.can_move && !turn.moved {
        options.push(MenuOption::Move);
    }
    if actor.can_act && !turn.acted {
        options.push(MenuOption::Attack);
    }
    options.push(MenuOption::EndTurn);

    Menu {
        options,
        selected_index: 0,
    }
}

fn return_to_active_unit(game_state: &mut GameState) {
    match game_state.active_actor_index {
        Some(index) => {
            let actor = &game_state.actors[index];
            if actor.player_controlled {
                let actor_position = &game_state.positions[index];
                game_state.cursor.x = actor_position.x;
                game_state.cursor.y = actor_position.y;
                select_tile(game_state);
            }
        },
        None => {},
    }
}

fn handle_unit_selected(input_state: Key, game_state: &mut GameState) {
    match input_state {
        Key { code: KeyCode::Escape, .. } => deselect_unit(game_state),
        Key { code: KeyCode::Up, .. } => menu_option_up(game_state),
        Key { code: KeyCode::Down, .. } => menu_option_down(game_state),
        Key { code: KeyCode::Enter, .. } => menu_option_select(game_state),
        _ => {}
    }
}

fn deselect_unit(game_state: &mut GameState) {
    game_state.menu = None;
    game_state.player_state = PlayerState::MovingCursor;
}

fn menu_option_up(game_state: &mut GameState) {
    match game_state.menu.as_mut() {
        Some(menu) => menu.move_up(),
        None => {},
    }
}

fn menu_option_down(game_state: &mut GameState) {
    match game_state.menu.as_mut() {
        Some(menu) => menu.move_down(),
        None => {},
    }
}

fn menu_option_select(game_state: &mut GameState) {
    match &mut game_state.menu {
        Some(menu) => match menu.select() {
            Some(&MenuOption::Move) => game_state.player_state = PlayerState::MovingActor,
            Some(&MenuOption::Attack) => game_state.player_state = PlayerState::ActorAttacking,
            Some(&MenuOption::EndTurn) => end_turn(game_state),
            None => {}
        },
        None => {}
    }
}

fn end_turn(game_state: &mut GameState) {
    if let Some(index) = game_state.active_actor_index {
        let mut new_charge_time = 80;
        match &game_state.menu {
            Some(menu) => {
                if menu.contains(&MenuOption::Move) == false {
                    new_charge_time -= 30;
                }
                if menu.contains(&MenuOption::Attack) == false {
                    new_charge_time -= 50;
                }
            },
            None => {}
        }
        game_state.charge_times[index] = new_charge_time;
        game_state.active_actor_index = None;
        game_state.turn = None;
        deselect_unit(game_state);
        game_state.player_state = PlayerState::WaitingForTurn;
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

    let other_actor_under_cursor = game_state.positions
        .iter()
        .position(|position| position == &game_state.cursor);

    let active_index = game_state.active_actor_index.unwrap();

    match other_actor_under_cursor {
        Some(index) if index == active_index => return,
        _ => {}
    }

    let actor_position = &mut game_state.positions[active_index];
    let cursor_distance_from_actor = (actor_position.x - cursor_x).abs()
        + (actor_position.y - cursor_y).abs();

    let actor = &game_state.actors[active_index];
    if cursor_distance_from_actor <= actor.move_range {
        actor_position.x = cursor_x;
        actor_position.y = cursor_y;

        match game_state.menu.as_mut() {
            Some(menu) => menu.remove(&MenuOption::Move),
            None => {},
        }
        match game_state.turn.as_mut() {
            Some(turn) => turn.moved = true,
            None => {},
        }
        game_state.player_state = PlayerState::UnitSelected;
    }

}

fn cancel_actor_action(game_state: &mut GameState) {
    let actor_position = &game_state.positions[game_state.active_actor_index.unwrap()];
    game_state.cursor.x = actor_position.x;
    game_state.cursor.y = actor_position.y;

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
    let active_index = game_state.active_actor_index.unwrap();
    let actor = &game_state.actors[active_index];
    let actor_position = &game_state.positions[active_index];

    let cursor_x = game_state.cursor.x;
    let cursor_y = game_state.cursor.y;
    let cursor_distance_from_actor = (actor_position.x - cursor_x).abs() + (actor_position.y - cursor_y).abs();

    if cursor_distance_from_actor > actor.attack_range {
        return
    }

    game_state.cursor.x = actor_position.x;
    game_state.cursor.y = actor_position.y;

    match game_state.menu.as_mut() {
        Some(menu) => menu.remove(&MenuOption::Attack),
        None => {},
    }
    match game_state.turn.as_mut() {
        Some(turn) => turn.acted = true,
        None => {},
    }

    println!("You swing wildly at the air.");
    game_state.player_state = PlayerState::UnitSelected;
}