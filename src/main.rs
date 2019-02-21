struct Actor {
    x: i32,
    y: i32,
}

struct GameState {
    actors: Vec<Actor>,
}

fn main() {
    let mut game_state = initial_game_state();

    loop {
        render_system(&mut game_state);
    }
}

fn initial_game_state() -> GameState {
    GameState {
        actors: vec![
            Actor { x: 0, y: 0 },
            Actor { x: 0, y: 1 },
        ]
    }
}
