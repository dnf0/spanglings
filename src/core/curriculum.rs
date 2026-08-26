use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Level {
    Baseline,
    B1,
    B2,
    C1,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Level::Baseline => "Baseline",
            Level::B1 => "B1",
            Level::B2 => "B2",
            Level::C1 => "C1",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("invalid level: {0}")]
pub struct ParseLevelError(pub String);

impl FromStr for Level {
    type Err = ParseLevelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "Baseline" | "baseline" => Ok(Level::Baseline),
            "B1" | "b1" => Ok(Level::B1),
            "B2" | "b2" => Ok(Level::B2),
            "C1" | "c1" => Ok(Level::C1),
            other => Err(ParseLevelError(other.to_string())),
        }
    }
}
