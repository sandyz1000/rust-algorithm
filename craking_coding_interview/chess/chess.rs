#![allow(unused)]

use std::collections::HashMap;
use std::io;
use std::io::Result;
use std::num;
use std::ops::Range;

#[derive(Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Empty,
}

#[derive(Copy, Clone)]
pub struct Piece {
    piece_type: PieceType,
    player: bool,
}

impl Piece {
    pub fn new(piece_type: PieceType, player: bool) -> Self {
        Piece { piece_type, player }
    }

    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }
    pub fn player(&self) -> bool {
        self.player
    }

    pub fn to_string(self) -> String {
        return if !self.player {
            match self.piece_type {
                PieceType::Pawn => "1P".to_string(),
                PieceType::Rook => "1R".to_string(),
                PieceType::Knight => "1N".to_string(),
                PieceType::Bishop => "1B".to_string(),
                PieceType::Queen => "1Q".to_string(),
                PieceType::King => "1K".to_string(),
                PieceType::Empty => " ".to_string(),
            }
        } else {
            match self.piece_type {
                PieceType::Pawn => "2P︎".to_string(),
                PieceType::Rook => "2R".to_string(),
                PieceType::Knight => "2N".to_string(),
                PieceType::Bishop => "2B".to_string(),
                PieceType::Queen => "2Q".to_string(),
                PieceType::King => "2K".to_string(),
                PieceType::Empty => " ".to_string(),
            }
        };
    }
}

pub struct BoardState {
    board: HashMap<u8, Piece>,
}

impl BoardState {
    pub fn new() -> Self {
        BoardState {
            board: HashMap::new(),
        }
    }

    pub fn reset(&mut self) {
        self.board.clear();

        self.board.insert(0, Piece::new(PieceType::Rook, false));
        self.board.insert(1, Piece::new(PieceType::Knight, false));
        self.board.insert(2, Piece::new(PieceType::Bishop, false));
        self.board.insert(3, Piece::new(PieceType::King, false));
        self.board.insert(4, Piece::new(PieceType::Queen, false));
        self.board.insert(5, Piece::new(PieceType::Bishop, false));
        self.board.insert(6, Piece::new(PieceType::Knight, false));
        self.board.insert(7, Piece::new(PieceType::Rook, false));

        for i in 8..16 {
            self.board.insert(i, Piece::new(PieceType::Pawn, false));
        }

        for i in 48..56 {
            self.board.insert(i, Piece::new(PieceType::Pawn, true));
        }

        self.board.insert(56, Piece::new(PieceType::Rook, true));
        self.board.insert(57, Piece::new(PieceType::Knight, true));
        self.board.insert(58, Piece::new(PieceType::Bishop, true));
        self.board.insert(59, Piece::new(PieceType::King, true));
        self.board.insert(60, Piece::new(PieceType::Queen, true));
        self.board.insert(61, Piece::new(PieceType::Bishop, true));
        self.board.insert(62, Piece::new(PieceType::Knight, true));
        self.board.insert(63, Piece::new(PieceType::Rook, true));
    }

    pub fn draw_board(&self) {
        println!("    0    1    2    3    4    5    6    7   ");
        println!("  -----------------------------------------");

        for y in 0u8..8 {
            print!("{} ", y);
            for x in 0u8..8 {
                if let Some(icon) = self.board.get(&(x + (8 * y))) {
                    print!("| {} ", icon.to_string());
                } else {
                    print!("|    ");
                }
            }
            println!("|");
            println!("  -----------------------------------------");
        }
    }
}

impl BoardState {
    // returns type of piece at (x,y) as a enumerated type of PieceType
    pub fn piece_type_at(&self, x: u8, y: u8) -> PieceType {
        return if let Some(piece) = self.board.get(&(x + (8 * y))) {
            piece.piece_type()
        } else {
            PieceType::Empty
        };
    }

    // returns the player (0 or 1) who owns the piece at (x, y) or -1 if no piece at (x, y)
    pub fn piece_player_at(&self, x: u8, y: u8) -> i8 {
        if let Some(piece) = self.board.get(&(x + (8 * y))) {
            if piece.player() {
                1
            } else {
                0
            }
        } else {
            return -1;
        }
    }

