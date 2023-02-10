const COL_LABELS: [char; 8] = ['a','b','c','d','e','f','g','h'];
const ROW_LABELS: [char; 8] = ['8','7','6','5','4','3','2','1'];

#[derive(Copy,Clone)]
enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    Empty
}

#[derive(Copy,Clone,PartialEq)]
enum Colour {
    White,
    Black,
}

#[derive(Copy,Clone)]
struct Piece {
    kind: PieceKind,
    colour: Colour,
    starting_position: Position,
}

impl Default for Piece {
    fn default() -> Piece {
        Piece {
            kind: PieceKind::Empty,
            starting_position: Position { x: 0, y: 0 },
            colour: Colour::White,
        }
    }
}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        self.starting_position == other.starting_position
    }
}

#[derive(Copy,Clone)]
struct Tile {
    piece: Piece,
    x: usize,
    y: usize,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.piece.colour {
                Colour::White => {
                    match self.piece.kind {
                        PieceKind::Pawn => write!(f,"[♟]"),
                        PieceKind::Knight => write!(f,"[♞]"),
                        PieceKind::Bishop => write!(f,"[♝]"),
                        PieceKind::Rook => write!(f,"[♜]"),
                        PieceKind::Queen => write!(f,"[♛]"),
                        PieceKind::King => write!(f,"[♚]"),
                        PieceKind::Empty => write!(f,"[ ]"),
                    }
                }
                Colour::Black => {
                    match self.piece.kind {
                        PieceKind::Pawn => write!(f,"[♙]"),
                        PieceKind::Knight => write!(f,"[♘]"),
                        PieceKind::Bishop => write!(f,"[♗]"),
                        PieceKind::Rook => write!(f,"[♖]"),
                        PieceKind::Queen => write!(f,"[♕]"),
                        PieceKind::King => write!(f,"[♔]"),
                        PieceKind::Empty => write!(f,"[ ]"),
                    }
                }
        }
    }
}

struct Board {
    tiles: [[Tile; 8];8],
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Position {
    x: usize,
    y: usize,
}


struct Move {
    piece: Piece,
    captured_piece: Piece,
    number: usize,
    start: Position,
    end: Position,
}

struct Game {
    board: Board,
    current_turn: Colour,
    move_history: Vec<Move>,
}

impl Game {
    fn move_piece(&mut self, start: Position, end: Position) {
        self.board.tiles[end.y][end.x].piece = self.board.tiles[start.y][start.x].piece;
        self.board.tiles[start.y][start.x].piece.kind = PieceKind::Empty;
        self.move_history.push(Move { number: self.move_history.len(), piece: self.board.tiles[start.y][start.x].piece, captured_piece: Piece{ kind: PieceKind::Empty, colour: Colour::Black, ..Default::default()},start: Position {x: start.x, y: start.y}, end: Position {x: end.x, y: end.y} })
    }
    
    fn validate_move(&mut self, start: Position, end: Position) -> bool{
        
        if off_board(end.x, end.y) {
            return false
        }

        let piece = self.board.tiles[start.y][start.x].piece;
        let end_tile = self.board.tiles[end.y][end.x];

        match piece.kind {
            PieceKind::Pawn => {
                if piece.colour == Colour::White {
                    if (start.y - 1 == end.y && start.x == end.x)  {
                        true
                    }
                    else if (self.pieces_first_move(&piece) && start.y - 2 == end.y && start.x == end.x) {
                        true
                    }
                    else {
                        false
                    }
                }
                else if piece.colour == Colour::Black  {
                    if (start.y + 1 == end.y && start.x == end.x)  {
                        true
                    }
                    else if (self.pieces_first_move(&piece) && start.y + 2 == end.y && start.x == end.x) {
                        true
                    }
                    else {
                        false
                    }
                }
                else {
                    false
                }
            },
            PieceKind::Knight => { false },
            PieceKind::Bishop => { false },
            PieceKind::Rook => { false },
            PieceKind::Queen => { false },
            PieceKind::King => { false },
            PieceKind::Empty => { false }
        }
    }

    fn pieces_first_move(&self, piece: &Piece) -> bool {
        for i in &self.move_history {
            if i.piece == *piece {
                return false
            }
        }
        return true
    }

