use crossterm::style::{
    Color::{Black, Blue, Green, White},
    StyledContent, Stylize,
};
use crossterm::{cursor, execute, terminal, ExecutableCommand, Result};
use rand::Rng;
use std::io::{stdout, Write};

struct ConsoleGameEngine<T: Rules> {
    height: usize,
    painter: Painter,
    rules: T,
    width: usize,
}

impl<T> ConsoleGameEngine<T>
where
    T: Rules,
{
    fn new(height: usize, width: usize, rules: T) -> ConsoleGameEngine<T> {
        ConsoleGameEngine {
            height,
            painter: Painter::new(height, width),
            rules,
            width,
        }
    }

    fn construct_console(&self) -> Result<()> {
        execute!(
            stdout(),
            terminal::SetSize(self.width as u16, self.height as u16),
            cursor::DisableBlinking,
            cursor::Hide
        )?;
        terminal::enable_raw_mode().unwrap(); // is this necessary?

        Ok(())
    }

    fn start(&mut self) -> Result<()> {
        let mut stdout = stdout();

        self.rules.on_user_create(&mut self.painter);

        let mut t_p_1 = std::time::Instant::now();
        let mut t_p_2: std::time::Instant;

        loop {
            t_p_2 = std::time::Instant::now();
            let elapsed_time = t_p_2.duration_since(t_p_1).as_secs_f64();
            t_p_1 = t_p_2;

            // todo: get user input (here? will that work?)

            self.rules.on_user_update(&mut self.painter, elapsed_time);

            // todo: set console title to something

            for coords in &self.painter.diff_coords {
                stdout.execute(cursor::MoveTo(coords.0 as u16, coords.1 as u16))?;
                stdout.write_all(
                    format!("{}", &self.painter.screen[coords.1 * self.width + coords.0])
                        .as_bytes(),
                )?;
            }
            &self.painter.diff_coords.clear();
        }
    }
}

struct Painter {
    diff_coords: Vec<(usize, usize)>,
    height: usize,
    screen: Vec<StyledContent<char>>,
    width: usize,
}

impl Painter {
    fn new(height: usize, width: usize) -> Painter {
        Painter {
            diff_coords: vec![],
            height,
            screen: vec![' '.with(Black); height * width],
            width,
        }
    }

    fn draw(&mut self, x: usize, y: usize, ch: char, color: crossterm::style::Color) {
        if x < self.width && y < self.height && self.screen[y * self.width + x] != ch.with(color) {
            self.screen[y * self.width + x] = ch.with(color);
            self.diff_coords.push((x, y));
        }
    }

    fn fill(
        &mut self,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        ch: char,
        color: crossterm::style::Color,
    ) {
        for x in x1..x2 {
            for y in y1..y2 {
                self.draw(x, y, ch, color);
            }
        }
    }
}

trait Rules {
    fn on_user_create(&mut self, painter: &mut Painter);
    fn on_user_update(&mut self, painter: &mut Painter, elapsed_time: f64);
}

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
            maze: vec![0x00; maze_height * maze_width],
            maze_height,
            maze_width,
            path_width: 1,
            stack: vec![],
            visited: 0,
        }
    }

    fn offset(&self, x: i32, y: i32) -> usize {
        let stack_top = self.stack.last().unwrap();
        ((stack_top.1 as i32 + y) * self.maze_width as i32 + stack_top.0 as i32 + x) as usize
    }
}

impl Rules for MazeRules {
    fn on_user_create(&mut self, painter: &mut Painter) {
        let mut rng = rand::thread_rng();
        painter.fill(0, 0, painter.width, painter.height, ' ', Black);

        let start_x = rng.gen_range(0..self.maze_width);
        let start_y = rng.gen_range(0..self.maze_height);
        self.stack.push((start_x, start_y));
        self.maze[start_y * self.maze_width + start_x] = 0x10;
        self.visited = 1;
        self.path_width = 2;
    }

    fn on_user_update(&mut self, painter: &mut Painter, _elapsed_time: f64) {
        std::thread::sleep(std::time::Duration::from_millis(20));

        /*let self.offset = |x: i32, y: i32, stack_top: (usize, usize)| -> usize {
            ((stack_top.1 as i32 + y) * self.maze_width as i32 + stack_top.0 as i32 + x) as usize
        };*/

        let mut rng = rand::thread_rng();

        if self.visited < self.maze_width * self.maze_height {
            let mut neighbors = vec![];

            // north neighbor
            if self.stack.last().unwrap().1 > 0 && self.maze[self.offset(0, -1)] & 0x10 == 0 {
                neighbors.push(0);
            }

            // east
            if self.stack.last().unwrap().0 < self.maze_width - 1
                && self.maze[self.offset(1, 0)] & 0x10 == 0
            {
                neighbors.push(1);
            }

            // south
            if self.stack.last().unwrap().1 < self.maze_height - 1
                && self.maze[self.offset(0, 1)] & 0x10 == 0
            {
                neighbors.push(2);
            }

            // west
            if self.stack.last().unwrap().0 > 0 && self.maze[self.offset(-1, 0)] & 0x10 == 0 {
                neighbors.push(3);
            }

            if !neighbors.is_empty() {
                let center = self.offset(0, 0);
                let north = self.offset(0, -1);
                let east = self.offset(1, 0);
                let west = self.offset(-1, 0);
                let south = self.offset(0, 1);
                match neighbors[rng.gen_range(0..neighbors.len())] {
                    0 => {
                        self.maze[center] |= 0x01;
                        self.maze[north] |= 0x04 | 0x10;
                        self.stack.push((
                            self.stack.last().unwrap().0,
                            self.stack.last().unwrap().1 - 1,
                        ));
                    }
                    1 => {
                        self.maze[center] |= 0x02;
                        self.maze[east] |= 0x08 | 0x10;
                        self.stack.push((
                            self.stack.last().unwrap().0 + 1,
                            self.stack.last().unwrap().1,
                        ));
                    }
                    2 => {
                        self.maze[center] |= 0x04;
                        self.maze[south] |= 0x01 | 0x10;
                        self.stack.push((
                            self.stack.last().unwrap().0,
                            self.stack.last().unwrap().1 + 1,
                        ));
                    }
                    3 => {
                        self.maze[center] |= 0x08;
                        self.maze[west] |= 0x02 | 0x10;
                        self.stack.push((
                            self.stack.last().unwrap().0 - 1,
                            self.stack.last().unwrap().1,
                        ));
                    }
                    _ => (),
                }
                self.visited += 1;
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
                            m_char if m_char & 0x10 == 0x10 => {
                                painter.draw(maze_x + px, maze_y + py, '█', White)
                            }
                            _ => painter.draw(maze_x + px, maze_y + py, '█', Blue),
                        }
                    }
                }

                for p in 0..self.path_width {
                    let maze_x = x * (self.path_width + 1);
                    let maze_y = y * (self.path_width + 1);
                    if maze_char & 0x04 > 0 {
                        painter.draw(maze_x + p, maze_y + self.path_width, '█', White)
                    }
                    if maze_char & 0x02 > 0 {
                        painter.draw(maze_x + self.path_width, maze_y + p, '█', White)
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
                    Green,
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
