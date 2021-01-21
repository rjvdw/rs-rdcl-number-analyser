//! Struct that holds a number and its sign.

use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
use std::{fmt, num};

/// Holds a number and its sign.
#[derive(Copy, Clone)]
pub(crate) struct Number {
    pub(crate) value: u128,
    pub(crate) is_negative: bool,
}

// impl Number {
//     pub fn sign<'a>(self) -> &'a str {
//         if self.is_negative {
//             "-"
//         } else {
//             ""
//         }
//     }
// }

impl FromStr for Number {
    type Err = NumberParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(r) = s.strip_prefix('-') {
            Ok(Number {
                value: r.parse()?,
                is_negative: true,
            })
        } else {
            Ok(Number {
                value: s.parse()?,
                is_negative: false,
            })
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_negative {
            write!(f, "-{}", self.value)
        } else {
            write!(f, "{}", self.value)
        }
    }
}

/// Represents possible ways in which parsing a number could go wrong.
pub enum NumberParseError {
    ParseIntError(num::ParseIntError),
}

impl From<num::ParseIntError> for NumberParseError {
    fn from(err: ParseIntError) -> Self {
        NumberParseError::ParseIntError(err)
    }
}

impl Display for NumberParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NumberParseError::ParseIntError(err) => write!(f, "{}", err),
        }
    }
}
