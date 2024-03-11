#[derive(Debug)]
pub struct Config {
    pub line_length: i32,
    pub line_width_increment: f32,
    pub line_length_scale_factor: f64,
    pub turning_angle: f32,
    pub turning_angle_increment: f32,
}

impl Default for Config {
    fn default() -> Self {
        Config::new(5, 1.0, 1.1, 45.0, 45.0)
    }
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
