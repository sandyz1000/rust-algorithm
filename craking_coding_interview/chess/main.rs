#![allow(unused)]

#[derive(Debug, PartialEq, Eq)]
enum PieceType {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
}

impl PieceType {
    fn create(&self, position: ChessPosition, color: Color) -> Piece {
        match self {
            PieceType::King => Piece::King(King::new(position, color)),
            PieceType::Queen => Piece::Queen(Queen::new(position, color)),
            PieceType::Knight => Piece::Knight(Knight::new(position, color)),
            PieceType::Rook => Piece::Rook(Rook::new(position, color)),
            PieceType::Bishop => Piece::Bishop(Bishop::new(position, color)),
            PieceType::Pawn => Piece::Pawn(Pawn::new(position, color)),
        }
    }
}

const CHESS_BOARD_SIZE: usize = 8;

const INITIAL_PIECE_SET_SINGLE: [(PieceType, i32, i32); 16] = [
    (PieceType::Rook, 0, 0),
    (PieceType::Knight, 1, 0),
    (PieceType::Bishop, 2, 0),
    (PieceType::Queen, 3, 0),
    (PieceType::King, 4, 0),
    (PieceType::Bishop, 5, 0),
    (PieceType::Knight, 6, 0),
    (PieceType::Rook, 7, 0),
    (PieceType::Pawn, 0, 1),
    (PieceType::Pawn, 1, 1),
    (PieceType::Pawn, 2, 1),
    (PieceType::Pawn, 3, 1),
    (PieceType::Pawn, 4, 1),
    (PieceType::Pawn, 5, 1),
    (PieceType::Pawn, 6, 1),
    (PieceType::Pawn, 7, 1),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Color {
    BLACK,
    WHITE
}

trait PieceMaker {
    
    fn move_to(&mut self, target_position: ChessPosition);

    fn get_threatened_positions(&self, board: &ChessBoard) -> Vec<ChessPosition>;

    fn get_movable_positions(&self, board: &ChessBoard) -> Vec<ChessPosition>;

}

trait PieceSymbol {
    fn __symbol(&self, color: &Color) -> String {
        let black_color_prefix = "\x1B[31;1m";
        let white_color_prefix = "\x1B[34;1m";
        let color_suffix = "\x1B[0m";
        let mut retval = self.symbol_impl();
        if *color == Color::BLACK {
            retval = format!("{}{}{}", black_color_prefix, retval, color_suffix);
        } else {
            retval = format!("{}{}{}", white_color_prefix, retval, color_suffix);
        }
        retval
    }

    fn symbol(&self) -> String;

    fn symbol_impl(&self) -> String;
}

#[derive(Debug, Clone)]
enum Piece {
    King(King),
    Queen(Queen),
    Knight(Knight),
    Rook(Rook),
    Bishop(Bishop),
    Pawn(Pawn)
}

impl PieceMaker for Piece {
    fn get_threatened_positions(&self, board: &ChessBoard) -> Vec<ChessPosition> {
        match self {
            Piece::Bishop(bishop) => bishop.get_threatened_positions(board),
            Piece::King(king) => king.get_threatened_positions(board),
            Piece::Knight(knight) => knight.get_threatened_positions(board),
            Piece::Pawn(pawn) => pawn.get_threatened_positions(board),
            Piece::Rook(rook) => rook.get_threatened_positions(board),
            Piece::Queen(queen) => queen.get_threatened_positions(board),
            _ => unreachable!()
        }
    }

    fn get_movable_positions(&self, board: &ChessBoard) -> Vec<ChessPosition> {
        // Use macros to generate all possible moves
        match self {
            Piece::Bishop(bishop) => bishop.get_movable_positions(board),
            Piece::King(king) => king.get_movable_positions(board),
            Piece::Knight(knight) => knight.get_movable_positions(board),
            Piece::Pawn(pawn) => pawn.get_movable_positions(board),
            Piece::Rook(rook) => rook.get_movable_positions(board),
            Piece::Queen(queen) => queen.get_movable_positions(board),
            _ => unreachable!()
        }
    }

    fn move_to(&mut self, target_position: ChessPosition) {
        match self {
            Piece::King(king) => king.move_to(target_position),
            Piece::Queen(queen) => queen.move_to(target_position),
            Piece::Knight(knight) => knight.move_to(target_position),
            Piece::Rook(rook) => rook.move_to(target_position),
            Piece::Bishop(bishop) => bishop.move_to(target_position),
            Piece::Pawn(pawn) => pawn.move_to(target_position),
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone)]
struct King {
    position: ChessPosition,
    color: Color,
    board_handle: Option<ChessBoard>,
}

impl King {
    const SPOT_INCREMENTS: [(i32, i32); 8] = [
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
    ];

    fn new(position: ChessPosition, color: Color) -> Self {
        King {
            position,
            color,
            board_handle: None,
        }
    }

    fn set_board_handle(&mut self, board: ChessBoard) {
        self.board_handle = Some(board);
        self.board_handle.as_ref().unwrap().register_king_position(self.position, self.color);
    }
}

impl PieceMaker for King {
    fn move_to(&mut self, target_position: ChessPosition) {
        self.position = target_position;
        self.board_handle.as_ref().unwrap().register_king_position(target_position, self.color);
    }

    fn get_threatened_positions(&self, board: &ChessBoard) -> Vec<ChessPosition> {
        let mut positions = Vec::new();
        for increment in King::SPOT_INCREMENTS.iter() {
            if let Some(position) = board.spot_search_threat(
                self.position, 
                &self.color, 
                increment.0, 
                increment.1,
                false, false
            ) {
                positions.push(position);
            }
        }
        positions
    }

    fn get_movable_positions(&self, board: &ChessBoard) -> Vec<ChessPosition> {
        self.get_threatened_positions(board)
    }

}

impl PieceSymbol for King {
    fn symbol(&self) -> String {
        self.__symbol(&self.color)
    }

    fn symbol_impl(&self) -> String {
        "KI".to_owned()
    }
}

#[derive(Debug, Clone)]
struct Pawn {
    position: ChessPosition,
    color: Color,
    moved: bool,
}

impl Pawn {
    const SPOT_INCREMENTS_MOVE: Vec<(i32, i32)> = vec![(0, 1)];
    const SPOT_INCREMENTS_MOVE_FIRST: Vec<(i32, i32)> = vec![(0, 1), (0, 2)];
    const SPOT_INCREMENTS_TAKE: Vec<(i32, i32)> = vec![(-1, 1), (1, 1)];

    fn new(position: ChessPosition, color: Color) -> Self {
        Pawn {
            position,
            color,
            moved: false,
        }
    }

}

impl PieceMaker for Pawn {
    
    fn get_threatened_positions(&self, board: &ChessBoard) -> Vec<ChessPosition> {
        let mut positions = Vec::new();
        let increments = Pawn::SPOT_INCREMENTS_TAKE;
        for increment in increments.iter() {
            let dy = if self.color == Color::WHITE {
                increment.1
            } else {
                -increment.1
            };
            if let Some(position) = board.spot_search_threat(
                self.position, 
                &self.color, 
                increment.0, 
                dy,
                false, false
            ) {
                positions.push(position);
            }
        }
        positions
    }

    fn get_movable_positions(&self, board: &ChessBoard) -> Vec<ChessPosition> {
        let mut positions = Vec::new();
        let increments = if self.moved {
            Pawn::SPOT_INCREMENTS_MOVE
        } else {
            Pawn::SPOT_INCREMENTS_MOVE_FIRST
        };
        for increment in increments.iter() {
            let dy = if self.color == Color::WHITE {
                increment.1
            } else {
                -increment.1
            };
            if let Some(position) = board.spot_search_threat(
                self.position, 
                &self.color, 
                increment.0, 
                dy, 
                true, false ) {
                positions.push(position);
            }
        }

        let increments = Pawn::SPOT_INCREMENTS_TAKE;
        for increment in increments.iter() {
            let dy = if self.color == Color::WHITE {
                increment.1
            } else {
                -increment.1
            };
            if let Some(position) = board.spot_search_threat(
                self.position, 
                &self.color, 
                increment.0, 
                dy, false, true) {
                positions.push(position);
            }
        }
        positions
    }

    fn move_to(&mut self, target_position: ChessPosition) {
        self.moved = true;
        self.position = target_position;
    }

}

impl PieceSymbol for Pawn {
    fn symbol(&self) -> String {
        self.__symbol(&self.color)
    }
    
    fn symbol_impl(&self) -> String {
        "PA".to_owned()
    }
}

#[derive(Debug, Clone)]
struct Bishop {
    position: ChessPosition,
    color: Color,
}

impl Bishop {
    const BEAM_INCREMENTS: [(i32, i32); 4] = [
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    fn new(position: ChessPosition, color: Color) -> Self {
        Bishop {
            position,
            color,
        }
    }
}


impl PieceMaker for Bishop {
    fn move_to(&mut self, target_position: ChessPosition) {
        self.position = target_position;
    }

    fn get_threatened_positions(&self, board: &ChessBoard) -> Vec<ChessPosition> {
        let mut positions = Vec::new();
        for increment in Bishop::BEAM_INCREMENTS.iter() {
            positions.extend(board.beam_search_threat(self.position, &self.color, increment.0, increment.1));
        }
        positions
    }

    fn get_movable_positions(&self, board: &ChessBoard) -> Vec<ChessPosition> {
        self.get_threatened_positions(board)
    }

}

impl PieceSymbol for Bishop {
    fn symbol(&self) -> String {
        self.__symbol(&self.color)
    }
    
    fn symbol_impl(&self) -> String {
        "BI".to_owned()
    }
}

#[derive(Debug, Clone)]
struct Knight {
    position: ChessPosition,
    color: Color,
}

impl Knight {
    const SPOT_INCREMENTS: [(i32, i32); 8] = [
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
    ];

    fn new(position: ChessPosition, color: Color) -> Self {
        Knight {
            position,
            color,
        }
    }
}

impl PieceMaker for Knight {
    fn move_to(&mut self, target_position: ChessPosition) {
        self.position = target_position;
    }

    fn get_threatened_positions(&self, board: &ChessBoard) -> Vec<ChessPosition> {
        let mut positions = Vec::new();
        for increment in Knight::SPOT_INCREMENTS.iter() {
            if let Some(position) = board.spot_search_threat(
                self.position, 
                &self.color, 
                increment.0, increment.1, 
                false, false) {
                positions.push(position);
            }
        }
        positions
    }

    fn get_movable_positions(&self, board: &ChessBoard) -> Vec<ChessPosition> {
        self.get_threatened_positions(board)
    }

}

impl PieceSymbol for Knight {
    fn symbol(&self) -> String {
        self.__symbol(&self.color)
    }
    
    fn symbol_impl(&self) -> String {
        "KN".to_owned()
    }
}

#[derive(Debug, Clone)]
struct Rook {
    position: ChessPosition,
    color: Color,
}

impl Rook {
    const BEAM_INCREMENTS: [(i32, i32); 4] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
    ];

    fn new(position: ChessPosition, color: Color) -> Self {
        Rook {
            position,
            color,
        }
    }
}


impl PieceMaker for Rook {
    fn move_to(&mut self, target_position: ChessPosition) {
        self.position = target_position;
    }

    
    fn get_threatened_positions(&self, board: &ChessBoard) -> Vec<ChessPosition> {
        let mut positions = Vec::new();
        for increment in Rook::BEAM_INCREMENTS.iter() {
            positions.extend(board.beam_search_threat(self.position, &self.color, increment.0, increment.1));
        }
        positions
    }

    fn get_movable_positions(&self, board: &ChessBoard) -> Vec<ChessPosition> {
        self.get_threatened_positions(board)
    }
    
}

impl PieceSymbol for Rook {
    fn symbol(&self) -> String {
        self.__symbol(&self.color)
    }
    fn symbol_impl(&self) -> String {
        "RO".to_owned()
    }
}

#[derive(Debug, Clone)]
struct Queen {
    position: ChessPosition,
    color: Color,
}

impl Queen {
    const BEAM_INCREMENTS: [(i32, i32); 8] = [
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
    ];

    fn new(position: ChessPosition, color: Color) -> Self {
        Queen {
            position,
            color,
        }
    }
}

impl PieceMaker for Queen {
    fn move_to(&mut self, target_position: ChessPosition) {
        self.position = target_position;
    }
    
    fn get_threatened_positions(&self, board: &ChessBoard) -> Vec<ChessPosition> {
        let mut positions = Vec::new();
        for increment in Queen::BEAM_INCREMENTS.iter() {
            positions.extend(board.beam_search_threat(self.position, &self.color, increment.0, increment.1));
        }
        positions
    }
    
    fn get_movable_positions(&self, board: &ChessBoard) -> Vec<ChessPosition> {
        self.get_threatened_positions(board)
    }
    
}

impl PieceSymbol for Queen {
    fn symbol(&self) -> String {
        self.__symbol(&self.color)
    }
    fn symbol_impl(&self) -> String {
        "QU".to_owned()
    }
}

struct Player;

impl Player {
    fn play_chess(&self) {
        let render = Some(ConsoleRender);
        let mut game = ChessGame::new(render.as_ref());
        game.run();
    }
}


#[derive(Debug, Clone)]
struct ChessPosition {
    x_coord: i32,
    y_coord: i32,
}

impl ChessPosition {
    fn new(x_coord: i32, y_coord: i32) -> Self {
        ChessPosition {
            x_coord,
            y_coord,
        }
    }

    fn from_string(string: &str) -> Option<Self> {
        if string.len() != 2 {
            return None;
        }
        let chars: Vec<char> = string.chars().collect();
        let x_coord = chars[0] as i32 - 'a' as i32;
        let y_coord = chars[1] as i32 - '1' as i32;
        Some(ChessPosition::new(x_coord, y_coord))
    }
}

impl std::fmt::Display for ChessPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let x_char = ('a' as u8 + self.x_coord as u8) as char;
        write!(f, "{}{}", x_char, self.y_coord + 1)
    }
}

