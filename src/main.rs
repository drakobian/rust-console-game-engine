use std::io::{stdout, Write};
use crossterm::{cursor, execute, ExecutableCommand, Result, terminal};

/*
pub trait ConsoleGameEngine {
    fn construct_console(&self, width: usize, height: usize) -> Result<()> {
        //self.set_screen(vec!['~'; (width * height).into()]);
        execute!(
            stdout(),
            terminal::SetSize(width as u16, height as u16),
            cursor::DisableBlinking,
            cursor::Hide
        )?;
        terminal::enable_raw_mode().unwrap(); // is this necessary?

        Ok(())
    }

    fn on_user_create(&self) -> bool;
    fn on_user_update(&self) -> bool;
    
    fn get_screen(&self) -> Vec<char>;
    fn set_screen(&mut self, screen: Vec<char>);

    fn start(&self) -> Result<()> {
        let mut b_atom_active = self.on_user_create();//true;
        let screen = self.get_screen();

        //self.on_user_create();
        //b_atom_active = self.on_user_create();
        
        while b_atom_active {
            // handle timing

            // get user input

            // todo: pass in elapsed_time
            b_atom_active = self.on_user_update();

            // todo: set console title to something
            stdout().execute(cursor::MoveTo(0, 0))?;
        
            for screen_char in &screen {
                stdout().write(format!("{}", screen_char).as_bytes())?;
            }
        }

        Ok(())
    }
}

struct Demo {
    screen: Vec<char>
}

impl ConsoleGameEngine for Demo {
    fn on_user_create(&self) -> bool {
        true
    }

    fn on_user_update(&self) -> bool {
        true
    }
    
    fn get_screen(&self) -> Vec<char> {
        self.screen.to_vec()
    }

    fn set_screen(&mut self, screen: Vec<char>) {
        self.screen = screen;
    }
}
*/

/*struct Screen {
    //height : usize,
    content : Vec<char>,
    //width : usize,
}

impl Screen {
    fn new(height: usize, width: usize) -> Screen {
        Screen {
            //height,
            content : vec!['~'; width * height],
            //width,
        }
    }

    fn height(&self) -> usize { self.height }
    fn content(&self) -> Vec<char> { self.content.to_vec() }
    fn width(&self) -> usize { self.width }
}*/

struct ConsoleGameEngine {
    height: usize,
    screen: Vec<char>,
    width: usize,
}

impl ConsoleGameEngine {
    fn new(height: usize, width: usize) -> ConsoleGameEngine { 
        ConsoleGameEngine{ 
            height,
            screen: vec!['~'; width * height],//Screen::new(height, width),
            width,
        }
    }

    fn construct_console(&self) -> Result<()> {
        //self.set_screen(vec!['~'; (width * height).into()]);
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
        /*let mut b_atom_active = match on_user_create() {

        };//true;*/
        //let mut b_atom_active = true;
        let mut stdout = stdout();

        self.screen = on_user_create(&self.screen)?;
        /*match on_user_create(&self.screen) {
            Ok(result) => self.screen = result,
            Err(_) => b_atom_active = false,
        }*/

        //self.on_user_create();
        //b_atom_active = self.on_user_create();
        
        //while b_atom_active {
        loop {
            // handle timing

            // get user input

            // todo: pass in elapsed_time
            //b_atom_active = on_user_update();
            /*match on_user_update(&self.screen) {
                Ok(result) => self.screen = result,
                Err(_) => {
                    //b_atom_active = false;
                    break;
                },
            }*/
            self.screen = on_user_update(&self.screen)?;

            // todo: set console title to something
            stdout.execute(cursor::MoveTo(0, 0))?;
        
            for screen_char in &self.screen {
                stdout.write(format!("{}", screen_char).as_bytes())?;
            }
        }

        //Ok(())
    }
}

fn main() -> Result<()> {
    /*let width = 120;
    let height = 40;
    let game = Demo{ screen: vec!['~'; width * height] };
    game.construct_console(width, height)?;
    game.start()?;*/
    let mut game = ConsoleGameEngine::new(40, 120);
    game.construct_console()?;

    let on_user_create = |screen: &Vec<char>| -> Result<Vec<char>> {
        let updated = screen.iter().map(|_| 'c').collect();
        Ok(updated)
    };

    let on_user_update = |screen: &Vec<char>| -> Result<Vec<char>> {
        //Ok(screen.to_vec())
        let updated = screen.iter().map(|_| 'd').collect();
        Ok(updated)
    };

    game.start(on_user_create, on_user_update)?;


    // console setup
    /*let n_screen_width = 120;
    let n_screen_height = 40;

    let mut screen = vec!['~'; n_screen_width * n_screen_height];

    let mut stdout = stdout();

    execute!(
        stdout,
        terminal::SetSize(n_screen_width as u16, n_screen_height as u16),
        cursor::DisableBlinking,
        cursor::Hide
    )?;
    terminal::enable_raw_mode().unwrap(); // is this necessary?

    // game loop
    loop {
        screen[n_screen_height * n_screen_width - 1] = '\0';
        stdout.execute(cursor::MoveTo(0, 0))?;
        
        for screen_char in &screen {
            stdout.write(format!("{}", screen_char).as_bytes())?;
        }
 
    }*/

    Ok(())
}