    fn draw_board(&self) {
        println!("");
        let mut row_label = 8;
        for row in self.board.tiles {
            print!("{} ", row_label);
            for mut tile in row {
                print!("{}",tile)
            }
            println!("");
            row_label -= 1;
        }
        println!("   a  b  c  d  e  f  g  h")
    }
}

fn main() {
    let mut tiles = [[Tile{piece: Piece{ kind: PieceKind::Empty, colour: Colour::White, ..Default::default() }, x: 0, y: 0}; 8]; 8];
    let mut board = Board{ tiles: tiles };
    let mut move_history: Vec<Move> = Vec::new();
    let mut game = Game{ board: board, current_turn: Colour::White, move_history: move_history };

    setup_board(&mut game.board);
    
    game.draw_board();
    
    let start_pos = parse_coordinate("d2".to_string());
    let end_pos = parse_coordinate("d3".to_string());

    println!();
    println!("> move d2 d3");
    println!();

    game.move_piece(start_pos,end_pos);
    game.draw_board();

}

fn setup_board(board: &mut Board){
    // Set black pawns
    board.tiles[1] = [Tile{piece: Piece{ kind: PieceKind::Pawn, colour: Colour::Black, ..Default::default()}, x: 0, y: 1 }; 8];
    
    // Set black rooks
    board.tiles[0][0].piece = Piece{ kind: PieceKind::Rook, colour: Colour::Black, ..Default::default()};
    board.tiles[0][7].piece = Piece{ kind: PieceKind::Rook, colour: Colour::Black, ..Default::default()};

    // Set black bishops
    board.tiles[0][1].piece = Piece{ kind: PieceKind::Bishop, colour: Colour::Black, ..Default::default()};
    board.tiles[0][6].piece = Piece{ kind: PieceKind::Bishop, colour: Colour::Black, ..Default::default()};

    // Set black knights
    board.tiles[0][2].piece = Piece{ kind: PieceKind::Knight, colour: Colour::Black, ..Default::default()};
    board.tiles[0][5].piece = Piece{ kind: PieceKind::Knight, colour: Colour::Black, ..Default::default()};

    // Set black queen
    board.tiles[0][3].piece = Piece{ kind: PieceKind::Queen, colour: Colour::Black, ..Default::default()};
    board.tiles[0][4].piece = Piece{ kind: PieceKind::King, colour: Colour::Black, ..Default::default()};
    
    //  Set white pawns
    board.tiles[6] = [Tile{piece: Piece{ kind: PieceKind::Pawn, colour: Colour::White, ..Default::default()}, x: 0, y: 6 }; 8];

    // Set black rooks
    board.tiles[7][0].piece = Piece{ kind: PieceKind::Rook, colour: Colour::White, ..Default::default()};
    board.tiles[7][7].piece = Piece{ kind: PieceKind::Rook, colour: Colour::White, ..Default::default()};

    // Set black bishops
    board.tiles[7][1].piece = Piece{ kind: PieceKind::Bishop, colour: Colour::White, ..Default::default()};
    board.tiles[7][6].piece = Piece{ kind: PieceKind::Bishop, colour: Colour::White, ..Default::default()};

    // Set black knights
    board.tiles[7][2].piece = Piece{ kind: PieceKind::Knight, colour: Colour::White, ..Default::default()};
    board.tiles[7][5].piece = Piece{ kind: PieceKind::Knight, colour: Colour::White, ..Default::default()};

    // Set black queen
    board.tiles[7][3].piece = Piece{ kind: PieceKind::Queen, colour: Colour::White, ..Default::default()};
    board.tiles[7][4].piece = Piece{ kind: PieceKind::King, colour: Colour::White, ..Default::default()};

    // Set tile coordinates
    let mut x = 0;
    let mut y = 0;

    for row in board.tiles {
        for col in row {
            board.tiles[x][y].x = x;
            board.tiles[x][y].y = y;
            board.tiles[x][y].piece.starting_position = Position{x: x, y: y};
            x += 1;
        }
        y += 1;
        x = 0;
    }
}

// Converts from chess coordinates to position mapping to the boards 2d array of tiles indexes
// Ex. d3 -> Position(x: 3, y: 5)
// 
//      Standard Chess coordinates      Application tile indexes
//      
//      8 [ ][ ][ ][ ][ ][ ][ ][ ]      0 [ ][ ][ ][ ][ ][ ][ ][ ]
//      7 [ ][ ][ ][ ][ ][ ][ ][ ]      1 [ ][ ][ ][ ][ ][ ][ ][ ]
//      6 [ ][ ][ ][ ][ ][ ][ ][ ]      2 [ ][ ][ ][ ][ ][ ][ ][ ]
//      5 [ ][ ][ ][ ][ ][ ][ ][ ]      3 [ ][ ][ ][ ][ ][ ][ ][ ]
//      4 [ ][ ][ ][ ][ ][ ][ ][ ]      4 [ ][ ][ ][ ][ ][ ][ ][ ]
//      3 [ ][ ][ ][ ][ ][ ][ ][ ]      5 [ ][ ][ ][ ][ ][ ][ ][ ]
//      2 [ ][ ][ ][ ][ ][ ][ ][ ]      6 [ ][ ][ ][ ][ ][ ][ ][ ]
//      1 [ ][ ][ ][ ][ ][ ][ ][ ]      7 [ ][ ][ ][ ][ ][ ][ ][ ]
//         a  b  c  d  e  f  g  h          0  1  2  3  4  5  6  7
// 
fn parse_coordinate(input: String) -> Position{
   let first_coordinate = &input.trim()[0..1];
   let second_coordinate = &input.trim()[1..2];

   let pos_x = COL_LABELS.iter().position(|&x| x == first_coordinate.chars().next().unwrap());
   let x = match pos_x {
    Some(i) => i,
    None => panic!("could not parse coordinates"),
   };
   let pos_y = ROW_LABELS.iter().position(|&x| x == second_coordinate.chars().next().unwrap());
   let y = match pos_y {
    Some(i) => i,
    None => panic!("cannot parse coordinates"),
   };

   Position {x: x, y: y}
}


fn draw_board(board: &Board) {
    println!("");
    let mut row_label = 8;
    for row in board.tiles {
        print!("{} ", row_label);
        for mut tile in row {
            print!("{}",tile)
        }
        println!("");
        row_label -= 1;
    }
    println!("   a  b  c  d  e  f  g  h")
}

fn off_board(x: usize, y: usize) -> bool {
    if x > 7 || x < 0 || y > 7 || y < 0 {
        return true
    }
    return false
}

// ---------------------------------  Unit Tests ----------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // Coordinate parsing ------------
    #[test]
    fn parse_coordinate_returns_position_c4() {
        let position = parse_coordinate("c4".to_string());
        assert_eq!(Position {x: 2, y: 4},position);
    }
    
