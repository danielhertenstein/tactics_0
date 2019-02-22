pub struct Actor {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone)]
pub struct Tile {
    pub selected: bool,
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            selected: false,
        }
    }
}

type Map = Vec<Vec<Tile>>;

pub struct GameState {
    pub actors: Vec<Actor>,
    pub map: Map,
}

pub fn initial_game_state(screen_height: i32, screen_width: i32) -> GameState {
    let mut map = vec![vec![Tile::new(); screen_height as usize]; screen_width as usize];
    map[0][0].selected = true;

    GameState {
        actors: vec![
            Actor { x: 0, y: 0 },
            Actor { x: 0, y: 1 },
        ],
        map,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
