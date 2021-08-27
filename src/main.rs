use olc_rust_game_engine::{Color, ConsoleGameEngine, Utils, Result, Rules};
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

    fn draw_maze(&self, utils: &mut Utils) {
        for x in 0..self.maze_width {
            for y in 0..self.maze_height {
                let maze_x = x * (self.path_width + 1);
                let maze_y = y * (self.path_width + 1);
                let maze_char = self.maze[y * self.maze_width + x];

                for px in 0..self.path_width {
                    for py in 0..self.path_width {
                        match maze_char {
                            m_char if m_char & VISITED == VISITED => {
                                utils.draw(maze_x + px, maze_y + py, '█', Color::White)
                            }
                            _ => utils.draw(maze_x + px, maze_y + py, '█', Color::Blue),
                        }
                    }
                }

                for p in 0..self.path_width {
                    let maze_x = x * (self.path_width + 1);
                    let maze_y = y * (self.path_width + 1);
                    if maze_char & SOUTH > 0 {
                        utils.draw(maze_x + p, maze_y + self.path_width, '█', Color::White)
                    }
                    if maze_char & EAST > 0 {
                        utils.draw(maze_x + self.path_width, maze_y + p, '█', Color::White)
                    }
                }
            }
        }

        for px in 0..self.path_width {
            for py in 0..self.path_width {
                utils.draw(
                    self.stack.last().unwrap().0 * (self.path_width + 1) + px,
                    self.stack.last().unwrap().1 * (self.path_width + 1) + py,
                    '█',
                    Color::Green,
                );
            }
        }
    }
}

impl Rules for MazeRules {
    fn on_user_create(&mut self, utils: &mut Utils) {
        let mut rng = rand::thread_rng();
        utils.fill(0, 0, utils.width, utils.height, ' ', Color::Black);

        let start_x = rng.gen_range(0..self.maze_width);
        let start_y = rng.gen_range(0..self.maze_height);
        self.stack.push((start_x, start_y));
        self.maze[start_y * self.maze_width + start_x] = VISITED;
        self.visited = 1;
        self.path_width = 2;
    }

    fn on_user_update(&mut self, utils: &mut Utils, _elapsed_time: f64) {
        //std::thread::sleep(std::time::Duration::from_millis(2));

        if self.visited < self.maze_width * self.maze_height {
            let neighbors = self.get_unvisited_neighbors();

            if !neighbors.is_empty() {
                self.visit_next_neighbor(neighbors);
            } else {
                self.stack.pop();
            }
        }

        self.draw_maze(utils);
    }
}

struct RacerRules {
    car_distance: f64,
    car_position: f64,
    current_lap_time: f64,
    curvature: f64,
    lap_times: Vec<f64>,
    player_curvature: f64,
    speed: f64,
    track: Vec<(f64, f64)>,
    track_curvature: f64,
    track_distance: f64,
}

impl RacerRules {
    fn new() -> Self {
        RacerRules {
            car_distance: 0.0,
            car_position: 0.0,
            current_lap_time: 0.0,
            curvature: 0.0,
            lap_times: vec![0.0; 5],
            player_curvature: 0.0,
            speed: 0.0,
            track: vec![],
            track_curvature: 0.0,
            track_distance: 0.0,
        }
    }
}

impl Rules for RacerRules {
    fn on_user_create(&mut self, _utils: &mut Utils) {
        self.track.push((0.0, 10.0));
        self.track.push((0.0, 250.0));
        self.track.push((0.5, 200.0));
        self.track.push((1.0, 200.0));
        self.track.push((0.5, 175.0));
        self.track.push((0.0, 200.0));
        self.track.push((-0.5, 100.0));
        self.track.push((-1.0, 100.0));
        self.track.push((0.0, 200.0));
        self.track.push((-1.0, 200.0));
        self.track.push((1.0, 200.0));
        self.track.push((0.0, 200.0));
        self.track.push((0.2, 500.0));
        self.track.push((0.0, 200.0));

        for track in &self.track {
            self.track_distance += track.1;
        }
    }