    #[test]
    fn parse_coordinate_returns_position_d3() {
        let position = parse_coordinate("d3".to_string());
        assert_eq!(Position {x: 3, y: 5},position);
    }

    #[test]
    #[should_panic]
    fn parse_coordinate_panics_when_input_is_invalid() {
        parse_coordinate("z9".to_string());
    }


    // Piece Movement ----------------
    // Pawns
    #[test]
    fn pieces_cannot_move_off_the_board() {
        // White pawn on e2 can not move outside the 8x8 grid
        //                                                                           P
        //                                                      
        //            8 [ ][ ][ ][ ][ ][ ][ ][ ]       8 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            7 [ ][ ][ ][ ][ ][ ][ ][ ]       7 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            6 [ ][ ][ ][ ][ ][ ][ ][ ]       6 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            5 [ ][ ][ ][ ][ ][ ][ ][ ]       5 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            4 [ ][ ][ ][ ][ ][ ][ ][ ]       4 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            3 [ ][ ][ ][ ][ ][ ][ ][ ]       3 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            2 [ ][ ][ ][ ][ ][P][ ][ ]       2 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            1 [ ][ ][ ][ ][ ][ ][ ][ ]       1 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //               a  b  c  d  e  f  g  h           a  b  c  d  e  f  g  h
        //

        let mut tiles = [[Tile{piece: Piece{..Default::default()}, x: 0, y: 0}; 8]; 8];
        let mut board = Board{ tiles: tiles };
        let mut move_history: Vec<Move> = Vec::new();
        let mut game = Game{ board: board, current_turn: Colour::White, move_history: move_history };
        setup_board(&mut game.board);

        let start_pos = parse_coordinate("e2".to_string());
        let end_pos = Position{x: 10, y: 10};

        let valid_move = game.validate_move(start_pos, end_pos);
        assert_eq!(false,valid_move);
    }

    #[test]
    fn pawns_can_move_one_forward() {
        // White pawn on f2 can move one tile forward to f3 on it's first move
        //
        //            8 [ ][ ][ ][ ][ ][ ][ ][ ]       8 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            7 [ ][ ][ ][ ][ ][ ][ ][ ]       7 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            6 [ ][ ][ ][ ][ ][ ][ ][ ]       6 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            5 [ ][ ][ ][ ][ ][ ][ ][ ]       5 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            4 [ ][ ][ ][ ][ ][ ][ ][ ]       4 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            3 [ ][ ][ ][ ][ ][ ][ ][ ]       3 [ ][ ][ ][ ][ ][P][ ][ ] 
        //            2 [ ][ ][ ][ ][ ][P][ ][ ]       2 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            1 [ ][ ][ ][ ][ ][ ][ ][ ]       1 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //               a  b  c  d  e  f  g  h           a  b  c  d  e  f  g  h
        //

        let mut tiles = [[Tile{piece: Piece{ kind: PieceKind::Empty, colour: Colour::White, ..Default::default() }, x: 0, y: 0}; 8]; 8];
        let mut board = Board{ tiles: tiles };
        let mut move_history: Vec<Move> = Vec::new();
        let mut game = Game{ board: board, current_turn: Colour::White, move_history: move_history };
        setup_board(&mut game.board);
        
        let start_pos = parse_coordinate("f2".to_string());
        let end_pos = parse_coordinate("f3".to_string());
        
        let valid_move = game.validate_move(start_pos, end_pos);
        assert_eq!(true,valid_move);
    }

