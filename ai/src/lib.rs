use game_state::GameState;

pub fn ai_control_system(game_state: &mut GameState) {
    if let Some(index) = game_state.active_actor_index {
        game_state.charge_times[index] = 0;
        game_state.active_actor_index = None;
        let name = &game_state.actors[index].name;
        println!("{} took its turn!", name);
    }
}
