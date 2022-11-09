#![allow(dead_code)]

pub mod bitboard {
    use std::fmt;
    use std::ops::{BitAnd, BitOr, BitXor, Shl, Shr};

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct BitBoard {
        board: usize,
    }

    impl Default for BitBoard {
        fn default() -> Self {
            Self::new()
        }
    }

    impl BitAnd for BitBoard {
        type Output = BitBoard;

        fn bitand(self, rhs: Self) -> Self::Output {
            Self {
                board: self.board & rhs.board,
            }
        }
    }

    impl BitOr for BitBoard {
        type Output = BitBoard;

        fn bitor(self, rhs: Self) -> Self::Output {
            Self {
                board: self.board | rhs.board,
            }
        }
    }

    impl BitXor for BitBoard {
        type Output = BitBoard;

        fn bitxor(self, rhs: Self) -> Self::Output {
            Self {
                board: self.board ^ rhs.board,
            }
        }
    }

    impl BitAnd<usize> for BitBoard {
        type Output = BitBoard;

        fn bitand(self, rhs: usize) -> Self::Output {
            Self {
                board: self.board & rhs,
            }
        }
    }

    impl BitOr<usize> for BitBoard {
        type Output = BitBoard;

        fn bitor(self, rhs: usize) -> Self::Output {
            Self {
                board: self.board | rhs,
            }
        }
    }

    impl BitXor<usize> for BitBoard {
        type Output = BitBoard;

        fn bitxor(self, rhs: usize) -> Self::Output {
            Self {
                board: self.board ^ rhs,
            }
        }
    }

    impl Shl<usize> for BitBoard {
        type Output = BitBoard;

        fn shl(self, rhs: usize) -> Self::Output {
            BitBoard {
                board: self.board << rhs,
            }
        }
    }

    impl Shr<usize> for BitBoard {
        type Output = BitBoard;

        fn shr(self, rhs: usize) -> Self::Output {
            BitBoard {
                board: self.board >> rhs,
            }
        }
    }

    impl fmt::Display for BitBoard {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:b}", self.board)
        }
    }

    impl BitBoard {
        pub fn new() -> BitBoard {
            BitBoard { board: 0 }
        }

        pub fn get_value(&self) -> usize {
            self.board
        }

        pub const fn with_bits(board: usize) -> BitBoard {
            BitBoard { board }
        }
    }
}

pub mod tictactoe {
    use crate::bitboard::BitBoard;
    use std::fmt;

    const FILLED_BOARD: BitBoard = BitBoard::with_bits(0b111111111);
    const EMPTY_BOARD: BitBoard = BitBoard::with_bits(0);

    const WON_BOARDS: &[BitBoard; 8] = &[
        BitBoard::with_bits(0b111000000), //top horizontal
        BitBoard::with_bits(0b000111000), //mid horizontal
        BitBoard::with_bits(0b000000111), //bot horizontal
        BitBoard::with_bits(0b100100100), //left vertical
        BitBoard::with_bits(0b010010010), //mid vertical
        BitBoard::with_bits(0b001001001), //right vertical
        BitBoard::with_bits(0b100010001), //left-right diagonal
        BitBoard::with_bits(0b001010100), //right-left diagonal
    ];

    #[derive(Debug, Clone, Copy)]
    pub struct TicTacToeBoard {
        x_board: BitBoard,
        o_board: BitBoard,
    }

    impl Default for TicTacToeBoard {
        fn default() -> Self {
            Self::new()
        }
    }

    impl TicTacToeBoard {
        pub fn new() -> TicTacToeBoard {
            TicTacToeBoard {
                x_board: BitBoard::new(),
                o_board: BitBoard::new(),
            }
        }

        pub fn is_empty(&self) -> bool {
            (self.x_board | self.o_board) == EMPTY_BOARD
        }

        pub fn is_filled(&self) -> bool {
            (self.x_board | self.o_board) == FILLED_BOARD
        }

        pub fn already_played(&self, placement: usize) -> bool {
            ((self.x_board | self.o_board) & (1 << placement)).get_value() > 0
        }

        pub fn place_on_x_board(&self, placement: usize) -> Option<TicTacToeBoard> {
            if self.already_played(placement) {
                None
            } else {
                let board = self.x_board | (1 << placement);
                Some(TicTacToeBoard {
                    x_board: board,
                    o_board: self.o_board,
                })
            }
        }

