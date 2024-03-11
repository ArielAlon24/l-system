use crate::graphics::Config;
use crate::system::{State, Symbol};
use raylib::prelude::*;

pub trait StateDrawer {
    fn draw_state(&mut self, state: &State, x: i32, y: i32, config: &mut Config, color: Color);
}

impl StateDrawer for RaylibDrawHandle<'_> {
    fn draw_state(&mut self, state: &State, x: i32, y: i32, config: &mut Config, color: Color) {
        let mut pos = Vector2::new(x as f32, y as f32);
        let mut angle = std::f32::consts::PI / 2.0;

        let mut stack = Vec::<(Vector2, f32)>::new();
        let mut thickness = 1.0;

        for symbol in state {
            match symbol {
                Symbol::Var(_) => {}
                // TODO: Draw only inside of the screen bounds
                Symbol::Draw => {
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
                Symbol::Move => {
                    pos = Vector2::new(
                        pos.x - config.line_length as f32 * angle.cos(),
                        pos.y - config.line_length as f32 * angle.sin(),
                    )
                }
                Symbol::Left => angle -= config.turning_angle,
                Symbol::Right => angle += config.turning_angle,
                Symbol::Reverse => angle += std::f32::consts::PI,
                Symbol::Push => stack.push((pos, angle)),
                Symbol::Pop => {
                    if !stack.is_empty() {
                        (pos, angle) = stack.pop().unwrap();
                    }
                }
                Symbol::IncLine => thickness += config.line_width_increment,
                Symbol::DecLine => thickness -= config.line_width_increment,
                Symbol::Dot => {
                    self.draw_circle(pos.x as i32, pos.y as i32, config.line_length as f32, color)
                }
                Symbol::OpenPolygon => todo!(),
                Symbol::ClosePolygon => todo!(),
                Symbol::MulLine => {
                    config.line_length =
                        (config.line_length as f64 * config.line_length_scale_factor) as i32
                }
                Symbol::DivLine => {
                    config.line_length =
                        (config.line_length as f64 * config.line_length_scale_factor) as i32
                }
                Symbol::SwapOperations => todo!(),
                Symbol::IncAngle => angle += config.turning_angle_increment,
                Symbol::DecAngle => angle -= config.turning_angle_increment,
            }
        }
    }
}