    fn on_user_update(&mut self, utils: &mut Utils, elapsed_time: f64) {
        if utils.keys.contains(&38) {
            self.speed += 2.0 * elapsed_time;
        } else {
            self.speed -= 1.0 * elapsed_time;
        }

        let mut car_direction = 0;

        if utils.keys.contains(&37) {
            self.player_curvature -= 0.7 * elapsed_time;
            car_direction = -1;
        }

        if utils.keys.contains(&39) {
            self.player_curvature += 0.7 * elapsed_time;
            car_direction = 1;
        }

        if (self.player_curvature - self.track_curvature).abs() >= 0.8 {
            self.speed -= 2.5 * elapsed_time;
        }

        if self.speed < 0.0 {
            self.speed = 0.0;
        }
        if self.speed > 1.0 {
            self.speed = 1.0;
        }

        self.car_distance += (70.0 * self.speed) * elapsed_time;

        let mut offset = 0.0;
        let mut track_section = 0;

        self.current_lap_time += elapsed_time;

        if self.car_distance >= self.track_distance {
            self.car_distance -= self.track_distance;
            self.lap_times.insert(0, self.current_lap_time);
            self.lap_times.pop();
            self.current_lap_time = 0.0;
        }

        while track_section < self.track.len() && offset <= self.car_distance {
            offset += self.track[track_section].1;
            track_section += 1;
        }

        let target_curvature = self.track[track_section - 1].0;
        let curvature_diff = (target_curvature - self.curvature) * elapsed_time * self.speed;
        self.curvature += curvature_diff;

        self.track_curvature += self.curvature * elapsed_time * self.speed;

        // draw the sky
        for y in 0..utils.height / 2 {
            for x in 0..utils.width {
                let sky_color = match y {
                    y if y < utils.height / 4 => Color::DarkBlue,
                    _ => Color::Blue
                };
                utils.draw(x, y, '█', sky_color);
            }
        }

        // draw a hill
        for x in 0..utils.width {
            let hill_height = (12.0 * (x as f64 * 0.0175 + self.track_curvature).sin()).abs();
            for y in ((utils.height as f64 / 2.0) - hill_height) as usize..(utils.height / 2) {
                utils.draw(x, y, '█', Color::DarkYellow);
            }
        }

        // draw road, clip, and grass
        for y in 0..(utils.height / 2) {
            for x in 0..utils.width {
                let perspective = y as f64 / (utils.height as f64 / 2.0);
                let middle = 0.5 + self.curvature * (1.0 - perspective).powf(3.0);
                let mut road_width = 0.1 + perspective * 0.8;
                let clip_width = road_width * 0.15;
                road_width /= 2.0;

                let left_grass = (middle - road_width - clip_width) * utils.width as f64;
                let right_grass = (middle + road_width + clip_width) * utils.width as f64;
                let left_clip = (middle - road_width) * utils.width as f64;
                let right_clip = (middle + road_width) * utils.width as f64;

                let y_flip = (utils.height / 2) + y;

                let grass_arg: f64 = 20.0 * (1.0 - perspective).powf(3.0) + self.car_distance * 0.1;
                let grass_color = match grass_arg.sin() {
                    x if x > 0.0 => Color::Green,
                    _ => Color::DarkGreen,
                };

                let clip_arg: f64 = 60.0 * (1.0 - perspective).powf(1.5) + self.car_distance;
                let clip_color = match clip_arg.sin() {
                    x if x > 0.0 => Color::Red,
                    _ => Color::White,
                };

                let road_color = match track_section - 1 {
                    0 => Color::White,
                    _ => Color::DarkGrey,
                } ;

                match x as f64 {
                    x if x < left_grass => utils.draw(x as usize, y_flip, '█', grass_color),
                    x if x >= left_grass && x < left_clip => {
                        utils.draw(x as usize, y_flip, '█', clip_color)
                    }
                    x if x >= left_clip && x < right_clip => {
                        utils.draw(x as usize, y_flip, '█', road_color)
                    }
                    x if x >= right_clip && x < right_grass => {
                        utils.draw(x as usize, y_flip, '█', clip_color)
                    }
                    x if x >= right_grass => utils.draw(x as usize, y_flip, '█', grass_color),
                    _ => (),
                }
            }
        }

        // draw car
        self.car_position = self.player_curvature - self.track_curvature;
        let car_x =
            utils.width as f64 / 2.0 + ((utils.width as f64 * self.car_position) / 2.0) - 6.0;
        let car_y = (0.8 * utils.height as f64) as usize;

        match car_direction {
            1 => {
                utils.draw_string(car_x as usize, car_y + 0, "      //####//", Color::Black, true);
                utils.draw_string(car_x as usize, car_y + 1, "         ##   ", Color::Black, true);
                utils.draw_string(car_x as usize, car_y + 2, "       ####   ", Color::Black, true);
                utils.draw_string(car_x as usize, car_y + 3, "      ####    ", Color::Black, true);
                utils.draw_string(car_x as usize, car_y + 4, "///  ####//// ", Color::Black, true);
                utils.draw_string(car_x as usize, car_y + 5, "//#######///O ", Color::Black, true);
                utils.draw_string(car_x as usize, car_y + 6, "/// #### //// ", Color::Black, true);
            },
            -1 => {
                utils.draw_string(car_x as usize, car_y + 0, "\\\\####\\\\      ", Color::Black, true);
                utils.draw_string(car_x as usize, car_y + 1, "   ##         ", Color::Black, true);
                utils.draw_string(car_x as usize, car_y + 2, "   ####       ", Color::Black, true);
                utils.draw_string(car_x as usize, car_y + 3, "    ####      ", Color::Black, true);
                utils.draw_string(car_x as usize, car_y + 4, " \\\\\\\\####  \\\\\\", Color::Black, true);
                utils.draw_string(car_x as usize, car_y + 5, " O\\\\\\#######\\\\", Color::Black, true);
                utils.draw_string(car_x as usize, car_y + 6, " \\\\\\\\ #### \\\\\\", Color::Black, true);
            },
            _ => {
                utils.draw_string(car_x as usize, car_y + 0, "   ||####||   ", Color::Black, true);
                utils.draw_string(car_x as usize, car_y + 1, "      ##      ", Color::Black, true);
                utils.draw_string(car_x as usize, car_y + 2, "     ####     ", Color::Black, true);
                utils.draw_string(car_x as usize, car_y + 3, "     ####     ", Color::Black, true);
                utils.draw_string(car_x as usize, car_y + 4, "|||  ####  |||", Color::Black, true);
                utils.draw_string(car_x as usize, car_y + 5, "|||########|||", Color::Black, true);
                utils.draw_string(car_x as usize, car_y + 6, "|||  ####  |||", Color::Black, true);
            }
		}
    
        // draw stats
        utils.draw_string(0, 1, &format!("Distance {:.2}", self.car_distance), Color::White, false);
        utils.draw_string(0, 2, &format!("Target Curvature {:.2}", self.curvature), Color::White, false);
        utils.draw_string(0, 3, &format!("Player Curvature {:.2}", self.player_curvature), Color::White, false);
        utils.draw_string(0, 4, &format!("Player Speed {}", (self.speed * 70.0) as usize), Color::White, false);
        utils.draw_string(0, 5, &format!("Track Curvature {:.2}", self.track_curvature), Color::White, false);
    
        let display_lap_time = |t: &f64| -> String {
            let mins = (t / 60.0) as usize;
            let secs = *t as usize - (mins * 60);
            let millis = (t - secs as f64) * 10000.0;

            // todo: format this betterer
            return format!("{:02}:{:02}:{:04}", mins, secs, millis as usize);
        };

        utils.draw_string(10, 8, &display_lap_time(&self.current_lap_time), Color::White, false);

        for (ind, lap_time) in self.lap_times.iter().enumerate() {
            utils.draw_string(10, 10 + ind, &display_lap_time(lap_time), Color::White, false);
        }
    }
}

fn main() -> Result<()> {
    //let rules = MazeRules::new(25, 40);
    let rules = RacerRules::new();
    let game = ConsoleGameEngine::new(75, 120);
    //game.construct_console()?;
    game.start(Box::leak(Box::new(rules)))
}
