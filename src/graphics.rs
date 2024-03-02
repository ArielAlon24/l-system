use crate::{Character, State, System};
use raylib;
use raylib::prelude::*;

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * (std::f64::consts::PI / 180.0)
}

#[derive(Debug)]
pub struct Config {
    pub line_length: i32,
    pub line_length_scale_factor: f64,
    pub turning_angle: f64,
    pub turning_angle_increment: f64,
}

impl Config {
    pub fn new(
        line_length: i32,
        line_length_scale_factor: f64,
        turning_angle: f64,
        turning_angle_increment: f64,
    ) -> Self {
        let s = Self {
            line_length,
            line_length_scale_factor,
            turning_angle: degrees_to_radians(turning_angle),
            turning_angle_increment: degrees_to_radians(turning_angle_increment),
        };
        dbg!("{:?}", &s);
        s
    }
}

trait StateDrawer {
    fn draw_state(&mut self, state: &State, x: i32, y: i32, config: &mut Config);
}

impl StateDrawer for RaylibDrawHandle<'_> {
    fn draw_state(&mut self, state: &State, x: i32, y: i32, config: &mut Config) {
        let (mut x, mut y) = (x, y);
        let mut angle = std::f64::consts::PI / 2.0;
        for character in state {
            match character {
                Character::Var(_) => {}
                Character::Draw => {
                    let temp_x = x - (config.line_length as f64 * angle.cos()) as i32;
                    let temp_y = y - (config.line_length as f64 * angle.sin()) as i32;
                    self.draw_line(x, y, temp_x, temp_y, Color::WHITE);
                    x = temp_x;
                    y = temp_y;
                }
                Character::Move => {
                    x -= (config.line_length as f64 * angle.cos()) as i32;
                    y -= (config.line_length as f64 * angle.sin()) as i32;
                }
                Character::Left => angle -= config.turning_angle,
                Character::Right => angle += config.turning_angle,
                Character::Reverse => todo!(),
                Character::Push => todo!(),
                Character::Pop => todo!(),
                Character::IncLine => todo!(),
                Character::DecLine => todo!(),
                Character::Dot => todo!(),
                Character::OpenPolygon => todo!(),
                Character::ClosePolygon => todo!(),
                Character::MulLine => {
                    config.line_length =
                        (config.line_length as f64 * config.line_length_scale_factor) as i32
                }
                Character::DivLine => {
                    config.line_length =
                        (config.line_length as f64 * config.line_length_scale_factor) as i32
                }
                Character::SwapOperations => todo!(),
                Character::IncAngle => todo!(),
                Character::DecAngle => todo!(),
            }
        }
    }
}

pub struct Visualizer {
    handle: RaylibHandle,
    thread: RaylibThread,
    width: i32,
    height: i32,
    state_drawer_config: Config,
}

impl Visualizer {
    pub fn new(name: &str, (width, height): (i32, i32), state_drawer_config: Config) -> Self {
        let (handle, thread) = raylib::init()
            .size(width, height)
            .title(name)
            .resizable()
            .build();
        Self {
            handle,
            thread,
            width,
            height,
            state_drawer_config,
        }
    }

    fn resize(&mut self) {
        self.width = self.handle.get_screen_width();
        self.height = self.handle.get_screen_height();
    }

    pub fn run(&mut self, system: System) {
        let mut iterator = system.into_iter();
        let mut state = iterator.next().unwrap();
        let mut iteration = 1;

        while !self.handle.window_should_close() {
            if self.handle.is_window_resized() {
                self.resize();
            }
            let mut d = self.handle.begin_drawing(&self.thread);

            d.clear_background(Color::BLACK);
            d.draw_state(
                &state,
                self.width / 2,
                self.height / 2,
                &mut self.state_drawer_config,
            );
            d.draw_text(&format!("n={}", iteration), 12, 12, 20, Color::WHITE);

            if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
                state = iterator.next().unwrap();
                iteration += 1;
            }
        }
    }
}
