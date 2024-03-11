use l_system::graphics::Visualizer;
use l_system::parser;

fn main() -> Result<(), String> {
    let (config, system) = parser::parse("systems/branch.lsys")?;

    let mut visualizer = Visualizer::new("L-Systems", (640, 480), system, config);
    visualizer.run();

    Ok(())
}
