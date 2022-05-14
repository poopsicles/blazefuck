#![allow(non_camel_case_types)]

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Instruction {
    // Cell pointer tokens
    POINTER_RIGHT,
    POINTER_LEFT,

    // Data manipulation tokens
    DATA_INCREMENT,
    DATA_DECREMENT,
    DATA_OUTPUT,
    DATA_INPUT,

    // Conditional tokens
    CONDITIONAL_BEGIN,
    CONDITIONAL_END,
}

#[derive(Debug)]
pub enum Error {
    NO_COMMANDS,
    UNMATCHED_CONDITIONAL(usize),
    INPUT_OVERFLOW,

    // Non-strict errors
    CELL_BOUNDS_VIOLATED_LEFT,
    CELL_BOUNDS_VIOLATED_RIGHT,
    CELL_DATA_OVERFLOW,
    CELL_DATA_UNDERFLOW,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NO_COMMANDS => write!(f, "No commands found in the source file"),
            Error::UNMATCHED_CONDITIONAL(i) => write!(f, "Unmatched loop at command #{}", i),
            Error::INPUT_OVERFLOW => write!(f, "Data input was greater than the cell capacity (expected < 256)"),
            Error::CELL_BOUNDS_VIOLATED_LEFT => write!(f, "The cell pointer was moved to the left of cell #1"),
            Error::CELL_BOUNDS_VIOLATED_RIGHT => write!(f, "The cell pointer was moved to the right of cell #30000"),
            Error::CELL_DATA_OVERFLOW => write!(f, "The cell data was incremented past the cell capacity (expected < 256 bytes)"),
            Error::CELL_DATA_UNDERFLOW => write!(f, "The cell data was decremented past the cell capacity (expected >= 0 bytes)"),
        }
    }
}

impl std::error::Error for Error {}
