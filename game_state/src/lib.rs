pub struct Actor {
    pub x: i32,
    pub y: i32,
    pub name: String,
    pub selected: bool,
    pub move_range: i32,
    pub attack_range: i32,
    pub speed: i32,
    pub player_controlled: bool,
    pub can_move: bool,
    pub can_act: bool,
}

#[derive(Clone)]
pub struct Tile {
    pub terrain: String,
    pub selected: bool,
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            terrain: String::from("Grass"),
            selected: false,
        }
    }
}

type Map = Vec<Vec<Tile>>;

#[derive(PartialEq)]
pub enum PlayerState {
    MovingCursor,
    UnitSelected,
    MovingActor,
    ActorAttacking,
}

pub struct Cursor {
    pub x: i32,
    pub y: i32,
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
        println!("Current selected index: {}", self.selected_index);
        println!("Current length {}", self.options.len());
        if self.selected_index < self.options.len() - 1 {
            self.selected_index += 1;
        }
        println!("New selected index: {}", self.selected_index);
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
        println!("New length {}", self.options.len());
    }

    pub fn contains(&self, option: &MenuOption) -> bool {
        self.options.contains(option)
    }
}

pub struct GameState {
    pub actors: Vec<Actor>,
    pub map: Map,
    pub player_state: PlayerState,
    pub cursor: Cursor,
    pub menu: Option<Menu>,
    // TODO: Switch to numeric array
    pub charge_times: Vec<i32>,
    pub active_actor_index: Option<usize>,
}

pub fn initial_game_state(map_height: i32, map_width: i32) -> GameState {
    GameState {
        actors: vec![
            Actor {
                x: 3,
                y: 0,
                name: String::from("Percy"),
                selected: false,
                move_range: 4,
                attack_range: 1,
                speed: 7,
                player_controlled: true,
                can_move: true,
                can_act: true,
            },
            Actor {
                x: 3,
                y: 1,
                name: String::from("Bad Guy"),
                selected: false,
                move_range: 3,
                attack_range: 1,
                speed: 4,
                player_controlled: false,
                can_move: true,
                can_act: true,
            },
        ],
        map: vec![vec![Tile::new(); map_height as usize]; map_width as usize],
        player_state: PlayerState::MovingCursor,
        cursor: Cursor { x: 0, y: 0 },
        menu: None,
        charge_times: vec![0, 0],
        active_actor_index: None,
    }
}
