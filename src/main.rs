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

    fn start(&mut self, on_user_create: fn(screen: &Vec<char>) -> Result<Vec<char>>, on_user_update: fn(screen: &Vec<char>) -> Result<Vec<char>>) -> Result<()> {
        let mut stdout = stdout();

        self.screen = on_user_create(&self.screen)?;

        loop {
            // handle timing

            // get user input

            // todo: pass in elapsed_time
            self.screen = on_user_update(&self.screen)?;

            // todo: set console title to something
            stdout.execute(cursor::MoveTo(0, 0))?;
        
            for screen_char in &self.screen {
                stdout.write(format!("{}", screen_char).as_bytes())?;
            }
        }
    }
}

fn main() -> Result<()> {
    let mut game = ConsoleGameEngine::new(40, 120);
    game.construct_console()?;

    let on_user_create = |screen: &Vec<char>| -> Result<Vec<char>> {
        let updated = screen.iter().map(|_| 'c').collect();
        Ok(updated)
    };

    let on_user_update = |screen: &Vec<char>| -> Result<Vec<char>> {
        let updated = screen.iter().map(|_| 'd').collect();
        Ok(updated)
    };

    game.start(on_user_create, on_user_update)?;

    Ok(())
}
