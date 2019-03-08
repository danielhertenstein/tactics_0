extern crate rand;

use rand::{thread_rng, Rng};

use game_state::GameState;

pub fn ai_control_system(game_state: &mut GameState) {
    if let Some(index) = game_state.active_actor_index {
        let actor = &mut game_state.actors[index];
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
        let map_width = game_state.map.len() as i32;
        let map_height = game_state.map[0].len() as i32;

        let new_x = actor.x + dx;
        let new_y = actor.y + dy;

        if !(new_x < 0 || new_y < 0 || new_x == map_width || new_y == map_height) {
            actor.x = new_x;
            actor.y = new_y;
        }

        game_state.charge_times[index] = 50;
        game_state.active_actor_index = None;
        let name = &game_state.actors[index].name;
        println!("{} took its turn!", name);
    }
}
