use core::fmt;

// Smybol           Meaning
// -------------------------------------------------------------------------
//    F             Move forward by line length drawing a line
//    f             Move forward by line length without drawing a line
//    +             Turn left by turning angle
//    -             Turn right by turning angle
//    |             Reverse direction (ie: turn by 180 degrees)
//    [             Push current drawing state onto stack
//    ]             Pop current drawing state from the stack
//    #             Increment the line width by line width increment
//    !             Decrement the line width by line width increment
//    @             Draw a dot with line width radius
//    {             Open a polygon
//    }             Close a polygon and fill it with fill colour
//    >             Multiply the line length by the line length scale factor
//    <             Divide the line length by the line length scale factor
//    &             Swap the meaning of + and -
//    (             Decrement turning angle by turning angle increment
//    )             Increment turning angle by turning angle increment
//
// source: http://www.paulbourke.net/fractals/lsys/

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum Symbol {
    Var(char),
    Draw,
    Move,
    Left,
    Right,
    Reverse,
    Push,
    Pop,
    IncLine,
    DecLine,
    Dot,
    OpenPolygon,
    ClosePolygon,
    MulLine,
    DivLine,
    SwapOperations,
    IncAngle,
    DecAngle,
}

impl From<char> for Symbol {
    fn from(c: char) -> Self {
        match c {
            'F' => Self::Draw,
            'f' => Self::Move,
            '+' => Self::Left,
            '-' => Self::Right,
            '|' => Self::Reverse,
            '[' => Self::Push,
            ']' => Self::Pop,
            '#' => Self::IncLine,
            '!' => Self::DecLine,
            '@' => Self::Dot,
            '{' => Self::OpenPolygon,
            '}' => Self::ClosePolygon,
            '>' => Self::MulLine,
            '<' => Self::DivLine,
            '&' => Self::SwapOperations,
            '(' => Self::IncAngle,
            ')' => Self::DecAngle,
            c => Self::Var(c),
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Draw => 'F',
            Self::Move => 'f',
            Self::Left => '+',
            Self::Right => '-',
            Self::Reverse => '|',
            Self::Push => '[',
            Self::Pop => ']',
            Self::IncLine => '#',
            Self::DecLine => '!',
            Self::Dot => '@',
            Self::OpenPolygon => '{',
            Self::ClosePolygon => '}',
            Self::MulLine => '>',
            Self::DivLine => '<',
            Self::SwapOperations => '&',
            Self::IncAngle => '(',
            Self::DecAngle => ')',
            Self::Var(c) => *c,
        };
        write!(f, "{}", c)
    }
}
