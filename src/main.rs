use crossterm::style::{
    Color::{Black, Blue, White},
    StyledContent, Stylize,
};
use crossterm::{cursor, execute, terminal, ExecutableCommand, Result};
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
            painter: Painter::new(height, width), //Painter{ height, screen: vec![' '; width * height], width },
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

        let mut b_atom_active = self.rules.on_user_create(&mut self.painter);

        let mut t_p_1 = std::time::Instant::now();
        let mut t_p_2: std::time::Instant;

        while b_atom_active {
            // handle timing
            t_p_2 = std::time::Instant::now();
            let elapsed_time = t_p_2.duration_since(t_p_1).as_secs_f64();
            t_p_1 = t_p_2;

            // get user input (here? will that work?)

            b_atom_active = self
                .rules
                .on_user_update(&mut self.painter, elapsed_time)
                .unwrap();

            // todo: set console title to something
            stdout.execute(cursor::MoveTo(0, 0))?;

            for screen_char in &self.painter.screen {
                stdout.write(format!("{}", screen_char).as_bytes())?;
            }
        }

        Ok(())
    }
}

struct Painter {
    height: usize,
    screen: Vec<StyledContent<char>>,
    width: usize,
}

impl Painter {
    fn new(height: usize, width: usize) -> Painter {
        Painter {
            height,
            screen: vec![' '.with(Black); height * width],
            width,
        }
    }

    fn draw(&mut self, x: usize, y: usize, ch: char, color: crossterm::style::Color) {
        self.screen[y * self.width + x] = ch.with(color);
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
    fn on_user_create(&mut self, painter: &mut Painter) -> bool;
    fn on_user_update(&mut self, painter: &mut Painter, elapsed_time: f64) -> Option<bool>;
}


/*enum CellPath {
    Z = 0x00,
    N = 0x01,
    E = 0x02,
    S = 0x04,
    W = 0x08,
    V = 0x10
}*/

// may need this bitwise later, or just figure out how to do without :)
#[derive(Clone, Copy)]
enum CellPath {
    Z = 0,
    // N,
    // E,
    // S,
    // W,
    V,
}

struct MazeRules {
    maze: Vec<CellPath>,
    maze_height: usize,
    maze_width: usize,
    stack: Vec<(usize, usize)>,
    visited: usize,
}

impl MazeRules {
    fn new(maze_height: usize, maze_width: usize) -> MazeRules {
        MazeRules {
            maze: vec![CellPath::Z; maze_height * maze_width],
            maze_height,
            maze_width,
            stack: vec![],
            visited: 0,
        }
    }
}

impl Rules for MazeRules {
    fn on_user_create(&mut self, _painter: &mut Painter) -> bool {
        self.stack.push((0, 0));
        self.maze[0] = CellPath::V;
        self.visited = 1;

        true
    }
    fn on_user_update(&mut self, painter: &mut Painter, _elapsed_time: f64) -> Option<bool> {
        painter.fill(0, 0, painter.width, painter.height, ' ', Black);

        for x in 0..self.maze_width {
            for y in 0..self.maze_height {
                match self.maze[y * self.maze_width + x] {
                    CellPath::V => painter.draw(x, y, '█', White),
                    _ => painter.draw(x, y, '█', Blue),
                }
            }
        }

        Some(true)
    }
}

fn main() -> Result<()> {
    let rules = MazeRules::new(25, 40);
    let mut game = ConsoleGameEngine::new(40, 120, rules);
    game.construct_console()?;
    game.start()?;

    Ok(())
}