    #[test]
    fn pawns_can_move_two_forward_on_first_move() {
        // White pawn on e2 can move two tiles forward to e4 on it's first move
        //
        //            8 [ ][ ][ ][ ][ ][ ][ ][ ]       8 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            7 [ ][ ][ ][ ][ ][ ][ ][ ]       7 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            6 [ ][ ][ ][ ][ ][ ][ ][ ]       6 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            5 [ ][ ][ ][ ][ ][ ][ ][ ]       5 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            4 [ ][ ][ ][ ][ ][ ][ ][ ]       4 [ ][ ][ ][ ][P][ ][ ][ ] 
        //            3 [ ][ ][ ][ ][ ][ ][ ][ ]       3 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            2 [ ][ ][ ][ ][P][ ][ ][ ]       2 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            1 [ ][ ][ ][ ][ ][ ][ ][ ]       1 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //               a  b  c  d  e  f  g  h           a  b  c  d  e  f  g  h
        //

        let mut tiles = [[Tile{piece: Piece{ kind: PieceKind::Empty, colour: Colour::White, ..Default::default() }, x: 0, y: 0}; 8]; 8];
        let mut board = Board{ tiles: tiles };
        let mut move_history: Vec<Move> = Vec::new();
        let mut game = Game{ board: board, current_turn: Colour::White, move_history: move_history };
        setup_board(&mut game.board);

        let start_pos = parse_coordinate("e2".to_string());
        let end_pos = parse_coordinate("e4".to_string());

        let valid_move = game.validate_move(start_pos, end_pos);
        assert_eq!(true,valid_move);
    }

    #[test]
    fn pawns_cannot_move_two_forward_after_first_move() {
        // After already moving from e2 to e4 the white pawn cannot move 2 tiles up from e4 to e6
        //
        //            8 [ ][ ][ ][ ][ ][ ][ ][ ]       8 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            7 [ ][ ][ ][ ][ ][ ][ ][ ]       7 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            6 [ ][ ][ ][ ][ ][ ][ ][ ]       6 [ ][ ][ ][ ][P][ ][ ][ ] 
        //            5 [ ][ ][ ][ ][ ][ ][ ][ ]       5 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            4 [ ][ ][ ][ ][P][ ][ ][ ]       4 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            3 [ ][ ][ ][ ][ ][ ][ ][ ]       3 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            2 [ ][ ][ ][ ][ ][ ][ ][ ]       2 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            1 [ ][ ][ ][ ][ ][ ][ ][ ]       1 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //               a  b  c  d  e  f  g  h           a  b  c  d  e  f  g  h
        //

        let mut tiles = [[Tile{piece: Piece{ kind: PieceKind::Empty, colour: Colour::White, ..Default::default() }, x: 0, y: 0}; 8]; 8];
        let mut board = Board{ tiles: tiles };
        let mut move_history: Vec<Move> = Vec::new();
        let mut game = Game{ board: board, current_turn: Colour::White, move_history: move_history };
        setup_board(&mut game.board);
        
        let start_pos = parse_coordinate("e2".to_string());
        let end_pos = parse_coordinate("e4".to_string());
        game.move_piece(start_pos, end_pos);

        let start_pos = parse_coordinate("d4".to_string());
        let end_pos = parse_coordinate("e6".to_string());
        
        let valid_move = game.validate_move(start_pos, end_pos);
        assert_eq!(false,valid_move);
    }

