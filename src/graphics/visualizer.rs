use crate::graphics::Config;
use crate::graphics::StateDrawer;
use crate::system::{State, System, SystemIterator};
use raylib::prelude::*;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Instant;

pub struct Visualizer {
    width: i32,
    height: i32,
    name: &'static str,
    system: Option<System>,
    config: Config,
}

impl Visualizer {
    const FONT_PATH: &str = "assets/Iosevka-Light.ttf";
    const FONT_SCALE: i32 = 30;
    const PADDING: i32 = 4;

    pub fn new(name: &'static str, (width, height): (i32, i32), config: Config) -> Self {
        Self {
            width,
            height,
            name,
            system: None,
            config,
        }
    }

    pub fn run(&mut self) {
        let (mut handle, thread) = self.init();
        let font = Self::load_font(&mut handle, &thread);
        handle.set_target_fps(20);

        let iterator = Arc::new(Mutex::new(self.system.clone().into_iter()));
        let duration = Arc::new(Mutex::new(0.0));
        let sender = self.create_iterator_channel(&iterator, &duration);

        let mut iteration = 1;

        while !handle.window_should_close() {
            if handle.is_window_resized() {
                self.resize(&handle);
            }

            {
                let mut d = handle.begin_drawing(&thread);
                if self.system.is_some() {
                    let iter = iterator.lock().unwrap();
                    let time = duration.lock().unwrap();
                    self.draw(&mut d, &font, iter.state(), iteration, *time);
                }
            }

            if handle.is_key_pressed(KeyboardKey::KEY_ENTER) {
                sender.send(Some(())).unwrap();
                iteration += 1;
            } else if handle.is_key_pressed(KeyboardKey::KEY_R) {
                let mut iter = iterator.lock().unwrap();
                *iter = self.system.clone().into_iter();
                let mut duration = duration.lock().unwrap();
                *duration = 0.0;
                iteration = 1;
            } else if handle.is_key_pressed(KeyboardKey::KEY_P)
                && handle.is_key_down(KeyboardKey::KEY_LEFT_SUPER)
            {
                handle.take_screenshot(&thread, "screenshot.png");
            }
        }

        sender.send(None).unwrap();
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

    fn draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        font: &Font,
        state: Option<&State>,
        iteration: i32,
        last_duration: f64,
    ) {
        d.clear_background(Color::new(24, 25, 26, 255));
        if let Some(state) = state {
            d.draw_state(
                state,
                self.width / 2,
                self.height,
                &mut self.config,
                Color::new(228, 230, 235, 255),
            );
        }
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
        if state.is_some() {
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
}