    // returns true if piece successfully moved from (x, y) to (x2, y2)
    pub fn move_piece(&mut self, x: u8, y: u8, x2: u8, y2: u8) -> bool {
        return if let Some(p) = self.board.get(&(x + (8 * y))) {
            match p.piece_type() {
                PieceType::Empty => false,
                PieceType::Pawn => self.move_pawn(x, y, x2, y2),
                PieceType::Rook => self.move_rook(x, y, x2, y2),
                PieceType::Knight => self.move_knight(x, y, x2, y2),
                PieceType::Bishop => self.move_bishop(x, y, x2, y2),
                PieceType::King => self.move_king(x, y, x2, y2),
                PieceType::Queen => self.move_queen(x, y, x2, y2),
            }
        } else {
            false
        };
    }

    fn move_pawn(&mut self, x: u8, y: u8, x2: u8, y2: u8) -> bool {
        return if self.is_valid_move_pawn(x, y, x2, y2) {
            if let Some(temp) = self.board.remove(&(x + (8 * y))) {
                self.board.remove(&(x2 + (8 * y2)));
                self.board.insert((x2 + (8 * y2)), temp);

                true
            } else {
                false
            }
        } else {
            false
        };
    }

    fn move_rook(&mut self, x: u8, y: u8, x2: u8, y2: u8) -> bool {
        return if self.is_valid_move_rook(x, y, x2, y2) {
            if let Some(temp) = self.board.remove(&(x + (8 * y))) {
                self.board.remove(&(x2 + (8 * y2)));
                self.board.insert((x2 + (8 * y2)), temp);

                true
            } else {
                false
            }
        } else {
            false
        };
    }

    fn move_knight(&mut self, x: u8, y: u8, x2: u8, y2: u8) -> bool {
        return if self.is_valid_move_knight(x, y, x2, y2) {
            if let Some(temp) = self.board.remove(&(x + (8 * y))) {
                self.board.remove(&(x2 + (8 * y2)));
                self.board.insert((x2 + (8 * y2)), temp);

                true
            } else {
                false
            }
        } else {
            false
        };
    }

    fn move_bishop(&mut self, x: u8, y: u8, x2: u8, y2: u8) -> bool {
        return if self.is_valid_move_bishop(x, y, x2, y2) {
            if let Some(temp) = self.board.remove(&(x + (8 * y))) {
                self.board.remove(&(x2 + (8 * y2)));
                self.board.insert((x2 + (8 * y2)), temp);

                true
            } else {
                false
            }
        } else {
            false
        };
    }

    fn move_king(&mut self, x: u8, y: u8, x2: u8, y2: u8) -> bool {
        return if self.is_valid_move_king(x, y, x2, y2) {
            if let Some(temp) = self.board.remove(&(x + (8 * y))) {
                self.board.remove(&(x2 + (8 * y2)));
                self.board.insert((x2 + (8 * y2)), temp);

                true
            } else {
                false
            }
        } else {
            false
        };
    }

    fn move_queen(&mut self, x: u8, y: u8, x2: u8, y2: u8) -> bool {
        return if self.is_valid_move_queen(x, y, x2, y2) {
            if let Some(temp) = self.board.remove(&(x + (8 * y))) {
                self.board.remove(&(x2 + (8 * y2)));
                self.board.insert((x2 + (8 * y2)), temp);

                true
            } else {
                false
            }
        } else {
            false
        };
    }

    pub fn is_valid_move(&self, x: u8, y: u8, x2: u8, y2: u8) -> bool {
        return if let Some(p) = self.board.get(&(x + (8 * y))) {
            match p.piece_type() {
                PieceType::Empty => false,
                PieceType::Pawn => self.is_valid_move_pawn(x, y, x2, y2),
                PieceType::Rook => self.is_valid_move_rook(x, y, x2, y2),
                PieceType::Knight => self.is_valid_move_knight(x, y, x2, y2),
                PieceType::Bishop => self.is_valid_move_bishop(x, y, x2, y2),
                PieceType::King => self.is_valid_move_king(x, y, x2, y2),
                PieceType::Queen => self.is_valid_move_queen(x, y, x2, y2),
            }
        } else {
            false
        };
    }

