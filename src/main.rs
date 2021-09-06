use olc_rust_game_engine::{Color, ConsoleGameEngine, Utils, Rules};

use std::error::Error;

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
                utils.draw(x, y, sky_color);
            }
        }

        // draw a hill
        for x in 0..utils.width {
            let hill_height = (12.0 * (x as f64 * 0.0175 + self.track_curvature).sin()).abs();
            for y in ((utils.height as f64 / 2.0) - hill_height) as usize..(utils.height / 2) {
                utils.draw(x, y, Color::DarkYellow);
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
                    x if x < left_grass => utils.draw(x as usize, y_flip, grass_color),
                    x if x >= left_grass && x < left_clip => {
                        utils.draw(x as usize, y_flip, clip_color)
                    }
                    x if x >= left_clip && x < right_clip => {
                        utils.draw(x as usize, y_flip, road_color)
                    }
                    x if x >= right_clip && x < right_grass => {
                        utils.draw(x as usize, y_flip, clip_color)
                    }
                    x if x >= right_grass => utils.draw(x as usize, y_flip, grass_color),
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

fn main() -> Result<(), Box<dyn Error>> {
    let rules = RacerRules::new();
    let game = ConsoleGameEngine::new(75, 120);
    
    game.start(Box::leak(Box::new(rules)))
}
