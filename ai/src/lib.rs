extern crate rand;

use rand::{thread_rng, Rng};

use game_state::GameState;

pub fn ai_control_system(game_state: &mut GameState) {
    if let Some(index) = game_state.active_actor_index {
        // Move ai controlled characters one random tile for now
        let mut rng = thread_rng();
        let direction: u32 = rng.gen_range(0, 4);
        let mut dx = 0;
        let mut dy = 0;
        match direction {
            0 => {
                dy = -1;
            },
            1 => {
                dx = 1;
            },
            2 => {
                dy = 1;
            },
            3 => {
                dx = -1;
            },
            _ => unreachable!()
        }
        let new_x = game_state.positions[index].x + dx;
        let new_y = game_state.positions[index].y + dy;

        let other_actor_under_cursor = game_state.positions
            .iter()
            .find(|position| position.x == new_x && position.y == new_y)
            .is_some();

        let map_width = game_state.map.len() as i32;
        let map_height = game_state.map[0].len() as i32;
        if !(new_x < 0 || new_y < 0 || new_x == map_width || new_y == map_height)
            && other_actor_under_cursor == false {
            let actor_position = &mut game_state.positions[index];
            actor_position.x = new_x;
            actor_position.y = new_y;
        }

        game_state.charge_times[index] = 50;
        game_state.active_actor_index = None;
        let name = &game_state.actors[index].name;
        println!("{} took its turn!", name);
    }
}