    #[test]
    fn pawns_cannot_move_more_than_two_forward() {
        // White pawn on e2 cannot move diagonally to e6
        //
        //            8 [ ][ ][ ][ ][ ][ ][ ][ ]       8 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            7 [ ][ ][ ][ ][ ][ ][ ][ ]       7 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            6 [ ][ ][ ][ ][ ][ ][ ][ ]       6 [ ][ ][ ][ ][P][ ][ ][ ] 
        //            5 [ ][ ][ ][ ][ ][ ][ ][ ]       5 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            4 [ ][ ][ ][ ][ ][ ][ ][ ]       4 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            3 [ ][ ][ ][ ][ ][ ][ ][ ]       3 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            2 [ ][ ][ ][ ][P][ ][ ][ ]       2 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            1 [ ][ ][ ][ ][ ][ ][ ][ ]       1 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //               a  b  c  d  e  f  g  h           a  b  c  d  e  f  g  h
        //

        let mut tiles = [[Tile{piece: Piece{ kind: PieceKind::Empty, colour: Colour::White, ..Default::default() }, x: 0, y: 0}; 8]; 8];
        let mut board = Board{ tiles: tiles };
        let mut move_history: Vec<Move> = Vec::new();
        let mut game = Game{ board: board, current_turn: Colour::Black, move_history: move_history };
        setup_board(&mut game.board);

        let start_pos = parse_coordinate("e2".to_string());
        let end_pos = parse_coordinate("e6".to_string());

        let valid_move = game.validate_move(start_pos, end_pos);
        assert_eq!(false,valid_move);
    }

        #[test]
    fn pawns_cannot_move_diagonally_when_not_capturing() {
        // White pawn on e2 cannot move diagonally to d3
        //
        //            8 [ ][ ][ ][ ][ ][ ][ ][ ]       8 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            7 [ ][ ][ ][ ][ ][ ][ ][ ]       7 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            6 [ ][ ][ ][ ][ ][ ][ ][ ]       6 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            5 [ ][ ][ ][ ][ ][ ][ ][ ]       5 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            4 [ ][ ][ ][ ][ ][ ][ ][ ]       4 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            3 [ ][ ][ ][ ][ ][ ][ ][ ]       3 [ ][ ][ ][P][ ][ ][ ][ ] 
        //            2 [ ][ ][ ][ ][P][ ][ ][ ]       2 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            1 [ ][ ][ ][ ][ ][ ][ ][ ]       1 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //               a  b  c  d  e  f  g  h           a  b  c  d  e  f  g  h
        //

        let mut tiles = [[Tile{piece: Piece{ kind: PieceKind::Empty, colour: Colour::White, ..Default::default() }, x: 0, y: 0}; 8]; 8];
        let mut board = Board{ tiles: tiles };
        let mut move_history: Vec<Move> = Vec::new();
        let mut game = Game{ board: board, current_turn: Colour::Black, move_history: move_history };
        setup_board(&mut game.board);
        
        let start_pos = parse_coordinate("e2".to_string());
        let end_pos = parse_coordinate("d3".to_string());

        let valid_move = game.validate_move(start_pos, end_pos);
        assert_eq!(false,valid_move);
    }

    #[test]
    fn pawns_can_move_one_diagonally_when_capturing() {
        // Black pawn on c7 captures white pawn on d6
        //
        //            8 [ ][ ][ ][ ][ ][ ][ ][ ]       8 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            7 [ ][ ][P][ ][ ][ ][ ][ ]       7 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            6 [ ][ ][ ][P][ ][ ][ ][ ]       6 [ ][ ][ ][P][ ][ ][ ][ ] 
        //            5 [ ][ ][ ][ ][ ][ ][ ][ ]       5 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            4 [ ][ ][ ][ ][ ][ ][ ][ ]       4 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            3 [ ][ ][ ][ ][ ][ ][ ][ ]       3 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            2 [ ][ ][ ][ ][ ][ ][ ][ ]       2 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //            1 [ ][ ][ ][ ][ ][ ][ ][ ]       1 [ ][ ][ ][ ][ ][ ][ ][ ] 
        //               a  b  c  d  e  f  g  h           a  b  c  d  e  f  g  h
        //
        let mut tiles = [[Tile{piece: Piece{ kind: PieceKind::Empty, colour: Colour::White, ..Default::default() }, x: 0, y: 0}; 8]; 8];
        let mut board = Board{ tiles: tiles };
        let mut move_history: Vec<Move> = Vec::new();
        let mut game = Game{ board: board, current_turn: Colour::Black, move_history: move_history };
        setup_board(&mut game.board);

        // Force move white pawn to e6
        let start_pos = parse_coordinate("e2".to_string());
        let end_pos = parse_coordinate("e6".to_string());
        game.move_piece(start_pos, end_pos);

        let start_pos = parse_coordinate("c7".to_string());
        let end_pos = parse_coordinate("d6".to_string());
        let valid_move = game.validate_move(start_pos, end_pos);
        
        assert_eq!(true,valid_move);
    }
}
