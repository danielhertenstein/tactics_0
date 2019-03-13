use game_state::{GameState, PlayerState};

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
        Some(index) => {
            game_state.active_actor_index = Some(index);
            let actor = &game_state.actors[index];
            let actor_position = &game_state.positions[index];
            game_state.charge_times[index] = 100;
            if actor.player_controlled {
                game_state.cursor.x = actor_position.x;
                game_state.cursor.y = actor_position.y;
                game_state.player_state = PlayerState::TurnReady;
            }
        },
        None => {
            for i in 0..game_state.charge_times.len() {
                game_state.charge_times[i] += game_state.actors[i].speed;
            }

        },
    }
}

