use crate::graphics::Config;
use crate::system::{State, Symbol, System};
use crate::{state, symbol};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

type LineIter = io::Lines<BufReader<File>>;

enum Section {
    Config,
    Rules,
    Start,
}

impl FromStr for Section {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "config" => Ok(Self::Config),
            "rules" => Ok(Self::Rules),
            "start" => Ok(Self::Start),
            _ => Err("Unrecognized section name.".to_string()),
        }
    }
}

pub fn parse(file_path: &str) -> Result<(Config, System), String> {
    let mut lines = create_buf_reader(file_path)?;
    let mut config: Option<Config> = None;
    let mut rules: Option<HashMap<Symbol, State>> = None;
    let mut start: Option<State> = None;

    while let Some(Ok(line)) = lines.next() {
        match line {
            ref line if is_header(&line) => match Section::from_str(&line[1..line.len() - 1])? {
                Section::Config => config = Some(parse_config(&mut lines)?),
                Section::Rules => rules = Some(parse_rules(&mut lines)?),
                Section::Start => start = Some(parse_start(&mut lines)?),
            },
            ref line if is_comment(&line) => continue,
            _ => return Err(format!("Invalid line: `{:?}`", line)),
        }
    }

    match (config, rules, start) {
        (Some(config), Some(rules), Some(start)) => Ok((config, System::new(rules, start))),
        _ => Err("Missing sections!".to_string()),
    }
}

fn create_buf_reader(file_path: &str) -> Result<LineIter, String> {
    let file = File::open(file_path).map_err(|_| "Could not open file".to_string())?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

fn is_header(line: &str) -> bool {
    line.starts_with("[") && line.ends_with("]")
}

fn is_comment(line: &str) -> bool {
    line.starts_with('#')
}

enum ConfigSetting {
    LineLength,
    LineWidthIncrement,
    LineLengthScaleFactor,
    TurningAngle,
    TurningAngleIncrement,
}

impl FromStr for ConfigSetting {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "line_length" => Ok(Self::LineLength),
            "line_width_increment" => Ok(Self::LineWidthIncrement),
            "line_length_scale_factor" => Ok(Self::LineLengthScaleFactor),
            "turning_angle" => Ok(Self::TurningAngle),
            "turning_angle_increment" => Ok(Self::TurningAngleIncrement),
            _ => Err(format!("Unrecognized config setting name: {}.", s)),
        }
    }
}

fn parse_config(lines: &mut LineIter) -> Result<Config, String> {
    let mut config = Config::default();

    while let Some(Ok(line)) = lines.next() {
        if line.trim().is_empty() {
            break;
        }

        let (setting, value) = parse_assigment(&line, "=")?;
        let setting = ConfigSetting::from_str(&setting)?;
        match setting {
            ConfigSetting::LineLength => {
                config.line_length = value
                    .parse()
                    .map_err(|_| "`line length` should be i32.".to_string())?
            }
            ConfigSetting::LineWidthIncrement => {
                config.line_width_increment = value
                    .parse()
                    .map_err(|_| "`line_width_increment` should be f32".to_string())?
            }
            ConfigSetting::LineLengthScaleFactor => {
                config.line_length_scale_factor = value
                    .parse()
                    .map_err(|_| "`line_length_scale_factor` should be f64".to_string())?
            }
            ConfigSetting::TurningAngle => {
                config.turning_angle = value
                    .parse::<f32>()
                    .map_err(|_| "`turning_angle` should be f32".to_string())?
                    .to_radians();
            }
            ConfigSetting::TurningAngleIncrement => {
                config.turning_angle_increment = value
                    .parse::<f32>()
                    .map_err(|_| "`turning_angle_increment` should be f32".to_string())?
                    .to_radians();
            }
        }
    }
    Ok(config)
}

fn parse_assigment<'a>(line: &'a str, delimiter: &str) -> Result<(&'a str, &'a str), String> {
    if line.matches(delimiter).count() != 1 {
        return Err(format!(
            "Assigment line: `{}`, does not contatin '{}'.",
            line, delimiter
        ));
    }

    let (key, value) = line.split_once(delimiter).unwrap();

    Ok((key.trim(), value.trim()))
}

fn parse_rules(lines: &mut LineIter) -> Result<HashMap<Symbol, State>, String> {
    let mut rules: HashMap<Symbol, State> = HashMap::new();
    while let Some(Ok(line)) = lines.next() {
        if line.trim().is_empty() {
            break;
        }
        let (symbol, state) = parse_assigment(&line, "->")?;
        if symbol.len() != 1 {
            return Err(format!("Symbol: `{}` isn't 1 character long.", symbol));
        }

        let symbol = symbol!(symbol.chars().next().unwrap());
        let state = state!(state);

        rules.insert(symbol, state);
    }

    Ok(rules)
}

enum StartSetting {
    Axiom,
    // TODO: Position and Angle
}

impl FromStr for StartSetting {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "axiom" => Ok(Self::Axiom),
            _ => Err(format!("Unrecognized start setting: {}.", s)),
        }
    }
}

fn parse_start(lines: &mut LineIter) -> Result<State, String> {
    let mut axiom: Option<State> = None;

    while let Some(Ok(line)) = lines.next() {
        if line.trim().is_empty() {
            break;
        }
        let (setting, state) = parse_assigment(&line, "=")?;
        let setting = StartSetting::from_str(&setting)?;
        match setting {
            StartSetting::Axiom => axiom = Some(state!(state)),
        }
    }

    match axiom {
        None => Err("Could not find `axiom`.".to_string()),
        Some(a) => Ok(a),
    }
}
