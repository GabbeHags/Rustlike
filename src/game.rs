use std::ops::{Index, IndexMut};

pub struct Model {
    board: Board,
    player: Player
}

impl Model {
    pub fn new(x: X, y: Y) -> Model{
        Model {
            board: Board::new(x, y),
            player: Player::new()
        }
    }

    pub fn get_size(&self) -> (X, Y) {
        self.board.size
    }

    pub fn get_cells(&self) -> &Cells {
        &self.board.cells
    }

    pub fn update(&mut self) {
        self.board = Board::new(self.board.size.0, self.board.size.1);
        self.board[self.player].entity = Some(Entity::Player);
    }

    pub fn do_action(&mut self, e: Events) {
        match e {
            Events::Nothing => {}
            Events::Move(dir) => {self.move_player(dir)}
            _ => {}
        }
    }

    fn is_cell_walkable(&self, point: Point) -> bool {
        !self.board[point].occupied() && self.board[point].structure.is_walkable()
    }

    fn move_player(&mut self, dir: Direction) {
        let mut new_point = self.player.pos;
        match dir {
            Direction::N => {
                if new_point.1 > 0 {
                    new_point.1 -= 1;
                }
            }
            Direction::E => {
                if new_point.0 < self.get_size().0-1 {
                    new_point.0 += 1
                }
            }
            Direction::S => {
                if new_point.1 < self.get_size().1-1 {
                    new_point.1 += 1
                }
            }
            Direction::W => {
                if new_point.0 > 0 {
                    new_point.0 -= 1
                }
            }
        }
        if self.is_cell_walkable(new_point) {
            self.player.pos = new_point;
        }
    }
}

pub enum Direction {
    N, E, S, W
}

pub enum Events {
    Nothing,
    Move(Direction),
    Quit,
}

type Cells = Vec<Vec<Cell>>;

struct Board {
    size: (usize, usize),
    cells: Cells,
}

impl Index<Point> for Board {
    type Output = Cell;

    fn index(&self, index: Point) -> &Self::Output {
        &self.cells[index.1][index.0]
    }
}
impl Index<Player> for Board {
    type Output = Cell;

    fn index(&self, player: Player) -> &Self::Output {
        let index = player.pos;
        &self.cells[index.1][index.0]
    }
}

impl IndexMut<Point> for Board {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.cells[index.1][index.0]
    }
}

impl IndexMut<Player> for Board {
    fn index_mut(&mut self, player: Player) -> &mut Self::Output {
        let index = player.pos;
        &mut self.cells[index.1][index.0]
    }
}

impl Board {
    fn new(x: X, y: Y) -> Board {
        let mut rows = Vec::with_capacity(y);
        for r in 0..y {
            rows.push(Vec::with_capacity(x));
            for _ in 0..x {
                rows[r].push(Cell::new());
            }
        }
        Board {
            size: (x, y),
            cells: rows,
        }
    }
}
#[derive(Copy, Clone)]
pub enum Structures {
    Void,
    Wall,
    Floor,
}

impl Structures {
    fn is_walkable(&self) -> bool {
        match self {
            Structures::Floor => true,
            _ => false
        }
    }
}
#[derive(Copy, Clone)]
pub enum Entity {
    Player,
}
type X = usize;
type Y = usize;
#[derive(Copy, Clone)]
struct Point(X, Y);

#[derive(Copy, Clone)]
struct Player {
    // health: usize,
    pos: Point,
}

impl Player {
    fn new() -> Player {
        Player {
            pos: Point(0,0)
        }
    }
}
#[derive(Copy, Clone)]
pub struct Cell {
    pub structure: Structures,
    pub entity: Option<Entity>,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            structure: Structures::Void,
            entity: None,
        }
    }

    pub fn occupied(&self) -> bool {
        self.entity.is_some()
    }

    fn set_struct(&mut self, s: Structures) {
        self.structure = s;
    }

}
