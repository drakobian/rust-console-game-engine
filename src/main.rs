use std::io::{stdout, Write};
use crossterm::{cursor, execute, ExecutableCommand, Result, terminal};
use rand::Rng;

struct ConsoleGameEngine<T: Rules + Copy> {
    height: usize,
    width: usize,
    painter : Painter,
    rules : T,
}

impl<T> ConsoleGameEngine<T>
where T : Rules + Copy{
    fn new(height: usize, width: usize, rules: T) -> ConsoleGameEngine<T> { 
        ConsoleGameEngine{ 
            height,
            rules,
            painter: Painter{ height, screen: vec![' '; width * height], width },
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

        while b_atom_active {
            // handle timing

            // get user input (here? will that work?)

            // todo: pass in elapsed_time (why?)
            b_atom_active = self.rules.on_user_update(&mut self.painter);

            // todo: set console title to something
            stdout.execute(cursor::MoveTo(0, 0))?;
        
            for screen_char in &self.painter.screen {
                stdout.write(format!("{}", screen_char).as_bytes())?;
            }

            std::thread::sleep(std::time::Duration::from_millis(50));
        }

        Ok(())
    }
}

struct Painter { height: usize, screen : Vec<char>, width: usize }

impl Painter { 
    fn draw(&mut self, x: usize, y: usize, ch: char) {
        self.screen[y * self.width + x] = ch;
    }

    fn fill(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, ch: char) {
        for x in x1..x2 {
            for y in y1..y2 {
                self.draw(x, y, ch);
            }
        }
    }
}

trait Rules {
    fn on_user_create(self, painter: &mut Painter) -> bool;
    fn on_user_update(self, painter: &mut Painter) -> bool;
}

#[derive(Copy, Clone)]
struct MyRules {
    f_player_x : f64,
    f_player_y : f64,
}

impl MyRules {
    fn new() -> MyRules{
        MyRules {
            f_player_x : 10.0,
            f_player_y : 10.0,
        }
    }
}

impl Rules for MyRules {
    fn on_user_create(self, painter: &mut Painter) -> bool { true }
    fn on_user_update(self, painter: &mut Painter) -> bool { 
        let mut rng = rand::thread_rng();
        painter.fill(0, 0, painter.width, painter.height, ' ');

        for x in (0..painter.width - 5).step_by(5) {
            for y in (0..painter.height - 5).step_by(5) {
                let ch = match rng.gen_range(0..5) {
                    0 => 'd',
                    1 => 'r',
                    2 => 'a',
                    3 => 'k',
                    4 => 'e',
                    _ => ' ',
                };
                painter.fill(x, y, x + 5, y + 5, ch);
            }
        }
        
        true 
    }
}

fn main() -> Result<()> {
    let rules = MyRules::new();
    let mut game = ConsoleGameEngine::new(40, 120, rules);//ConsoleGameEngine::new(40, 120, rules);
    game.construct_console()?;
    game.start()?;

    Ok(())
}
