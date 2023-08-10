use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

const DIAGONALS: [[isize; 2]; 4] = [[1, 1], [1, -1], [-1, 1], [-1, -1]];
const SIDES: [[isize; 2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];

trait InfoArray {
    fn unwrap_array(&self) -> Result<[Info; 2], &'static str>;
}

impl InfoArray for [Result<Info, &'static str>; 2] {
    fn unwrap_array(&self) -> Result<[Info; 2], &'static str> {
        match &self {
            [Ok(a), Ok(b)] => Ok([*a, *b]),
            [Err(err), _] => Err(err),
            [_, Err(err)] => Err(err),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum BoardSpot {
    Piece(Piece),
    BLANK,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PieceType {
    PAWN,
    ROOK,
    KNIGHT,
    BISHOP,
    QUEEN,
    KING,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Color {
    WHITE,
    BLACK,
}

impl Color {
    fn reverse(&self) -> Self {
        match &self {
            Color::WHITE => Color::BLACK,
            Color::BLACK => Color::WHITE,
        }
    }
}

enum Columns {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Piece {
    piece_type: PieceType,
    color: Color,
}

type Board = [[BoardSpot; 8]; 8];

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.column == other.column
    }
}

impl BoardSpot {
    fn to_board_representation(&self) -> char {
        match self {
            BoardSpot::Piece(Piece {
                piece_type: PieceType::PAWN,
                color: Color::WHITE,
            }) => '♟',
            BoardSpot::Piece(Piece {
                piece_type: PieceType::ROOK,
                color: Color::WHITE,
            }) => '♜',
            BoardSpot::Piece(Piece {
                piece_type: PieceType::KNIGHT,
                color: Color::WHITE,
            }) => '♞',
            BoardSpot::Piece(Piece {
                piece_type: PieceType::BISHOP,
                color: Color::WHITE,
            }) => '♝',
            BoardSpot::Piece(Piece {
                piece_type: PieceType::QUEEN,
                color: Color::WHITE,
            }) => '♛',
            BoardSpot::Piece(Piece {
                piece_type: PieceType::KING,
                color: Color::WHITE,
            }) => '♚',

            BoardSpot::Piece(Piece {
                piece_type: PieceType::PAWN,
                color: Color::BLACK,
            }) => '♙',
            BoardSpot::Piece(Piece {
                piece_type: PieceType::ROOK,
                color: Color::BLACK,
            }) => '♖',
            BoardSpot::Piece(Piece {
                piece_type: PieceType::KNIGHT,
                color: Color::BLACK,
            }) => '♘',
            BoardSpot::Piece(Piece {
                piece_type: PieceType::BISHOP,
                color: Color::BLACK,
            }) => '♗',
            BoardSpot::Piece(Piece {
                piece_type: PieceType::QUEEN,
                color: Color::BLACK,
            }) => '♕',
            BoardSpot::Piece(Piece {
                piece_type: PieceType::KING,
                color: Color::BLACK,
            }) => '♔',

            BoardSpot::BLANK => '.',
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    line: usize,
    column: usize,
}

#[derive(Debug, Clone, Copy)]
struct Info {
    position: Position,
    piece: BoardSpot,
}

#[derive(PartialEq, Eq)]
enum CheckPiece {
    SAMECOLOR,
    DIFFERENTCOLOR,
    NOPIECE,
}

fn diagonals_movement_check(
    board: &Board,
    piece: &Piece,
    piece_position: &Position,
) -> Vec<Position> {
    let mut possible_positions: Vec<Position> = vec![];
    for movement in DIAGONALS {
        let mut new_position = piece_position.clone();
        while new_position.line < 8 && new_position.column < 8 {
            new_position.line = (new_position.line as isize + movement[0]) as usize;
            new_position.column = (new_position.column as isize + movement[1]) as usize;

            if new_position.line >= 8 || new_position.column >= 8 {
                break;
            }

            let check_piece =
                check_piece_in_possible_movement_spot(board, &piece.color, &new_position);
            if check_piece == CheckPiece::SAMECOLOR {
                break;
            } else {
                possible_positions.push(new_position.clone());
                if check_piece == CheckPiece::DIFFERENTCOLOR {
                    break;
                }
            }
        }
    }
    possible_positions
}

fn sides_movement_check(board: &Board, piece: &Piece, piece_position: &Position) -> Vec<Position> {
    let mut possible_positions: Vec<Position> = vec![];
    for movement in SIDES {
        let mut new_position = piece_position.clone();
        while new_position.line < 8 && new_position.column < 8 {
            new_position.line = (new_position.line as isize + movement[0]) as usize;
            new_position.column = (new_position.column as isize + movement[1]) as usize;
            if new_position.line >= 8 || new_position.column >= 8 {
                break;
            }

            let check_piece =
                check_piece_in_possible_movement_spot(board, &piece.color, &new_position);

            if check_piece == CheckPiece::SAMECOLOR {
                break;
            } else {
                possible_positions.push(new_position.clone());
                if check_piece == CheckPiece::DIFFERENTCOLOR {
                    break;
                }
            }
        }
    }
    possible_positions
}

fn letter_to_column(letter: char) -> Option<Columns> {
    match letter {
        'a' => Some(Columns::A),
        'b' => Some(Columns::B),
        'c' => Some(Columns::C),
        'd' => Some(Columns::D),
        'e' => Some(Columns::E),
        'f' => Some(Columns::F),
        'g' => Some(Columns::G),
        'h' => Some(Columns::H),
        _ => None,
    }
}

fn translate_notation(board: &Board, notation_position: &str) -> Result<Info, &'static str> {
    let vectorized_position: Vec<char> = notation_position.chars().collect();

    let column = letter_to_column(vectorized_position[0]).ok_or("Invalid line")?;

    let line = vectorized_position[1].to_digit(10).ok_or("Invalid line")?;

    if line > 8 {
        return Err("Invalid line");
    }

    let piece_position = Position {
        column: (column as u32) as usize,
        line: (line as u32 - 1) as usize,
    };

    Ok(Info {
        position: piece_position,
        piece: board[piece_position.line][piece_position.column],
    })
}

fn check_piece_in_possible_movement_spot(
    board: &Board,
    piece_color: &Color,
    position_to_check: &Position,
) -> CheckPiece {
    let piece_on_possible_position = board[position_to_check.line][position_to_check.column];
    match piece_on_possible_position {
        BoardSpot::BLANK => CheckPiece::NOPIECE,
        BoardSpot::Piece(piece) => {
            if piece.color == *piece_color {
                CheckPiece::SAMECOLOR
            } else {
                CheckPiece::DIFFERENTCOLOR
            }
        }
    }
}
fn generate_all_pieces_possible_movements(board: &Board, pieces_infos: Vec<Info>) -> Vec<Position> {
    let mut all_possible_movements: Vec<Position> = vec![];

    for info in pieces_infos {
        match info.piece {
            BoardSpot::Piece(piece) => all_possible_movements.extend(generate_possible_movements(
                board,
                &piece,
                &info.position,
            )),
            _ => continue,
        }
    }

    all_possible_movements
}

fn generate_possible_movements(
    board: &Board,
    piece: &Piece,
    piece_position: &Position,
) -> Vec<Position> {
    let mut possible_positions: Vec<Position> = vec![];

    match piece.piece_type {
        PieceType::PAWN => {
            let pawn_movements = HashMap::from([
                ("forward", [[1, 0], [2, 0]]),
                ("diagonal", [[1, 1], [1, -1]]),
            ]);
            for (key, value) in pawn_movements.iter() {
                for movement in value {
                    if movement[0] == 2 {
                        if (piece_position.line != 1 && piece.color == Color::WHITE)
                            || (piece_position.line != 6 && piece.color == Color::BLACK)
                        {
                            continue;
                        }
                    }
                    let line = piece_position.line as isize
                        + (if piece.color == Color::BLACK {
                            movement[0] * -1
                        } else {
                            movement[0]
                        });
                    let new_position = Position {
                        line: line as usize,
                        column: (piece_position.column as isize + movement[1]) as usize,
                    };

                    if new_position.line >= 8 || new_position.column >= 8 {
                        continue;
                    }
                    if key == &"forward" {
                        if check_piece_in_possible_movement_spot(board, &piece.color, &new_position)
                            == CheckPiece::NOPIECE
                        {
                            possible_positions.push(new_position);
                        }
                    } else {
                        if check_piece_in_possible_movement_spot(board, &piece.color, &new_position)
                            == CheckPiece::DIFFERENTCOLOR
                        {
                            possible_positions.push(new_position);
                        }
                    }
                }
            }
        }

        PieceType::BISHOP => {
            possible_positions = diagonals_movement_check(board, piece, piece_position);
        }

        PieceType::KNIGHT => {
            let knight_movements: Vec<[isize; 2]> = vec![
                [2, 1],
                [2, -1],
                [1, 2],
                [1, -2],
                [-2, 1],
                [-2, -1],
                [-1, 2],
                [-1, -2],
            ];
            for movemement in knight_movements {
                let new_position = Position {
                    line: (piece_position.line as isize + movemement[0]) as usize,
                    column: (piece_position.column as isize + movemement[1]) as usize,
                };

                if new_position.line >= 8 || new_position.column >= 8 {
                    continue;
                }

                let check_piece =
                    check_piece_in_possible_movement_spot(board, &piece.color, &new_position);

                if check_piece != CheckPiece::SAMECOLOR {
                    possible_positions.push(new_position);
                }
            }
        }

        PieceType::ROOK => {
            possible_positions = sides_movement_check(board, piece, piece_position);
        }

        PieceType::QUEEN => {
            let diagonals_moves = diagonals_movement_check(board, piece, &piece_position);
            possible_positions.extend(diagonals_moves);

            let sides_moves = sides_movement_check(board, piece, &piece_position);

            possible_positions.extend(sides_moves);
        }

        PieceType::KING => {
            let mut king_movements: Vec<[isize; 2]> = vec![];
            king_movements.extend(DIAGONALS);
            king_movements.extend(SIDES);
            for movement in king_movements {
                let new_position = Position {
                    line: (piece_position.line as isize + movement[0]) as usize,
                    column: (piece_position.column as isize + movement[1]) as usize,
                };

                if new_position.line >= 8 || new_position.column >= 8 {
                    continue;
                }

                let check_piece =
                    check_piece_in_possible_movement_spot(board, &piece.color, &new_position);

                if check_piece != CheckPiece::SAMECOLOR {
                    possible_positions.push(new_position);
                }
            }
        }
    }
    possible_positions
}

fn check_move(
    board: &Board,
    piece: Piece,
    starting_position: &Position,
    destination_position: &Position,
) -> bool {
    let possible_movements = generate_possible_movements(board, &piece, &starting_position);
    println!("POSSIBLE MOVEMENTS");
    println!("{:?}", possible_movements);
    if possible_movements.contains(&destination_position) {
        return true;
    }
    false
}

fn move_piece(
    board: &Board,
    piece: Piece,
    starting_position: &Position,
    destination_position: &Position,
) -> (Board, bool) {
    let mut new_board = *board;
    let is_legal_move = check_move(board, piece, starting_position, &destination_position);
    if is_legal_move {
        new_board[destination_position.line][destination_position.column] = BoardSpot::Piece(piece);
        new_board[starting_position.line][starting_position.column] = BoardSpot::BLANK;
        (new_board, true)
    } else {
        println!("MOVIMENTO ILEGAL");
        (new_board, false)
    }
}

fn find_king(board: &Board, color: &Color) -> Result<Position, &'static str> {
    for (line_index, line) in board.iter().enumerate() {
        for (column_index, column) in line.iter().enumerate() {
            match column {
                BoardSpot::Piece(piece) => {
                    if piece.color == *color && piece.piece_type == PieceType::KING {
                        return Ok(Position {
                            line: line_index,
                            column: column_index,
                        });
                    }
                }
                BoardSpot::BLANK => continue,
            }
        }
    }
    return Err("Error while findig for king");
}
fn verify_if_was_check(board: &Board, color: Color) -> bool {
    let king_position = find_king(board, &color).unwrap();
    let enemy_color = color.reverse();
    let all_enemy_pieces = find_all_one_color_pieces(board, enemy_color);
    let all_enemy_possible_movements =
        generate_all_pieces_possible_movements(board, all_enemy_pieces);
    all_enemy_possible_movements.contains(&king_position)
}

fn read_player_move() -> String {
    print!("Your move: ");
    stdout().flush().unwrap();

    let mut player_move = String::new();
    stdin()
        .read_line(&mut player_move)
        .expect("Error computing your move!");
    player_move
}

fn show_board(board: &Board) {
    let mut line_number = 8;
    for line in board.iter().rev() {
        print!("{} ", line_number);
        for spot in line.iter() {
            print!("{} ", spot.to_board_representation());
        }
        line_number -= 1;
        println!("");
    }
    print!("  ");
    for letter in 'A'..'I' {
        print!("{} ", letter);
    }
    println!("");
    println!("");
}

fn get_info(board: &Board, player_move: String) -> [Result<Info, &'static str>; 2] {
    let positions: Vec<&str> = player_move.split(',').map(|s| s.trim()).collect();
    let start = translate_notation(board, positions[0]);
    let end = translate_notation(board, positions[1]);
    [start, end]
}

fn find_all_one_color_pieces(board: &Board, color: Color) -> Vec<Info> {
    let mut pieces: Vec<Info> = vec![];

    for (line_index, line) in board.iter().enumerate() {
        for (column_index, column) in line.iter().enumerate() {
            match column {
                BoardSpot::Piece(piece) => {
                    if piece.color == color {
                        pieces.push(Info {
                            position: Position {
                                line: line_index,
                                column: column_index,
                            },
                            piece: BoardSpot::Piece(Piece {
                                piece_type: piece.piece_type,
                                color: piece.color,
                            }),
                        })
                    }
                }
                BoardSpot::BLANK => continue,
            }
        }
    }

    pieces
}

fn main() {
    let mut board: Board = [
        [
            BoardSpot::Piece(Piece {
                piece_type: PieceType::ROOK,
                color: Color::WHITE,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::KNIGHT,
                color: Color::WHITE,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::BISHOP,
                color: Color::WHITE,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::QUEEN,
                color: Color::WHITE,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::KING,
                color: Color::WHITE,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::BISHOP,
                color: Color::WHITE,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::KNIGHT,
                color: Color::WHITE,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::ROOK,
                color: Color::WHITE,
            }),
        ],
        [
            BoardSpot::Piece(Piece {
                piece_type: PieceType::PAWN,
                color: Color::WHITE,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::PAWN,
                color: Color::WHITE,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::PAWN,
                color: Color::WHITE,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::PAWN,
                color: Color::WHITE,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::PAWN,
                color: Color::WHITE,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::PAWN,
                color: Color::WHITE,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::PAWN,
                color: Color::WHITE,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::PAWN,
                color: Color::WHITE,
            }),
        ],
        [
            BoardSpot::BLANK,
            BoardSpot::BLANK,
            BoardSpot::BLANK,
            BoardSpot::BLANK,
            BoardSpot::BLANK,
            BoardSpot::BLANK,
            BoardSpot::BLANK,
            BoardSpot::BLANK,
        ],
        [
            BoardSpot::BLANK,
            BoardSpot::BLANK,
            BoardSpot::BLANK,
            BoardSpot::BLANK,
            BoardSpot::BLANK,
            BoardSpot::BLANK,
            BoardSpot::BLANK,
            BoardSpot::BLANK,
        ],
        [
            BoardSpot::BLANK,
            BoardSpot::BLANK,
            BoardSpot::BLANK,
            BoardSpot::BLANK,
            BoardSpot::BLANK,
            BoardSpot::BLANK,
            BoardSpot::BLANK,
            BoardSpot::BLANK,
        ],
        [
            BoardSpot::BLANK,
            BoardSpot::BLANK,
            BoardSpot::BLANK,
            BoardSpot::BLANK,
            BoardSpot::BLANK,
            BoardSpot::BLANK,
            BoardSpot::BLANK,
            BoardSpot::BLANK,
        ],
        [
            BoardSpot::Piece(Piece {
                piece_type: PieceType::PAWN,
                color: Color::BLACK,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::PAWN,
                color: Color::BLACK,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::PAWN,
                color: Color::BLACK,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::PAWN,
                color: Color::BLACK,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::PAWN,
                color: Color::BLACK,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::PAWN,
                color: Color::BLACK,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::PAWN,
                color: Color::BLACK,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::PAWN,
                color: Color::BLACK,
            }),
        ],
        [
            BoardSpot::Piece(Piece {
                piece_type: PieceType::ROOK,
                color: Color::BLACK,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::KNIGHT,
                color: Color::BLACK,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::BISHOP,
                color: Color::BLACK,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::QUEEN,
                color: Color::BLACK,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::KING,
                color: Color::BLACK,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::BISHOP,
                color: Color::BLACK,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::KNIGHT,
                color: Color::BLACK,
            }),
            BoardSpot::Piece(Piece {
                piece_type: PieceType::ROOK,
                color: Color::BLACK,
            }),
        ],
    ];

    println!("");
    println!("{:-^40}", "TERMINAL CHESS");
    println!("");

    let mut turn = Color::WHITE;
    loop {
        show_board(&board);
        let was_check = verify_if_was_check(&board, turn);
        if was_check {
            println!("XEQUE!!!");
        }

        let player_move = read_player_move();

        let [start, end] = match get_info(&board, player_move).unwrap_array() {
            Ok(positions) => positions,
            Err(err) => {
                println!("{err}");
                continue;
            }
        };

        match start.piece {
            BoardSpot::Piece(piece) => {
                if piece.color != turn {
                    println!("Nao e o seu turno");
                    continue;
                }
                let (new_board, was_moved) =
                    move_piece(&board, piece, &start.position, &end.position);
                board = new_board;
                if was_moved {
                    turn = turn.reverse();
                }
            }
            BoardSpot::BLANK => {
                println!("Escolha uma peca valida");
                continue;
            }
        }

        // if (piece_color == "White" && is_white_turn) || (piece_color == "Black" && !is_white_turn) {
        //     let (new_board, was_moved) = move_piece(&board, &start, &end);
        //     board = new_board;
        //     if was_moved {
        //         is_white_turn = !is_white_turn;
        //
        //         let new_position = Info {
        //             position: Position {
        //                 line: end.position.line,
        //                 column: end.position.column,
        //             },
        //             piece: start.piece,
        //         };
        //         /* let was_check = verify_if_was_check(&board, &new_position); */
        //
        //         //TO DO
        //         if was_check {
        //             loop {}
        //         }
        //     }
        //     /* show_board(&board); */
        // } else {
        //     println!("NAO E SUA VEZ DE JOGAR");
        // }
    }
}
