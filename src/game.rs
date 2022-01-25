use std::ops::{Index, IndexMut};

pub struct Model {
    board: Board,
    player: Player
}

impl Model {
    pub fn new(x: X, y: Y) -> Model{
        Model {
            board: Board::new(x, y),
            player: Player::new(x, y)
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
        self.make_room(Point(0, 0), self.get_size(), None);
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
        let mut x = self.player.pos.0 as isize;
        let mut y = self.player.pos.1 as isize;

        let (size_x, size_y) = self.get_size();

        match dir {
            Direction::N => y -= 1,
            Direction::E => x += 1,
            Direction::S => y += 1,
            Direction::W => x -= 1
        }
        // checks if the x and y is within the bounds
        if !(0 <= x && x < size_x as isize-1 && 0 <= y && y < size_y as isize-1) {
            return
        }
        debug_assert!(0 <= x, "x is {x}");
        debug_assert!(0 <= y, "y is {y}");
        let new_point = Point(x as usize, y as usize);
        if self.is_cell_walkable(new_point) {
            self.player.pos = new_point;
        }
        // self.player.pos = new_point;
    }

    fn make_room(&mut self, top_left: Point, size: (X, Y), opening: Option<Direction>) {
        let p2 = Point(top_left.0 + size.0-1, top_left.1 + size.1-1);
        if self.board.point_in(&top_left) && self.board.point_in(&p2) {
            for y in top_left.1..p2.1+1 {
                for x in top_left.0..p2.0+1 {
                    let p = Point(x, y);
                    // if self.board[p].structure == Structures::Floor || self.board[p].structure == Structures::Void {
                    if y == top_left.1 || x == top_left.0 || y == p2.1 || x == p2.0 {
                        self.board[p].set_struct(Structures::Wall);
                    }
                    else {
                        self.board[p].set_struct(Structures::Floor);
                    }
                    // }
                }
            }
        }
        if let Some(dir) = opening {
            match dir {
                Direction::N => {
                    self.board[Point(top_left.0+(size.0/2), top_left.1)].set_struct(Structures::Floor);
                }
                Direction::E => {
                    self.board[Point(top_left.0+size.0-1, top_left.1+(size.1/2))].set_struct(Structures::Floor);
                }
                Direction::S => {
                    self.board[Point(top_left.0+(size.0/2), top_left.1+size.1-1)].set_struct(Structures::Floor);
                }
                Direction::W => {
                    self.board[Point(top_left.0, top_left.1+(size.1/2))].set_struct(Structures::Floor);
                }
            }
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

type Cells = Vec<Cell>;

struct Board {
    size: (usize, usize),
    cells: Cells,
}

impl Index<Point> for Board {
    type Output = Cell;

    fn index(&self, index: Point) -> &Self::Output {
        &self.cells[index.1 * self.size.0 + index.0]
    }
}

impl Index<Player> for Board {
    type Output = Cell;

    fn index(&self, player: Player) -> &Self::Output {
        let index = player.pos;
        &self.cells[index.1 * self.size.0 + index.0]
    }
}

impl IndexMut<Point> for Board {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.cells[index.1 * self.size.0 + index.0]
    }
}

impl IndexMut<Player> for Board {
    fn index_mut(&mut self, player: Player) -> &mut Self::Output {
        let index = player.pos;
        &mut self.cells[index.1 * self.size.0 + index.0]
    }
}

impl Board {
    fn new(x: X, y: Y) -> Board {
        let mut cells = Vec::with_capacity(x*y);
        for _ in 0..x*y {
            cells.push(Cell::new())
        }
        Board {
            size: (x, y),
            cells,
        }
    }

    fn point_in(&self, p: &Point) -> bool {
        let (size_x, size_y) = self.size;
        (0..size_x).contains(&p.0) && (0..size_y).contains(&p.1)
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
        matches!(self, Structures::Floor)
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
    fn new(x: X, y: Y) -> Player {
        Player {
            pos: Point(x/2,y/2)
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