impl PartialEq for ChessPosition {
    fn eq(&self, other: &Self) -> bool {
        self.x_coord == other.x_coord && self.y_coord == other.y_coord
    }
}

struct MoveCommand {
    src: ChessPosition,
    dst: ChessPosition,
}

impl MoveCommand {
    fn new(src: ChessPosition, dst: ChessPosition) -> Self {
        MoveCommand {
            src,
            dst,
        }
    }

    fn from_string(string: &str) -> Option<Self> {
        let tokens: Vec<&str> = string.split_whitespace().collect();
        if tokens.len() != 2 {
            return None;
        }
        let src = ChessPosition::from_string(tokens[0]);
        let dst = ChessPosition::from_string(tokens[1]);
        match (src, dst) {
            (Some(src), Some(dst)) => Some(MoveCommand::new(src, dst)),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct ChessGameState {
    pieces: Vec<Piece>,
    board_size: i32,
}

struct ChessGame<'a, R: InputRender> {
    finished: bool,
    board: ChessBoard,
    renderer: Option<&'a R>,
    status: &'static str,
}

impl<'a, R: InputRender> ChessGame<'a, R> {
    const STATUS_WHITE_MOVE: &'static str = "white_move";
    const STATUS_BLACK_MOVE: &'static str = "black_move";
    const STATUS_WHITE_VICTORY: &'static str = "white_victory";
    const STATUS_BLACK_VICTORY: &'static str = "black_victory";

    fn new(renderer: Option<&'a R>) -> Self {
        ChessGame {
            finished: false,
            board: ChessBoard::new(8),
            renderer,
            status: ChessGame::STATUS_WHITE_MOVE,
        }
    }

    fn run(&mut self) {
        if let Some(renderer) = self.renderer {
            renderer.render(self.get_game_state());
        }
        while !self.finished {
            let command = self.parse_command();
            if command.is_none() && self.renderer.is_some() {
                if let Some(renderer) = self.renderer {
                    renderer.print_line("Invalid command, please re-enter.");
                }
                continue;
            }
            if !self.try_move(command.unwrap()) {
                if let Some(renderer) = self.renderer {
                    renderer.print_line("Invalid command, please re-enter.");
                }
                continue;
            }

            self.board.execute_move(command.as_ref().unwrap());
            self.status = match self.status {
                ChessGame::STATUS_WHITE_MOVE => ChessGame::STATUS_BLACK_MOVE,
                ChessGame::STATUS_BLACK_MOVE => ChessGame::STATUS_WHITE_MOVE,
                _ => self.status,
            };
            if let Some(renderer) = self.renderer {
                renderer.render(self.get_game_state());
            }
        }
    }

    fn try_move(&self, command: MoveCommand) -> bool {
        let mut board_copy = self.board.clone();
        let src_piece = board_copy.get_piece(command.src);
        if src_piece.is_none() {
            return false;
        }
        let src_piece = src_piece.unwrap();
        if (self.status == ChessGame::STATUS_WHITE_MOVE && src_piece.color == Color::BLACK)
            || (self.status == ChessGame::STATUS_BLACK_MOVE && src_piece.color == Color::WHITE)
        {
            return false;
        }
        if !src_piece.get_movable_positions(&board_copy).contains(&command.dst)
            && !src_piece.get_threatened_positions(&board_copy).contains(&command.dst)
        {
            return false;
        }
        board_copy.execute_move(&command);
        for piece in &board_copy.pieces {
            if self.status == ChessGame::STATUS_WHITE_MOVE
                && board_copy.white_king_position()
                    .map_or(false, |pos| piece.get_threatened_positions(&board_copy).contains(&pos))
            {
                return false;
            } else if self.status == ChessGame::STATUS_BLACK_MOVE
                && board_copy.black_king_position()
                    .map_or(false, |pos| piece.get_threatened_positions(&board_copy).contains(&pos))
            {
                return false;
            }
        }
        true
    }

    fn parse_command(&self) -> Option<MoveCommand> {
        let mut input = String::new();
        if let Ok(_) = std::io::stdin().read_line(&mut input) {
            MoveCommand::from_string(&input)
        } else {
            None
        }
    }

    fn get_game_state(&self) -> ChessGameState {
        ChessGameState {
            pieces: self.board.pieces.clone(),
            board_size: self.board.size as i32,
        }
    }
}

trait InputRender {
    fn render(&self, game_state: ChessGameState);

    fn print_line(&self, string: &str);
}

struct ConsoleRender;

impl InputRender for ConsoleRender {
    fn render(&self, game: ChessGameState) {
        for i in (0..game.board_size).rev() {
            self._draw_board_line(i, &game.pieces, game.board_size);
        }
        self._draw_bottom_line(game.board_size);
    }

    fn print_line(&self, string: &str) {
        println!("{}", string);
    }
}

impl ConsoleRender {
    fn _draw_time_line(&self, countdown_white: i32, countdown_black: i32) {
        println!("Time remaining: {}s W / B {}s", countdown_white, countdown_black);
    }

    fn _draw_board_line(&self, line_number: i32, pieces: &Vec<Piece>, board_size: i32) {
        let empty_square = " ";
        let white_square_prefix = "\u{001b}[47m";
        let black_square_prefix = "\u{001b}[40m";
        let reset_suffix = "\u{001b}[0m";
        let black_first_offset = line_number % 2;

        let legend = format!("{:<2} ", line_number + 1);
        print!("{}", legend);
        for i in 0..board_size {
            let is_black = (i + black_first_offset) % 2;
            let prefix = if is_black == 1 { black_square_prefix } else { white_square_prefix };
            let contents = empty_square.to_owned();
            let curr_position = ChessPosition::new(i, line_number);
            for piece in pieces {
                if curr_position == piece.position {
                    contents = piece.symbol();
                }
            }
            let square_str = format!("{}{}{}", prefix, contents, reset_suffix);
            print!("{}", square_str);
        }
        println!();
    }

    fn _draw_bottom_line(&self, board_size: i32) {
        let vertical_legend_offset = 3;
        let mut line = " ".repeat(vertical_legend_offset as usize);
        for i in 0..board_size {
            line.push((b'a' + i as u8) as char);
        }
        println!("{}", line);
    }
}

#[derive(Debug, Clone)]
struct ChessBoard {
    size: usize,
    pieces: Vec<Piece>,
    white_king_position: Option<ChessPosition>,
    black_king_position: Option<ChessPosition>,
}

impl ChessBoard {
    fn new(size: usize) -> Self {
        let mut board = ChessBoard {
            size,
            pieces: vec![],
            white_king_position: None,
            black_king_position: None,
        };
        board.initialize_pieces(&INITIAL_PIECE_SET_SINGLE);
        board
    }

    fn initialize_pieces(&mut self, pieces_setup: &[(PieceType, i32, i32)]) {
        for &(piece_type, x, y) in pieces_setup {
            let piece_white = piece_type.create( 
                ChessPosition::new(x, y), 
                Color::WHITE
            );
            if piece_type == PieceType::King {
                // TODO: fix this
                // piece_white.downcast_ref::<King>().unwrap().set_board_handle(self);
            }
            self.pieces.push(piece_white);

            let piece_black = piece_type.create(
                ChessPosition::new(self.size as i32 - x - 1, self.size as i32 - y - 1),
                Color::BLACK,
            );
            if piece_type == PieceType::King {
                // TODO: fix this
                // piece_black.set_board_handle(self);
            }
            self.pieces.push(piece_black);
        }
    }

    fn get_piece(&self, position: ChessPosition) -> Option<Piece> {
        self.pieces.into_iter().find(|piece| piece.position == position)
    }

    fn beam_search_threat(
        &self,
        start_position: ChessPosition,
        own_color: &Color,
        increment_x: i32,
        increment_y: i32,
    ) -> Vec<ChessPosition> {
        let mut threatened_positions = vec![];
        let (mut curr_x, mut curr_y) = (start_position.x_coord, start_position.y_coord);
        curr_x += increment_x;
        curr_y += increment_y;
        while curr_x >= 0 && curr_y >= 0 && curr_x < self.size as i32 && curr_y < self.size as i32 {
            let curr_position = ChessPosition::new(curr_x, curr_y);
            if let Some(curr_piece) = self.get_piece(curr_position) {
                if curr_piece.color != own_color {
                    threatened_positions.push(curr_position);
                }
                break;
            }
            threatened_positions.push(curr_position);
            curr_x += increment_x;
            curr_y += increment_y;
        }
        threatened_positions
    }

    fn spot_search_threat(
        &self,
        start_position: ChessPosition,
        own_color: &Color,
        increment_x: i32,
        increment_y: i32,
        threat_only: bool,
        free_only: bool,
    ) -> Option<ChessPosition> {
        let curr_x = start_position.x_coord + increment_x;
        let curr_y = start_position.y_coord + increment_y;

        if curr_x >= self.size as i32 || curr_y >= self.size as i32 || curr_x < 0 || curr_y < 0 {
            return None;
        }

        let curr_position = ChessPosition::new(curr_x, curr_y);
        if let Some(curr_piece) = self.get_piece(curr_position) {
            if free_only {
                return None;
            }
            return if curr_piece.color != own_color {
                Some(curr_position)
            } else {
                None
            };
        }
        if threat_only {
            None
        } else {
            Some(curr_position)
        }
    }

    fn pieces(&self) -> Vec<Piece> {
        self.pieces.clone()
    }

    fn size(&self) -> usize {
        self.size
    }

    fn white_king_position(&self) -> Option<ChessPosition> {
        self.white_king_position
    }

    fn black_king_position(&self) -> Option<ChessPosition> {
        unimplemented!()
    }

    fn execute_move(&mut self, command: &MoveCommand) {
        let source_piece = self.get_piece(command.src).expect("Invalid source position");
        if let Some(idx) = self.pieces.iter().position(|piece| piece.position == command.dst) {
            self.pieces.remove(idx);
        }
        source_piece.move_to(command.dst);
    }

    fn register_king_position(&mut self, position: ChessPosition, color: Color) {
        match color {
            Color::WHITE => self.white_king_position = Some(position),
            Color::BLACK => self.black_king_position = Some(position),
            _ => panic!("Unknown color of the king piece"),
        }
    }
}


fn main() {
    let player = Player;
    player.play_chess();
}