        pub fn place_on_o_board(&self, placement: usize) -> Option<TicTacToeBoard> {
            if self.already_played(placement) {
                None
            } else {
                let board = self.o_board | (1 << placement);
                Some(TicTacToeBoard {
                    x_board: self.x_board,
                    o_board: board,
                })
            }
        }

        pub fn get_bit_boards(&self) -> (BitBoard, BitBoard) {
            (self.x_board, self.o_board)
        }
    }

    impl fmt::Display for TicTacToeBoard {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut output: Vec<String> = Vec::new();

            let piece_placement = |bit| {
                if ((self.x_board >> bit) & 1) != EMPTY_BOARD {
                    " X ".to_string()
                } else if ((self.o_board >> bit) & 1) != EMPTY_BOARD {
                    " O ".to_string()
                } else {
                    format!(" {} ", bit + 1)
                }
            };

            output.push(
                (0..3)
                    .map(|bit| piece_placement(bit))
                    .collect::<Vec<String>>()
                    .join("|"),
            );

            output.push(
                (3..6)
                    .map(|bit| piece_placement(bit))
                    .collect::<Vec<String>>()
                    .join("|"),
            );

            output.push(
                (6..9)
                    .map(|bit| piece_placement(bit))
                    .collect::<Vec<String>>()
                    .join("|"),
            );

            write!(f, "{}", output.join("\n---+---+---\n"))
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub enum PlayerSign {
        X,
        O,
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum GameStatus {
        XWon,
        OWon,
        Draw,
        StillGoing,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct GameState {
        board: TicTacToeBoard,
        current_player: PlayerSign,
        status: GameStatus,
    }

    impl fmt::Display for GameState {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let status = match self.status() {
                GameStatus::XWon => "X Wins!!",
                GameStatus::OWon => "O Wins!!",
                GameStatus::Draw => "Draw!",
                GameStatus::StillGoing => "Still playing.",
            };
            write!(
                f,
                "{}\nState: {}\nCurrent Player: {:?}",
                self.board, status, self.current_player
            )
        }
    }

    impl GameState {
        pub fn new() -> GameState {
            GameState {
                board: TicTacToeBoard::default(),
                current_player: PlayerSign::X,
                status: GameStatus::StillGoing,
            }
        }

        pub fn is_over(&self) -> bool {
            self.status() != GameStatus::StillGoing
        }

        pub fn status(&self) -> GameStatus {
            self.status
        }

        fn get_status(&self) -> GameStatus {
            for board_state in WON_BOARDS {
                if (self.board.x_board & *board_state) == *board_state {
                    return GameStatus::XWon;
                }
            }

            for board_state in WON_BOARDS {
                if (self.board.o_board & *board_state) == *board_state {
                    return GameStatus::OWon;
                }
            }

            if self.board.is_filled() {
                return GameStatus::Draw;
            }

            GameStatus::StillGoing
        }

        pub fn get_current_player(&self) -> PlayerSign {
            self.current_player
        }

        pub fn make_play(&self, placement: usize) -> Option<GameState> {
            if self.status() != GameStatus::StillGoing {
                return None;
            }
            if placement > 9 {
                return None;
            }
            let tttboard = match self.current_player {
                PlayerSign::X => self.board.place_on_x_board(placement - 1),
                PlayerSign::O => self.board.place_on_o_board(placement - 1),
            };

            let tttboard = match tttboard {
                Some(board) => board,
                None => return None,
            };

            let next_player = match self.current_player {
                PlayerSign::X => PlayerSign::O,
                PlayerSign::O => PlayerSign::X,
            };

            let mut game_state = GameState {
                board: tttboard,
                current_player: next_player,
                status: GameStatus::StillGoing,
            };
            game_state.status = game_state.get_status();

            Some(game_state)
        }
    }
}

use crate::tictactoe::GameState;
use std::io::{self, Write};

fn main() {
    let mut game = GameState::new();
    while !game.is_over() {
        println!("\n{}\n\n", game);
        print!("Place {:?} >> ", game.get_current_player());
        let _ = io::stdout().flush();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        let placement = buffer.trim().parse::<usize>().unwrap_or(10);

        game = game.make_play(placement).unwrap_or(game);
    }

    println!("\n{}", game);
}
