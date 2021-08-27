//! # Rust OlcConsoleGameEngine
//!
//! `game_engine` is an attempt at a rust port of
//! [Javidx9's](https://www.youtube.com/channel/UC-yuWVUplUJZvieEligKBkA)
//! [Console Game Engine](https://github.com/OneLoneCoder/videos/blob/master/olcConsoleGameEngine.h)
//!
//! Better docs *definitely* coming soon 😁

#[macro_use]
extern crate glium;

use glium::{glutin, Surface};
use std::io::Cursor;

use crossterm::style::{Print, StyledContent, Stylize};
use crossterm::{cursor, execute, terminal};
use keyboard_query::{DeviceQuery, DeviceState};
use std::io::stdout;

pub use crossterm::style::Color;
pub use crossterm::Result;

pub struct ConsoleGameEngine {
    height: usize,
    //utils: 
    width: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
    //tex_coords: [f32; 2],
}

impl ConsoleGameEngine
{
    pub fn new(height: usize, width: usize) -> ConsoleGameEngine {
        ConsoleGameEngine {
            height,
            //util
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

    pub fn start(self, rules: &'static mut dyn Rules) -> Result<()> {
        
        implement_vertex!(Vertex, position, color);

        let vertex_shader_src = r#"
            #version 140

            in vec2 position;
            in vec3 color;

            out vec3 v_color;

            void main() {
                v_color = color;
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140

            in vec3 v_color;
            out vec4 color;

            void main() {
                color = vec4(v_color, 1.0);
            }
        "#;

        let mut event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new().with_title("Game time!");
        //.with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

        let mut utils = Utils::new(self.height, self.width);
        rules.on_user_create(&mut utils);

        let mut t_p_1 = std::time::Instant::now();
        
        let device_state = DeviceState::new();

        //loop {
        event_loop.run(move |ev, _, control_flow| {

            match ev {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    },
                    _ => return,
                },
                glutin::event::Event::NewEvents(cause) => match cause {
                    glutin::event::StartCause::ResumeTimeReached { .. } => {},
                    glutin::event::StartCause::Init => (),
                    _ => return,
                },
                _ => return,
            }
    
            let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16);
            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

            //let mut t_p_2: std::time::Instant::now();
            let mut t_p_2 = std::time::Instant::now();
            let elapsed_time = t_p_2.duration_since(t_p_1).as_secs_f64();
            t_p_1 = t_p_2;

            utils.keys = device_state.get_keys();

            rules.on_user_update(&mut utils, elapsed_time);

            let mut target = display.draw();
            //target.clear_color(0., 0., 0.2, 1.0);

            //println!("{}", &utils.index_buffer.len());
            //println!("{}", &utils.vertex_buffer.len());
            let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &utils.index_buffer).unwrap();
            let vertex_buffer = glium::VertexBuffer::new(&display, &utils.vertex_buffer).unwrap();

            target.draw(&vertex_buffer, &indices, &program, &uniform!{}, &Default::default()).unwrap();
            
            // now clear the index buffer and vertex_buffer on utils??
            utils.vertex_buffer.clear();
            utils.index_buffer.clear();
            //println!("{}", &utils.index_buffer.len());
            //println!("{}", &utils.vertex_buffer.len());
            //return;
            
            target.finish().unwrap();
        });
        //}
    }
}

pub struct Utils {
    diff_coords: Vec<(usize, usize)>,
    pub height: usize,
    pub keys: Vec<u16>,
    screen: Vec<StyledContent<char>>,
    pub width: usize,
    pub index_buffer: Vec<u16>,
    pub vertex_buffer: Vec<Vertex>,
}

impl Utils {
    fn new(height: usize, width: usize) -> Utils {
        Utils {
            diff_coords: vec![],
            height,
            keys: vec![],
            screen: vec![' '.with(Color::Black); height * width],
            width,
            index_buffer: vec![],
            vertex_buffer: vec![],
        }
    }

    pub fn redraw_screen(&mut self) -> Result<()> {
        let scr: String = self.screen.iter().map(|&x| format!("{}", x)).collect();

        execute!(stdout(), cursor::MoveTo(0, 0), Print(scr))?;

        Ok(())
    }

    pub fn draw_screen(&mut self) -> Result<()> {
        for coords in &self.diff_coords {
            execute!(
                stdout(),
                cursor::MoveTo(coords.0 as u16, coords.1 as u16),
                Print(self.screen[coords.1 * self.width + coords.0])
            )?;
        }
        self.diff_coords.clear();

        Ok(())
    }

    pub fn draw_string(&mut self, x: usize, y: usize, draw_str: &str, color: Color, alpha: bool) {
        for (i, c) in draw_str.chars().enumerate() {
            if alpha && c == ' ' {
                continue;
            }

            self.draw(x + i, y, c, color);
        }
    }

    pub fn draw(&mut self, x: usize, y: usize, ch: char, c_color: Color) {
        //hmm how scale
        let x_scale = 13. / 800.;
        let y_scale = 15. / 600.;
        let x_off = x as f32 * x_scale;
        let y_off = y as f32 * y_scale;
        let right = (x as f32 + 1.) * x_scale;
        let down = (y as f32 + 1.) * y_scale;

        let color = match c_color {
            Color::Black => [0., 0., 0.],
            Color::Red => [1., 0., 0.],
            Color::Green => [0., 1., 0.],
            Color::Blue => [0., 0., 1.],
            Color::DarkGrey => [0.2, 0.2, 0.2],
            Color::DarkGreen => [0., 0.4, 0.],
            Color::DarkYellow => [0.8, 0.6, 0.2],
            Color::DarkBlue => [0., 0.0, 0.4],
            _ => [1., 1., 1.],
        };

        self.vertex_buffer.push(Vertex{ position: [-1. + x_off, 1. - y_off], color});
        self.vertex_buffer.push(Vertex{ position: [-1. + right, 1. - y_off], color});
        self.vertex_buffer.push(Vertex{ position: [-1. + x_off, 1. - down], color});
        self.vertex_buffer.push(Vertex{ position: [-1. + right, 1. - down], color});
    
        let start_ind = (self.vertex_buffer.len() - 4) as u16;
        self.index_buffer.push(start_ind);
        self.index_buffer.push(start_ind + 1);
        self.index_buffer.push(start_ind + 2);
        self.index_buffer.push(start_ind + 1);
        self.index_buffer.push(start_ind + 2);
        self.index_buffer.push(start_ind + 3);
    }

    pub fn fill(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, ch: char, color: Color) {
        for x in x1..x2 {
            for y in y1..y2 {
                self.draw(x, y, ch, color);
            }
        }
    }
}

pub trait Rules {
    fn on_user_create(&mut self, utils: &mut Utils);
    fn on_user_update(&mut self, utils: &mut Utils, elapsed_time: f64);
}
