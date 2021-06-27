use game_engine::{Color, ConsoleGameEngine, Painter, Result, Rules};
use rand::Rng;

const EMPTY: i32 = 0x00;
const NORTH: i32 = 0x01;
const EAST: i32 = 0x02;
const SOUTH: i32 = 0x04;
const WEST: i32 = 0x08;
const VISITED: i32 = 0x10;

struct MazeRules {
    maze: Vec<i32>,
    maze_height: usize,
    maze_width: usize,
    path_width: usize,
    stack: Vec<(usize, usize)>,
    visited: usize,
}

impl MazeRules {
    fn new(maze_height: usize, maze_width: usize) -> MazeRules {
        MazeRules {
            maze: vec![EMPTY; maze_height * maze_width],
            maze_height,
            maze_width,
            path_width: 1,
            stack: vec![],
            visited: 0,
        }
    }

    fn peek(&self) -> (usize, usize) {
        *self.stack.last().unwrap()
    }

    fn offset(&self, x: i32, y: i32) -> usize {
        let (x1, y1) = self.peek();
        ((y1 as i32 + y) * self.maze_width as i32 + x1 as i32 + x) as usize
    }

    fn get_unvisited_neighbors(&self) -> Vec<usize>{
        let mut neighbors = vec![];
        let (x, y) = self.peek();

        // north neighbor
        if y > 0 && self.maze[self.offset(0, -1)] & VISITED == 0 {
            neighbors.push(0);
        }

        // east
        if x < self.maze_width - 1 && self.maze[self.offset(1, 0)] & VISITED == 0 {
            neighbors.push(1);
        }

        // south
        if y < self.maze_height - 1 && self.maze[self.offset(0, 1)] & VISITED == 0 {
            neighbors.push(2);
        }

        // west
        if x > 0 && self.maze[self.offset(-1, 0)] & VISITED == 0 {
            neighbors.push(3);
        }

        neighbors
    }

    fn visit_next_neighbor(&mut self, neighbors: Vec<usize>) {
        let center = self.offset(0, 0);
        let north = self.offset(0, -1);
        let east = self.offset(1, 0);
        let west = self.offset(-1, 0);
        let south = self.offset(0, 1);
        let (x, y) = self.peek();

        let mut rng = rand::thread_rng();
        match neighbors[rng.gen_range(0..neighbors.len())] {
            0 => {
                self.maze[center] |= NORTH;
                self.maze[north] |= SOUTH | VISITED;
                self.stack.push((x, y - 1));
            }
            1 => {
                self.maze[center] |= EAST;
                self.maze[east] |= WEST | VISITED;
                self.stack.push((x + 1, y));
            }
            2 => {
                self.maze[center] |= SOUTH;
                self.maze[south] |= NORTH | VISITED;
                self.stack.push((x, y + 1));
            }
            3 => {
                self.maze[center] |= WEST;
                self.maze[west] |= EAST | VISITED;
                self.stack.push((x - 1, y));
            }
            _ => (),
        }
        self.visited += 1;
    }
}

impl Rules for MazeRules {
    fn on_user_create(&mut self, painter: &mut Painter) {
        let mut rng = rand::thread_rng();
        painter.fill(0, 0, painter.width, painter.height, ' ', Color::Black);

        let start_x = rng.gen_range(0..self.maze_width);
        let start_y = rng.gen_range(0..self.maze_height);
        self.stack.push((start_x, start_y));
        self.maze[start_y * self.maze_width + start_x] = VISITED;
        self.visited = 1;
        self.path_width = 2;
    }

    fn on_user_update(&mut self, painter: &mut Painter, _elapsed_time: f64) {
        std::thread::sleep(std::time::Duration::from_millis(20));

        if self.visited < self.maze_width * self.maze_height {
            let neighbors = self.get_unvisited_neighbors();

            if !neighbors.is_empty() {
                self.visit_next_neighbor(neighbors);
            } else {
                self.stack.pop();
            }
        }

        for x in 0..self.maze_width {
            for y in 0..self.maze_height {
                let maze_x = x * (self.path_width + 1);
                let maze_y = y * (self.path_width + 1);
                let maze_char = self.maze[y * self.maze_width + x];

                for px in 0..self.path_width {
                    for py in 0..self.path_width {
                        match maze_char {
                            m_char if m_char & VISITED == VISITED => {
                                painter.draw(maze_x + px, maze_y + py, '█', Color::White)
                            }
                            _ => painter.draw(maze_x + px, maze_y + py, '█', Color::Blue),
                        }
                    }
                }

                for p in 0..self.path_width {
                    let maze_x = x * (self.path_width + 1);
                    let maze_y = y * (self.path_width + 1);
                    if maze_char & SOUTH > 0 {
                        painter.draw(maze_x + p, maze_y + self.path_width, '█', Color::White)
                    }
                    if maze_char & EAST > 0 {
                        painter.draw(maze_x + self.path_width, maze_y + p, '█', Color::White)
                    }
                }
            }
        }

        for px in 0..self.path_width {
            for py in 0..self.path_width {
                painter.draw(
                    self.stack.last().unwrap().0 * (self.path_width + 1) + px,
                    self.stack.last().unwrap().1 * (self.path_width + 1) + py,
                    '█',
                    Color::Green,
                );
            }
        }
    }
}

fn main() -> Result<()> {
    let rules = MazeRules::new(25, 40);
    let mut game = ConsoleGameEngine::new(100, 160, rules);
    game.construct_console()?;
    game.start()?;

    Ok(())
}
