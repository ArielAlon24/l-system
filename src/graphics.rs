use crate::system::SystemIterator;
use crate::{Character, State, System};
use raylib;
use raylib::prelude::*;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

#[derive(Debug)]
pub struct Config {
    line_length: i32,
    line_width_increment: f32,
    line_length_scale_factor: f64,
    turning_angle: f32,
    turning_angle_increment: f32,
}

impl Config {
    pub fn new(
        line_length: i32,
        line_width_increment: f32,
        line_length_scale_factor: f64,
        turning_angle: f32,
        turning_angle_increment: f32,
    ) -> Self {
        Self {
            line_length,
            line_width_increment,
            line_length_scale_factor,
            turning_angle: turning_angle.to_radians(),
            turning_angle_increment: turning_angle_increment.to_radians(),
        }
    }
}

trait StateDrawer {
    fn draw_state(&mut self, state: &State, x: i32, y: i32, config: &mut Config, color: Color);
}

impl StateDrawer for RaylibDrawHandle<'_> {
    fn draw_state(&mut self, state: &State, x: i32, y: i32, config: &mut Config, color: Color) {
        let mut pos = Vector2::new(x as f32, y as f32);
        let mut angle = std::f32::consts::PI / 2.0;
        let mut thickness = 1.0;
        for character in state {
            match character {
                Character::Var(_) => {}
                // TODO: Draw only inside of the screen bounds
                Character::Draw => {
                    let temp = Vector2::new(
                        pos.x - config.line_length as f32 * angle.cos(),
                        pos.y - config.line_length as f32 * angle.sin(),
                    );
                    if 0.0 <= temp.x
                        && temp.x <= self.get_screen_width() as f32
                        && 0.0 <= temp.y
                        && temp.y <= self.get_screen_height() as f32
                    {
                        self.draw_line_ex(pos, temp, thickness, color);
                    }
                    pos = temp;
                }
                Character::Move => {
                    pos = Vector2::new(
                        pos.x - config.line_length as f32 * angle.cos(),
                        pos.y - config.line_length as f32 * angle.sin(),
                    )
                }
                Character::Left => angle -= config.turning_angle,
                Character::Right => angle += config.turning_angle,
                Character::Reverse => todo!(),
                Character::Push => todo!(),
                Character::Pop => todo!(),
                Character::IncLine => thickness += config.line_width_increment,
                Character::DecLine => thickness -= config.line_width_increment,
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
                Character::IncAngle => angle += config.turning_angle_increment,
                Character::DecAngle => angle -= config.turning_angle_increment,
            }
        }
    }
}

pub struct Visualizer {
    width: i32,
    height: i32,
    name: &'static str,
    system: System,
    config: Config,
}

impl Visualizer {
    const FONT_PATH: &str = "assets/Iosevka-Light.ttf";
    const FONT_SCALE: i32 = 30;
    const PADDING: i32 = 4;

    pub fn new(
        name: &'static str,
        (width, height): (i32, i32),
        system: System,
        config: Config,
    ) -> Self {
        Self {
            width,
            height,
            name,
            system,
            config,
        }
    }

    fn init(&mut self) -> (RaylibHandle, RaylibThread) {
        raylib::init()
            .size(self.width, self.height)
            .title(self.name)
            .resizable()
            .build()
    }

    fn resize(&mut self, handle: &RaylibHandle) {
        self.width = handle.get_screen_width();
        self.height = handle.get_screen_height();
    }

    fn load_font(handle: &mut RaylibHandle, thread: &RaylibThread) -> Font {
        handle
            .load_font(thread, Self::FONT_PATH)
            .expect("Couldn't load font.")
    }

    fn create_iterator_channel(
        &mut self,
        iterator: &Arc<Mutex<SystemIterator>>,
        duration: &Arc<Mutex<f64>>,
    ) -> Sender<Option<()>> {
        let (sender, reciever) = mpsc::channel::<Option<()>>();

        let iterator_clone = Arc::clone(&iterator);
        let duration_clone = Arc::clone(&duration);

        thread::spawn(move || loop {
            match reciever.recv() {
                Ok(Some(_)) => {
                    let mut iterator = iterator_clone.lock().unwrap();
                    let start = Instant::now();
                    iterator.next().unwrap();
                    let mut duration = duration_clone.lock().unwrap();
                    *duration = (Instant::now() - start).as_secs_f64();
                }
                Ok(None) => break,
                Err(e) => panic!("{}", e),
            }
        });

        sender
    }

    pub fn run(&mut self) {
        let (mut handle, thread) = self.init();
        let font = Self::load_font(&mut handle, &thread);
        handle.set_target_fps(60);

        let iterator = Arc::new(Mutex::new(self.system.clone().into_iter()));
        let duration = Arc::new(Mutex::new(0.0));
        let sender = self.create_iterator_channel(&iterator, &duration);

        let mut iteration = 1;

        while !handle.window_should_close() {
            if handle.is_window_resized() {
                self.resize(&handle);
            }
            let mut d = handle.begin_drawing(&thread);

            {
                let iter = iterator.lock().unwrap();
                let time = duration.lock().unwrap();
                self.draw(&mut d, &font, iter.state(), iteration, *time);
            }

            if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
                sender.send(Some(())).unwrap();
                iteration += 1;
            } else if d.is_key_pressed(KeyboardKey::KEY_R) {
                let mut iter = iterator.lock().unwrap();
                *iter = self.system.clone().into_iter();
                let mut duration = duration.lock().unwrap();
                *duration = 0.0;
                iteration = 1;
            }
        }

        sender.send(None).unwrap();
    }

    fn draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        font: &Font,
        state: &State,
        iteration: i32,
        last_duration: f64,
    ) {
        d.clear_background(Color::new(24, 25, 26, 255));
        d.draw_state(
            state,
            self.width / 2,
            self.height / 2,
            &mut self.config,
            Color::new(228, 230, 235, 255),
        );
        d.draw_rectangle(
            0,
            0,
            self.width,
            2 * Self::PADDING + self.height / Self::FONT_SCALE,
            Color::new(36, 37, 38, 255),
        );
        d.draw_line(
            0,
            2 * Self::PADDING + self.height / Self::FONT_SCALE,
            self.width,
            2 * Self::PADDING + self.height / Self::FONT_SCALE,
            Color::new(228, 230, 235, 255),
        );
        d.draw_text_ex(
            &font,
            &format!("N={}, took: {:.3}s", iteration, last_duration),
            Vector2::new((2 * Self::PADDING) as f32, Self::PADDING as f32),
            (self.height / Self::FONT_SCALE) as f32,
            (Self::PADDING / 2) as f32,
            Color::new(228, 230, 235, 255),
        );
    }
}
