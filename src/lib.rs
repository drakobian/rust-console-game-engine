use crossterm::style::{StyledContent, Stylize};
use crossterm::{cursor, execute, terminal, ExecutableCommand};
use std::io::{stdout, Write};

pub use crossterm::style::Color;
pub use crossterm::Result;

pub struct ConsoleGameEngine<T: Rules> {
    height: usize,
    painter: Painter,
    rules: T,
    width: usize,
}

impl<T> ConsoleGameEngine<T>
where
    T: Rules,
{
    pub fn new(height: usize, width: usize, rules: T) -> ConsoleGameEngine<T> {
        ConsoleGameEngine {
            height,
            painter: Painter::new(height, width),
            rules,
            width,
        }
    }

    pub fn construct_console(&self) -> Result<()> {
        // todo: set console title to something
        execute!(
            stdout(),
            terminal::SetSize(self.width as u16, self.height as u16),
            cursor::DisableBlinking,
            cursor::Hide
        )?;
        terminal::enable_raw_mode().unwrap(); // is this necessary?

        Ok(())
    }

    pub fn start(&mut self) -> Result<()> {
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

pub struct Painter {
    diff_coords: Vec<(usize, usize)>,
    pub height: usize,
    screen: Vec<StyledContent<char>>,
    pub width: usize,
}

impl Painter {
    fn new(height: usize, width: usize) -> Painter {
        Painter {
            diff_coords: vec![],
            height,
            screen: vec![' '.with(Color::Black); height * width],
            width,
        }
    }

    pub fn draw(&mut self, x: usize, y: usize, ch: char, color: crossterm::style::Color) {
        if x < self.width && y < self.height && self.screen[y * self.width + x] != ch.with(color) {
            self.screen[y * self.width + x] = ch.with(color);
            self.diff_coords.push((x, y));
        }
    }

    pub fn fill(
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

pub trait Rules {
    fn on_user_create(&mut self, painter: &mut Painter);
    fn on_user_update(&mut self, painter: &mut Painter, elapsed_time: f64);
}