    fn is_valid_move_pawn(&self, x: u8, y: u8, x2: u8, y2: u8) -> bool {
        // can move forward 1 or 2 if at starting location and NOT blocked by piece
        // can move diagonally forward 1 ONLY IF blocked by piece of enemy player

        if let Some(p) = self.board.get(&(x + (8 * y))) {
            if x2 < 0 || x2 > 7 || y2 < 0 || y2 > 7 {
                return false;
            } // if movement outside of board bounds

            if p.player() == false
            // player 0
            {
                // if move forward
                if x == x2 {
                    // if moving forward 1 piece
                    if y2 == y + 1 {
                        return self.piece_type_at(x, y + 1) == PieceType::Empty;
                    }
                    // if moving forward 2 piece
                    else if y2 == y + 2 && y == 1 {
                        self.piece_type_at(x, y + 1) == PieceType::Empty
                            && self.piece_type_at(x, y + 2) == PieceType::Empty
                    }
                    // if not moving forward 1 or 2 spaces
                    else {
                        false
                    }
                }
                // if attacking diagonally
                else if abs_dif(x, x2) == 1 && y + 1 == y2 {
                    return self.piece_player_at(x2, y2) == 1;
                } else {
                    false
                }
            } else
            // player 1
            {
                // if move forward
                if x == x2 {
                    // if moving forward 1 piece
                    if y2 == y - 1 {
                        self.piece_type_at(x, y - 1) == PieceType::Empty
                    }
                    // if moving forward 2 piece
                    else if y2 == y - 2 && y == 6 {
                        self.piece_type_at(x, y - 1) == PieceType::Empty
                            && self.piece_type_at(x, y - 2) == PieceType::Empty
                    }
                    // if not moving forward 1 or 2 spaces
                    else {
                        false
                    }
                }
                // if attacking diagonally
                else if abs_dif(x, x2) == 1 && y - 1 == y2 {
                    return self.piece_player_at(x2, y2) == 1;
                } else {
                    false
                }
            }
        } else {
            false
        }
    }

    fn is_valid_move_rook(&self, x: u8, y: u8, x2: u8, y2: u8) -> bool {
        // can move 4 ways, optionally move into piece owned by enemy player

        if x2 < 0 || x2 > 7 || y2 < 0 || y2 > 7 {
            return false;
        } // if movement outside of board bounds

        return if x == x2
        // if vertically aligned
        {
            for i in y + 1..y2 {
                // if non-empty space encountered before destination, then invalid move
                if self.piece_type_at(x, i) != PieceType::Empty {
                    return false;
                }
            }

            // if path to destination is empty AND destination is empty OR enemy, then move is valid
            if self.piece_player_at(x2, y2) != self.piece_player_at(x, y) {
                true
            } else {
                false
            }
        } else if y == y2
        // if horizontally aligned
        {
            for i in x + 1..x2 {
                // if non-empty space encountered before destination, then invalid move
                if self.piece_type_at(y, i) != PieceType::Empty {
                    return false;
                }
            }

            // if path to destination is empty AND destination is empty OR enemy, then move is valid
            return if self.piece_player_at(x2, y2) != self.piece_player_at(x, y) {
                true
            } else {
                false
            };
        } else {
            false
        }; // if not aligned, false

        // if x = x2 then moving along y axis
        //      all spaces from between y and y2 are empty, and (x2, y2) is empty or enemy
        //          true
        //      else
        //          false
        // if y = y2 then moving along x axis
        //      all spaces from between x and x2 are empty, and (x2, y2) is empty or enemy
        //          true
        //      else
        //          false
        // if does not align with x or y axis, invalid
    }

    fn is_valid_move_knight(&self, x: u8, y: u8, x2: u8, y2: u8) -> bool {
        if x2 < 0 || x2 > 7 || y2 < 0 || y2 > 7 {
            return false;
        } // if movement outside of board bounds

        // can move in L pattern in any direction, jumping over pieces, can only take enemy player piece

        // if L pattern away
        return if (abs_dif(x, x2) == 1 && abs_dif(y, y2) == 2)
            || (abs_dif(x, x2) == 2 && abs_dif(y, y2) == 1)
        {
            if self.piece_player_at(x2, y2) != self.piece_player_at(x, y) {
                true
            } else {
                false
            }
        } else {
            false
        };

        // if (x2, y2) is L jump away, and destination is empty or enemy
        //      true
        // else
        //      false
    }

    fn is_valid_move_bishop(&self, x: u8, y: u8, x2: u8, y2: u8) -> bool {
        if x2 < 0 || x2 > 7 || y2 < 0 || y2 > 7 {
            return false;
        }
        // if movement outside of board bounds
        else if x == x2 && y == y2 {
            return false;
        } // if destination == origin

        if abs_dif(x, x2) == abs_dif(y, y2) {
            // if every space between origin and destination is empty

            let mut temp_x: Range<u8>;
            let mut temp_y: Range<u8>;

            if x2 < x {
                temp_x = (x - 1)..x2;
            } else {
                temp_x = (x + 1)..x2;
            }

            if y2 < y {
                temp_y = (y - 1)..y2;
            } else {
                temp_y = (y + 1)..y2;
            }

            for (X, Y) in temp_x.zip(temp_y)
            // check every square between origin and destination
            {
                if self.piece_type_at(X, Y) != PieceType::Empty {
                    return false;
                }
            }

            return if self.piece_player_at(x2, y2) != self.piece_player_at(x, y) {
                true
            } else {
                false
            };

            // AND if destination is empty or enemy
        }

        // can move 4 ways diagonally, until contact with piece is made, can only take enemy player piece

        false
    }

