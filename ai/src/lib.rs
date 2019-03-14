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
        let new_pos = &game_state.positions[index] + (dx, dy);

        let other_actor_under_cursor = game_state.positions
            .iter()
            .find(|&position| position == &new_pos)
            .is_some();

        let map_width = game_state.map.len() as i32;
        let map_height = game_state.map[0].len() as i32;
        if !(new_pos.x < 0 || new_pos.y < 0 || new_pos.x >= map_width || new_pos.y >= map_height)
            && other_actor_under_cursor == false {
            &mut game_state.positions[index].move_to(&new_pos);
        }

        game_state.charge_times[index] = 50;
        game_state.active_actor_index = None;
        let name = &game_state.actors[index].name;
        println!("{} took its turn!", name);
    }
}
