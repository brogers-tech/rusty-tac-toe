pub mod bitboard {
    use std::fmt;
    use std::ops::{BitAnd, BitOr, BitXor, Shl, Shr};

    #[derive(Default, Debug, Clone, Copy, PartialEq)]
    pub struct BitBoard {
        board: usize,
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

        pub const fn with_bits(board: usize) -> BitBoard {
            BitBoard { board }
        }

        pub fn get_bit(&self, bit: usize) -> usize {
            (self.board >> bit) & 1
        }
    }
}



pub mod Game {

    #[derive(Copy, Clone, Debug)]
    pub struct Action<T>(T);

    #[derive(Debug)]    
    pub struct Player<S: Strategy> {
        strategy: S,
    }

    #[derive(Debug)]
    pub enum Status<P: Playing> {
        Win(P),
        Draw,
        OnGoing,
    }

    impl <T> Action <T> {
        pub fn new(action: T) -> Action<T> {
            Action(action)
        }

        pub fn get(self) -> T {
            self.0
        }
    }

    impl <S: Strategy> Player<S> {
        fn new(strategy: S) -> Player<S> {
            Player { strategy }
        }
    
        fn make_move<G: GameState>(&self, game_state: &G) -> Action<S::Output> {
            self.strategy.decide_action(game_state)
        }
    }

    impl <S: Strategy> Playing for Player<S> {}

    //Partially inspired by 
    //Introduction to Artificial Intelligence
    //A Modern Approach
    //https://github.com/aimacode/aima-java/blob/AIMA3e/aima-core/src/main/java/aima/core/search/adversarial/Game.java
    pub trait GameState{
        type State;

        fn get_current_state(&self) -> Self::State;
        fn get_players<P: Playing>(&self) -> Vec<P>;
        fn is_over(&self) -> bool;
        fn get_actions<T>(&self) -> Vec<Action<T>>;
        fn do_action<T>(&self, action: Action<T>) -> Option<Self::State>;
        fn get_game_status<P: Playing>(&self) -> Status<P>;
        fn get_current_player<P: Playing>(&self) -> Option<P>;
    }

    pub trait Playing{}

    pub trait Strategy{
        type Output;
    
        fn decide_action<G>(&self, game_state: &G) -> Action<Self::Output>;
    }

}

pub mod tictactoe {
    extern crate colored;

    use crate::bitboard::BitBoard;
    use crate::Game::GameState;
    use colored::Colorize;
    use std::fmt;


    const FILLED_BOARD: BitBoard = BitBoard::with_bits(0b111111111);
    const EMPTY_BOARD: BitBoard = BitBoard::with_bits(0);

    const H_EDGE: &str = "\u{2500}";
    const V_EDGE:  &str = "\u{2502}";
    const CONNECTOR:  &str = "\u{253C}";

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

    #[derive(Default, Debug, Clone, Copy)]
    pub struct TicTacToeBoard {
        x_board: BitBoard,
        o_board: BitBoard,
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
            (self.x_board | self.o_board) & (1 << placement) != EMPTY_BOARD
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
                    " X ".bright_red()
                } else if ((self.o_board >> bit) & 1) != EMPTY_BOARD {
                    " O ".bright_blue()
                } else {
                    format!(" {} ", bit + 1).bold().on_cyan()
                }
            };

            output.push(
                (0..3)
                    .map(|bit| piece_placement(bit).to_string())
                    .collect::<Vec<String>>()
                    .join(V_EDGE),
            );

            output.push(
                (3..6)
                    .map(|bit| piece_placement(bit).to_string())
                    .collect::<Vec<String>>()
                    .join(V_EDGE),
            );

            output.push(
                (6..9)
                    .map(|bit| piece_placement(bit).to_string())
                    .collect::<Vec<String>>()
                    .join(V_EDGE),
            );

            let wall = H_EDGE.repeat(3);
            let divider = format!("\n{div}{con}{div}{con}{div}\n", div=wall, con=CONNECTOR);
            write!(f, "{}", output.join(&divider))
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
    pub struct TicTacToeGame {
        board: TicTacToeBoard,
        current_player: PlayerSign,
        status: GameStatus,
    }

    impl fmt::Display for TicTacToeGame {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let status = match self.current_status() {
                GameStatus::XWon => "X Wins!!".green().blink(),
                GameStatus::OWon => "O Wins!!".green().blink(),
                GameStatus::Draw => "Draw!".purple().italic(),
                GameStatus::StillGoing => "Still playing.".bold(),
            };

            let player = match self.current_player {
                PlayerSign::X => "X",
                PlayerSign::O => "O",
            };

            write!(
                f,
                "{}\nState: {}\nCurrent Player: {}",
                self.board,
                status,
                if !self.is_over() { player } else { "--" }
            )
        }
    }

    impl Default for TicTacToeGame {
        fn default() -> Self {
            Self::new()
        }
    }

    impl TicTacToeGame {
        pub fn new() -> TicTacToeGame {
            TicTacToeGame {
                board: TicTacToeBoard::default(),
                current_player: PlayerSign::X,
                status: GameStatus::StillGoing,
            }
        }

        pub fn is_over(&self) -> bool {
            self.current_status() != GameStatus::StillGoing
        }

        pub fn current_status(&self) -> GameStatus {
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

        pub fn get_moves(&self) -> Vec<usize> {
            let current_places = self.board.x_board | self.board.o_board;
            let mut moves = Vec::new();
            for b in 0..9 {
                let bit = current_places.get_bit(b);
                if bit == 0 {
                    moves.push(b);
                }
            }

            moves
        }

        pub fn make_play(&self, placement: usize) -> Option<TicTacToeGame> {
            if self.get_status() != GameStatus::StillGoing {
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

            let mut game_state = TicTacToeGame {
                board: tttboard,
                current_player: next_player,
                status: GameStatus::StillGoing,
            };
            game_state.status = game_state.get_status();

            Some(game_state)
        }
    }
}

use crate::tictactoe::TicTacToeGame;
use std::io::{self, Write};

//https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed
fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn main() {
    let mut game = TicTacToeGame::default();

    while !game.is_over() {
        
        clear_screen();
        
        let player_moves = game.get_moves().iter().map(|m| m + 1).collect::<Vec<usize>>();

        println!("\n{}\n\n", game);
        println!("Possible choices: {:?}", player_moves);
        print!("Place {:?} >> ", game.get_current_player());
        let _ = io::stdout().flush();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        let placement = buffer.trim().parse::<usize>().unwrap_or(10);

        game = game.make_play(placement).unwrap_or(game);
    }

    clear_screen();
    println!("\n{}", game);
}