    fn is_valid_move_king(&self, x: u8, y: u8, x2: u8, y2: u8) -> bool {
        // can move 8 ways, one space, can only take enemy player piece

        if x2 < 0 || x2 > 7 || y2 < 0 || y2 > 7 {
            return false;
        } // if movement outside of board bounds

        return if abs_dif(x, x2) <= 1 && abs_dif(y, y2) <= 1
        // if within 1 square
        {
            if self.piece_player_at(x2, y2) != self.piece_player_at(x, y)
            // if destination is empty or enemy piece
            {
                true
            } else {
                false
            }
        } else {
            false
        };

        // if (x2, y2) is within 1 square, and is empty or enemy
        //      true
        // else
        //      false
    }

    fn is_valid_move_queen(&self, x: u8, y: u8, x2: u8, y2: u8) -> bool {
        // can move 8 ways, until contact is made, can only take enemy player piece

        return if self.is_valid_move_bishop(x, y, x2, y2) || self.is_valid_move_rook(x, y, x2, y2) {
            true
        } else {
            false
        };

        // if (x2, y2) is along horizontal/vertical
        //      reuse Rook logic
        // if (x2, y2) is along diagonal
        //      reuse bishop logic
        // else
        //      false
    }
}

fn abs_dif(x: u8, y: u8) -> u8 {
    return (x as i16 - y as i16).abs() as u8;
}

fn main() {
    let mut board = BoardState::new();

    board.reset();
    board.draw_board();

    let help_message = "move <origin X> <origin Y> <destination X> <destination Y>\n\
        valid_move <origin X> <origin Y> <destination X> <destination Y>\n\
        draw\n\
        reset\n\
        exit\n\
        help";

    loop {
        println!(":");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line"); // read input

        loop {
            if input.ends_with('\n') || input.ends_with('\r') || input.ends_with(' ') {
                input.pop();
            } else {
                break;
            }
        }

        let vector: Vec<&str> = input.split(' ').collect();

        match vector.get(0) {
            Some(&"move") => {
                call_move(&mut board, &vector);
            }
            Some(&"valid_move") => {
                call_valid_move(&mut board, &vector);
            }
            Some(&"draw") => {
                board.draw_board();
            }
            Some(&"reset") => {
                board.reset();
                board.draw_board();
            }
            Some(&"help") => {
                println!("{}", help_message);
            }
            Some(&"exit") => {
                println!("EXITING");
                return;
            }
            _ => {
                println!("unknown command, try help");
                continue;
            }
        }
    }

    return;
}

fn call_move(bs: &mut BoardState, s: &Vec<&str>) -> bool {
    if s.len() != 5 {
        for i in s {
            println!("{}", i);
        }
    }

    let mut array: [u8; 4] = [0; 4];

    for i in 1..5 {
        if let Some(slice) = s.get(i) {
            if let Ok(val) = slice.parse::<u8>() {
                array[i - 1] = val;
            } else {
                println!("usage: move <origin X> <origin Y> <destination X> <destination Y>");
                return false;
            }
        } else {
            println!("usage: move <origin X> <origin Y> <destination X> <destination Y>");
            return false;
        } // if failed to parse term, return false
    }

    return if bs.move_piece(array[0], array[1], array[2], array[3]) {
        println!("successfully moved");
        true
    } else {
        println!("invalid move");
        false
    };
}

fn call_valid_move(bs: &mut BoardState, s: &Vec<&str>) -> bool {
    if s.len() != 5 {
        for i in s {
            println!("{}", i);
        }
    }

    let mut array: [u8; 4] = [0; 4];

    for i in 1..5 {
        if let Some(slice) = s.get(i) {
            if let Ok(val) = slice.parse::<u8>()
            // parse str to u8
            {
                array[i - 1] = val;
            } else {
                println!(
                    "usage: is_valid_move <origin X> <origin Y> <destination X> <destination Y>"
                );
                return false;
            }
        } else {
            println!("usage: is_valid_move <origin X> <origin Y> <destination X> <destination Y>");
            return false;
        }
    }

    return if bs.is_valid_move(array[0], array[1], array[2], array[3]) {
        println!("valid move");
        true
    } else {
        println!("invalid move");
        false
    };
}
