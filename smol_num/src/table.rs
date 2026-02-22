#[derive(Clone)]
pub struct Point(pub u16);

pub trait PointsToString {
    fn from_points(value: Vec<Point>) -> String {
        if value.len() == 0 { return " ".repeat(6); }

        let mut buf = String::new();
        let mut last_x = 0;

        for (i, c) in value.iter().enumerate() {
            if i == 0 {
                buf.push_str(&" ".repeat(c.0 as usize));
            } else if let fill = c.0 - last_x - 1 && fill > 0 {
                buf.push_str(&" ".repeat(fill as usize));
            }
            buf.push('█');
            last_x = c.0;
        }

        if last_x < 5 {
            let rest = 5 - last_x;
            buf.push_str(&" ".repeat(rest as usize));
        }

        buf
    }
}

impl PointsToString for String {}
type Lines = [&'static [Point]; 5];

const fn fill_line() -> [Point; 6] {
    [
        Point(0),
        Point(1),
        Point(2),
        Point(3),
        Point(4),
        Point(5),
    ]
}

/// the big 0
/// grid:
///   012345
/// 0 xxxxxx
/// 1 xx  xx
/// 2 xx  xx
/// 3 xx  xx
/// 4 xxxxxx
pub const NUMBER_ZERO: Lines = [
    &fill_line(),

    &[
        Point(0),
        Point(1),
        Point(4),
        Point(5),
    ],

    &[
        Point(0),
        Point(1),
        Point(4),
        Point(5),
    ],

    &[
        Point(0),
        Point(1),
        Point(4),
        Point(5),
    ],

    &fill_line(),
];

/// the big 1
/// grid:
///   012345
/// 0     xx
/// 1     xx
/// 2     xx
/// 3     xx
/// 4     xx
pub const NUMBER_ONE: Lines = [
    &[
        Point(4),
        Point(5),
    ],

    &[
        Point(4),
        Point(5),
    ],

    &[
        Point(4),
        Point(5),
    ],

    &[
        Point(4),
        Point(5),
    ],

    &[
        Point(4),
        Point(5),
    ]
];

/// the big 2
/// grid:
///   012345
/// 0 xxxxxx
/// 1     xx
/// 2 xxxxxx
/// 3 xx
/// 4 xxxxxx
pub const NUMBER_TWO: Lines = [
    &fill_line(),

    &[
        Point(4),
        Point(5),
    ],

    &fill_line(),

    &[
        Point(0),
        Point(1),
    ],

    &fill_line()
];

/// the big 3
/// grid:
///   012345
/// 0 xxxxxx
/// 1     xx
/// 2 xxxxxx
/// 3     xx
/// 4 xxxxxx
pub const NUMBER_THREE: Lines = [
    &fill_line(),

    &[
        Point(4),
        Point(5),
    ],

    &fill_line(),

    &[
        Point(4),
        Point(5),
    ],

    &fill_line()
];

/// the big 4
/// grid:
///   012345
/// 0 xx  xx
/// 1 xx  xx
/// 2 xxxxxx
/// 3     xx
/// 4     xx
pub const NUMBER_FOUR: Lines = [
    &[
        Point(0),
        Point(1),
        Point(4),
        Point(5),
    ],

    &[
        Point(0),
        Point(1),
        Point(4),
        Point(5),
    ],

    &[
        Point(0),
        Point(1),
        Point(2),
        Point(3),
        Point(4),
        Point(5),
    ],

    &[
        Point(4),
        Point(5),
    ],

    &[
        Point(4),
        Point(5),
    ],
];

/// the big 5
/// grid:
///   012345
/// 0 xxxxxx
/// 1 xx
/// 2 xxxxxx
/// 3     xx
/// 4 xxxxxx
pub const NUMBER_FIVE: Lines = [
    &fill_line(),

    &[
        Point(0),
        Point(1),
    ],

    &fill_line(),

    &[
        Point(4),
        Point(5),
    ],

    &fill_line(),
];

/// the big 6
/// grid:
///   012345
/// 0 xxxxxx
/// 1 xx
/// 2 xxxxxx
/// 3 xx  xx
/// 4 xxxxxx
pub const NUMBER_SIX: Lines = [
    &fill_line(),

    &[
        Point(0),
        Point(1),
    ],

    &fill_line(),

    &[
        Point(0),
        Point(1),
        Point(4),
        Point(5),
    ],

    &fill_line(),
];

/// the big 7
/// grid:
///   012345
/// 0 xxxxxx
/// 1     xx
/// 2     xx
/// 3     xx
/// 4     xx
pub const NUMBER_SEVEN: Lines = [
    &fill_line(),

    &[
        Point(4),
        Point(5),
    ],

    &[
        Point(4),
        Point(5),
    ],

    &[
        Point(4),
        Point(5),
    ],

    &[
        Point(4),
        Point(5),
    ],
];

/// the big 8
/// grid:
///   012345
/// 0 xxxxxx
/// 1 xx  xx
/// 2 xxxxxx
/// 3 xx  xx
/// 4 xxxxxx
pub const NUMBER_EIGHT: Lines = [
    &fill_line(),

    &[
        Point(0),
        Point(1),
        Point(4),
        Point(5),
    ],

    &fill_line(),

    &[
        Point(0),
        Point(1),
        Point(4),
        Point(5),
    ],

    &fill_line(),
];

/// the big 9
/// grid:
///   012345
/// 0 xxxxxx
/// 1 xx  xx
/// 2 xxxxxx
/// 3     xx
/// 4 xxxxxx
pub const NUMBER_NINE: Lines = [
    &fill_line(),

    &[
        Point(0),
        Point(1),
        Point(4),
        Point(5),
    ],

    &fill_line(),

    &[
        Point(4),
        Point(5),
    ],

    &fill_line(),
];

/// colon
/// grid:
///   012345
/// 0
/// 1   xx
/// 2
/// 3   xx
/// 4
pub const SYMBOL_COLON: Lines = [
    &[],

    &[
        Point(2),
        Point(3),
    ],

    &[],

    &[
        Point(2),
        Point(3),
    ],

    &[],
];

#[derive(PartialEq)]
pub enum NumberVariant {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Colon,
}

impl TryFrom<char> for NumberVariant {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::One),
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            ':' => Ok(Self::Colon),
            _   => Err(())
        }
    }
}

impl NumberVariant {
    pub fn get_points(&self, line: usize) -> Vec<Point> {
        match *self {
            Self::Zero  => NUMBER_ZERO,
            Self::One   => NUMBER_ONE,
            Self::Two   => NUMBER_TWO,
            Self::Three => NUMBER_THREE,
            Self::Four  => NUMBER_FOUR,
            Self::Five  => NUMBER_FIVE,
            Self::Six   => NUMBER_SIX,
            Self::Seven => NUMBER_SEVEN,
            Self::Eight => NUMBER_EIGHT,
            Self::Nine  => NUMBER_NINE,
            Self::Colon => SYMBOL_COLON,
        }.get(line).map(|p| p.to_vec()).unwrap_or_default()
    }
}

