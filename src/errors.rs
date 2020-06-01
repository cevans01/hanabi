use thiserror::Error;

#[derive(Debug)]
pub struct Limits {
    pub lo: usize,
    pub hi: usize,
}

#[derive(Error, Debug)]
pub enum HanabiError {
    #[error("Int conversion failed")]
    IndexError(#[from] std::num::TryFromIntError),
    #[error("Invalid move: {0}")]
    InvalidMove(String),
    #[error("LogicError: {0}")]
    LogicError(String),
    #[error("Game is finished")]
    GameFinished,
    #[error("invalid index {idx}, expected at least {} and at most {}", .limits.lo, .limits.hi)]
    OutOfBounds { idx: usize, limits: Limits },
    #[error("unknown error")]
    Unknown,
}
