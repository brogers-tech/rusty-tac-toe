use std::ops::{BitAnd,BitOr,BitXor};
use std::fmt;

#[derive(Debug,Clone,Copy)]
struct BitBoard {
    board: usize,
}

impl BitAnd for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self { board: self.board & rhs.board }
    }
}

impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self { board: self.board | rhs.board }
    }
}

impl BitXor for BitBoard {
    type Output = BitBoard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { board: self.board ^ rhs.board }
    }
}

impl BitAnd<usize> for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: usize) -> Self::Output {
        Self { board: self.board & rhs }
    }
}

impl BitOr<usize> for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: usize) -> Self::Output {
        Self { board: self.board | rhs }
    }
}

impl BitXor<usize> for BitBoard {
    type Output = BitBoard;

    fn bitxor(self, rhs: usize) -> Self::Output {
        Self { board: self.board ^ rhs }
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:b}", self.board)
    }
}

impl BitBoard {
    fn new() -> BitBoard {
        BitBoard { board: 0}
    }
}

fn main() {}
