use std::io::{stdout, Write};
use crossterm::{cursor, execute, ExecutableCommand, Result, terminal};

struct ConsoleGameEngine {
    height: usize,
    screen: Vec<char>,
    width: usize,
}

impl ConsoleGameEngine {
    fn new(height: usize, width: usize) -> ConsoleGameEngine { 
        ConsoleGameEngine{ 
            height,
            screen: vec!['~'; width * height],
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

    fn start(&mut self, on_user_create: fn(game: &mut Self) -> bool, on_user_update: fn(game: &mut Self) -> bool) -> Result<()> {
        let mut stdout = stdout();

        //self.screen = on_user_create(&self)?;
        let mut b_atom_active = on_user_create(self);

        while b_atom_active {
            // handle timing

            // get user input

            // todo: pass in elapsed_time
            //self.screen = on_user_update(&self.screen)?;
            b_atom_active = on_user_update(self);

            // todo: set console title to something
            stdout.execute(cursor::MoveTo(0, 0))?;
        
            for screen_char in &self.screen {
                stdout.write(format!("{}", screen_char).as_bytes())?;
            }
        }

        Ok(())
    }

    fn draw(&mut self, x: usize, y: usize, ch: char) {
        self.screen[y * self.width + x] = ch;
    }
}

fn main() -> Result<()> {
    let mut game = ConsoleGameEngine::new(40, 120);
    game.construct_console()?;

    let on_user_create = |game: &mut ConsoleGameEngine| -> bool {
        //let updated = game.iter().map(|_| 'c').collect();
        //Ok(updated)
        for x in 0..game.width {
            for y in 0..game.height {
                game.draw(x, y, 'd');
            }
        }
        true
    };

    let on_user_update = |game: &mut ConsoleGameEngine| -> bool {
        //let updated = game.iter().map(|_| 'd').collect();
        //Ok(updated)
        for x in 0..game.width {
            for y in 0..game.height {
                game.draw(x, y, 'r');
            }
        }
        true
    };

    game.start(on_user_create, on_user_update)?;

    Ok(())
}
