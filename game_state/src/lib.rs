use std::ops::Add;

pub struct Actor {
    pub name: String,
    pub player_controlled: bool,
    pub can_move: bool,
    pub can_act: bool,
}

#[derive(Eq, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn distance_to(&self, other: &Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn move_to(&mut self, other: &Position) {
        self.x = other.x;
        self.y = other.y;
    }

    pub fn magnitude(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl Add<(i32, i32)> for &Position {
    type Output = Position;

    fn add(self, other: (i32, i32)) -> Position {
        Position {
            x: self.x + other.0,
            y: self.y + other.1,
        }
    }
}

pub struct CombatStatistics {
    pub health: i32,
    pub strength: i32,
    pub constitution: i32,
    pub move_range: i32,
    pub attack_range: i32,
    pub speed: i32,
}

#[derive(Clone)]
pub struct Tile {
    pub terrain: String,
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            terrain: String::from("Grass"),
        }
    }
}

type Map = Vec<Vec<Tile>>;

pub fn map_contains_position(map: &Map, position: &Position) -> bool {
    let map_width = map.len() as i32;
    let map_height = map[0].len() as i32;
    !(position.x < 0 || position.y < 0 || position.x >= map_width || position.y >= map_height)

}

#[derive(PartialEq)]
pub enum PlayerState {
    WaitingForTurn,
    TurnReady,
    MovingCursor,
    UnitSelected,
    MovingActor,
    ActorAttacking,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum MenuOption {
    Move,
    Attack,
    EndTurn,
}

impl std::fmt::Display for MenuOption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            MenuOption::Move => write!(f, "Move"),
            MenuOption::Attack => write!(f, "Attack"),
            MenuOption::EndTurn => write!(f, "End Turn"),
        }
    }
}

#[derive(Clone)]
pub struct Menu {
    pub options: Vec<MenuOption>,
    pub selected_index: usize,
}

impl Menu {
    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected_index < self.options.len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn select(&mut self) -> Option<&MenuOption> {
        if self.options.len() == 0 {
            return None
        }
        let option_to_return = Some(&self.options[self.selected_index]);
        self.selected_index = 0;

        option_to_return
    }

    pub fn remove(&mut self, option: &MenuOption) {
        let pos = self.options
            .iter()
            .position(|x| x == option)
            .expect("Option does not exist in menu.");
        self.options.remove(pos);
    }

    pub fn contains(&self, option: &MenuOption) -> bool {
        self.options.contains(option)
    }
}

pub struct Turn {
    pub moved: bool,
    pub acted: bool,
}

impl Turn {
    pub fn new() -> Turn {
        Turn {
            moved: false,
            acted: false,
        }
    }
}

pub struct GameState {
    pub actors: Vec<Actor>,
    pub positions: Vec<Position>,
    pub combat_stats: Vec<CombatStatistics>,
    pub map: Map,
    pub player_state: PlayerState,
    pub cursor: Position,
    pub menu: Option<Menu>,
    // TODO: Switch to numeric array
    pub charge_times: Vec<i32>,
    pub active_actor_index: Option<usize>,
    pub turn: Option<Turn>,
}

pub fn initial_game_state(map_height: i32, map_width: i32) -> GameState {
    GameState {
        actors: vec![
            Actor {
                name: String::from("Percy"),
                player_controlled: true,
                can_move: true,
                can_act: true,
            },
            Actor {
                name: String::from("Bad Guy"),
                player_controlled: false,
                can_move: true,
                can_act: true,
            },
            Actor {
                name: String::from("Pansy"),
                player_controlled: true,
                can_move: true,
                can_act: true,
            },
        ],
        positions: vec![
            Position { x: 5, y: 5 },
            Position { x: 5, y: 6 },
            Position { x: 8, y: 5 },
        ],
        combat_stats: vec![
            CombatStatistics {
                health: 50,
                strength: 3,
                constitution: 4,
                move_range: 4,
                attack_range: 1,
                speed: 7,
            },
            CombatStatistics {
                health: 50,
                strength: 3,
                constitution: 4,
                move_range: 3,
                attack_range: 1,
                speed: 6,
            },
            CombatStatistics {
                health: 50,
                strength: 3,
                constitution: 4,
                move_range: 4,
                attack_range: 1,
                speed: 1,
            },
        ],
        map: vec![vec![Tile::new(); map_height as usize]; map_width as usize],
        player_state: PlayerState::MovingCursor,
        cursor: Position { x: 0, y: 0 },
        menu: None,
        charge_times: vec![0, 0, 0],
        active_actor_index: None,
        turn: None,
    }
}
