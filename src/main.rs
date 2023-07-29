// TO DO: XEQUE DESCOBERTO, MOVIMENTOS ESPECIAIS, MELHORAR A NOTACAO, MAIS DE 1 PECA DANDO XEQUE AO
// MESMO TEMPO

use regex::Regex;
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

const DIAGONALS: [[isize; 2]; 4] = [[1, 1], [1, -1], [-1, 1], [-1, -1]];
const SIDES: [[isize; 2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];

type Board = [[char; 8]; 8];
type Columns = HashMap<char, u32>;

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.column == other.column
    }
}

impl ToString for Piece {
    fn to_string(&self) -> String {
        match self {
            Piece::WHITEPAWN => String::from("White Pawn"),
            Piece::WHITEROOK => String::from("White Rook"),
            Piece::WHITEKNIGHT => String::from("White Knight"),
            Piece::WHITEBISHOP => String::from("White Bishop"),
            Piece::WHITEQUEEN => String::from("White Queen"),
            Piece::WHITEKING => String::from("White King"),

            Piece::BLACKPAWN => String::from("Black Pawn"),
            Piece::BLACKROOK => String::from("Black Rook"),
            Piece::BLACKKNIGHT => String::from("Black Knight"),
            Piece::BLACKBISHOP => String::from("Black Bishop"),
            Piece::BLACKQUEEN => String::from("Black Queen"),
            Piece::BLACKKING => String::from("Black King"),

            Piece::BLANK => String::from("Blank"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Piece {
    WHITEPAWN,
    WHITEROOK,
    WHITEKNIGHT,
    WHITEBISHOP,
    WHITEQUEEN,
    WHITEKING,
    BLACKPAWN,
    BLACKROOK,
    BLACKKNIGHT,
    BLACKBISHOP,
    BLACKQUEEN,
    BLACKKING,
    BLANK,
}

#[derive(PartialEq, Eq)]
enum CheckPiece {
    SAMECOLOR,
    DIFFERENTCOLOR,
    NOPIECE,
}

#[derive(Debug, Clone)]
struct Position {
    line: usize,
    column: usize,
}

#[derive(Debug)]
struct Info {
    position: Position,
    piece: Piece,
}

fn diagonals_movement_check(board: &Board, piece: &Info, piece_color: &String) -> Vec<Position> {
    let mut possible_positions: Vec<Position> = vec![];
    for movement in DIAGONALS {
        let mut new_position = piece.position.clone();
        while new_position.line < 8 && new_position.column < 8 {
            new_position.line = (new_position.line as isize + movement[0]) as usize;
            new_position.column = (new_position.column as isize + movement[1]) as usize;

            if new_position.line >= 8 || new_position.column >= 8 {
                break;
            }

            let check_piece =
                check_piece_in_possible_movement_spot(board, &piece_color, &new_position);
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

fn sides_movement_check(board: &Board, piece: &Info, piece_color: &String) -> Vec<Position> {
    let mut possible_positions: Vec<Position> = vec![];
    for movement in SIDES {
        let mut new_position = piece.position.clone();
        while new_position.line < 8 && new_position.column < 8 {
            new_position.line = (new_position.line as isize + movement[0]) as usize;
            new_position.column = (new_position.column as isize + movement[1]) as usize;
            if new_position.line >= 8 || new_position.column >= 8 {
                break;
            }

            let check_piece =
                check_piece_in_possible_movement_spot(board, &piece_color, &new_position);

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

fn get_piece(piece: char) -> Piece {
    match piece {
        '♟' => Piece::WHITEPAWN,
        '♜' => Piece::WHITEROOK,
        '♞' => Piece::WHITEKNIGHT,
        '♝' => Piece::WHITEBISHOP,
        '♛' => Piece::WHITEQUEEN,
        '♚' => Piece::WHITEKING,

        '♙' => Piece::BLACKPAWN,
        '♖' => Piece::BLACKROOK,
        '♘' => Piece::BLACKKNIGHT,
        '♗' => Piece::BLACKBISHOP,
        '♕' => Piece::BLACKQUEEN,
        '♔' => Piece::BLACKKING,

        _ => Piece::BLANK,
    }
}

fn get_piece_color(piece: String) -> String {
    let color_regex = Regex::new(r"^[A-Z][a-z]{4}").unwrap();
    let color = color_regex.captures(&piece).unwrap();
    color[0].to_string()
}

fn translate_notation(board: &Board, notation_position: &str, columns: &Columns) -> Info {
    let vectorized_position: Vec<char> = notation_position.chars().collect();
    let column = columns.get(&vectorized_position[0]).unwrap_or(&0);

    if *column == 0 {
        Info {
            position: Position { column: 0, line: 0 },
            piece: Piece::BLANK,
        }
    } else {
        let line = vectorized_position[1].to_digit(10).unwrap_or(0);
        if line == 0 {
            return Info {
                position: Position { column: 0, line: 0 },
                piece: Piece::BLANK,
            };
        }
        Info {
            position: Position {
                column: (*column - 1) as usize,
                line: (line - 1) as usize,
            },
            piece: get_piece(board[(line - 1) as usize][(*column - 1) as usize]),
        }
    }
}

fn check_piece_in_possible_movement_spot(
    board: &Board,
    piece_color: &String,
    position_to_check: &Position,
) -> CheckPiece {
    let piece_on_possible_position =
        get_piece(board[position_to_check.line][position_to_check.column]);
    match piece_on_possible_position {
        Piece::BLANK => CheckPiece::NOPIECE,
        _ => {
            let piece_on_possible_position_color =
                get_piece_color(piece_on_possible_position.to_string());
            if piece_on_possible_position_color == *piece_color {
                CheckPiece::SAMECOLOR
            } else {
                CheckPiece::DIFFERENTCOLOR
            }
        }
    }
}

fn generate_possible_movements(board: &Board, piece: &Info) -> Vec<Position> {
    let piece_color = get_piece_color(piece.piece.to_string());

    let mut possible_positions: Vec<Position> = vec![];
    match piece.piece {
        Piece::WHITEPAWN | Piece::BLACKPAWN => {
            let white_pawn_movements = HashMap::from([
                ("forward", [[1, 0], [2, 0]]),
                ("diagonal", [[1, 1], [1, -1]]),
            ]);
            for (key, value) in white_pawn_movements.iter() {
                for movement in value {
                    let line = piece.position.line as isize
                        + (if piece.piece == Piece::BLACKPAWN {
                            movement[0] * -1
                        } else {
                            movement[0]
                        });
                    let new_position = Position {
                        line: line as usize,
                        column: (piece.position.column as isize + movement[1]) as usize,
                    };
                    if new_position.line >= 8 || new_position.column >= 8 {
                        continue;
                    }
                    if key == &"forward" {
                        if check_piece_in_possible_movement_spot(board, &piece_color, &new_position)
                            == CheckPiece::NOPIECE
                        {
                            possible_positions.push(new_position);
                        }
                    } else {
                        if check_piece_in_possible_movement_spot(board, &piece_color, &new_position)
                            == CheckPiece::DIFFERENTCOLOR
                        {
                            possible_positions.push(new_position);
                        }
                    }
                }
            }
        }

        Piece::WHITEBISHOP | Piece::BLACKBISHOP => {
            possible_positions = diagonals_movement_check(board, piece, &piece_color);
        }

        Piece::WHITEKNIGHT | Piece::BLACKKNIGHT => {
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
                    line: (piece.position.line as isize + movemement[0]) as usize,
                    column: (piece.position.column as isize + movemement[1]) as usize,
                };

                if new_position.line >= 8 || new_position.column >= 8 {
                    continue;
                }

                let check_piece =
                    check_piece_in_possible_movement_spot(board, &piece_color, &new_position);

                if check_piece != CheckPiece::SAMECOLOR {
                    possible_positions.push(new_position);
                }
            }
        }

        Piece::WHITEROOK | Piece::BLACKROOK => {
            possible_positions = sides_movement_check(board, piece, &piece_color);
        }
        Piece::WHITEQUEEN | Piece::BLACKQUEEN => {
            let diagonals_moves = diagonals_movement_check(board, piece, &piece_color);
            possible_positions.extend(diagonals_moves);

            let sides_moves = sides_movement_check(board, piece, &piece_color);

            possible_positions.extend(sides_moves);
        }
        Piece::WHITEKING | Piece::BLACKKING => {
            let mut king_movements: Vec<[isize; 2]> = vec![];
            king_movements.extend(DIAGONALS);
            king_movements.extend(SIDES);
            for movement in king_movements {
                let new_position = Position {
                    line: (piece.position.line as isize + movement[0]) as usize,
                    column: (piece.position.column as isize + movement[1]) as usize,
                };

                if new_position.line >= 8 || new_position.column >= 8 {
                    continue;
                }

                let check_piece =
                    check_piece_in_possible_movement_spot(board, &piece_color, &new_position);

                if check_piece != CheckPiece::SAMECOLOR {
                    possible_positions.push(new_position);
                }
            }
        }

        _ => possible_positions = vec![],
    }
    possible_positions
}

fn check_move(board: &Board, starting_position: &Info, destination_position: &Position) -> bool {
    let possible_movements = generate_possible_movements(board, starting_position);
    if possible_movements.contains(&destination_position) {
        return true;
    }
    false
}

fn move_piece(
    board: &Board,
    starting_position: &Info,
    destination_position: &Info,
) -> (Board, bool) {
    let mut new_board = *board;
    let is_legal_move = check_move(board, starting_position, &destination_position.position);
    if is_legal_move {
        new_board[destination_position.position.line][destination_position.position.column] =
            board[starting_position.position.line][starting_position.position.column];
        new_board[starting_position.position.line][starting_position.position.column] = '.';
        (new_board, true)
    } else {
        println!("MOVIMENTO ILEGAL");
        (new_board, false)
    }
}

fn find_opposite_king(board: &Board, color: String) -> Result<Position, &'static str> {
    let opposite_king = if color == "White" { '♔' } else { '♚' };
    for (line_index, line) in board.iter().enumerate() {
        for (column_index, column) in line.iter().enumerate() {
            if column == &opposite_king {
                return Ok(Position {
                    line: line_index,
                    column: column_index,
                });
            }
        }
    }
    Err("invalid board")
}

fn verify_if_was_check(board: &Board, piece_info: &Info) -> bool {
    let color = get_piece_color(piece_info.piece.to_string());
    let opposite_king_position = find_opposite_king(board, color).unwrap();
    let piece_movements = generate_possible_movements(board, piece_info);
    piece_movements.contains(&opposite_king_position)
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
            print!("{} ", spot)
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

fn get_info(board: &Board, player_move: String, columns: &Columns) -> [Info; 2] {
    let positions: Vec<&str> = player_move.split(',').map(|s| s.trim()).collect();
    let start = translate_notation(board, positions[0], &columns);
    let end = translate_notation(board, positions[1], &columns);
    [start, end]
}

fn main() {
    let columns: Columns = HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
    ]);

    let mut board = [
        ['♜', '♞', '♝', '♛', '♚', '♝', '♞', '♜'],
        ['♟', '♟', '♟', '♟', '♟', '♟', '♟', '♟'],
        ['.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.'],
        ['♙', '♙', '♙', '♙', '♙', '♙', '♙', '♙'],
        ['♖', '♘', '♗', '♕', '♔', '♗', '♘', '♖'],
    ];

    println!("");
    println!("{:-^40}", "TERMINAL CHESS");
    println!("");

    let mut is_white_turn = true;

    show_board(&board);
    loop {
        let player_move = read_player_move();

        let [start, end] = get_info(&board, player_move, &columns);
        let piece_color = get_piece_color(start.piece.to_string());
        if piece_color == "Blank" {
            println!("Escolha uma peca valida");
            continue;
        }
        if (piece_color == "White" && is_white_turn) || (piece_color == "Black" && !is_white_turn) {
            let (new_board, was_moved) = move_piece(&board, &start, &end);
            board = new_board;
            if was_moved {
                is_white_turn = !is_white_turn;

                let new_position = Info {
                    position: Position {
                        line: end.position.line,
                        column: end.position.column,
                    },
                    piece: start.piece,
                };
                let was_check = verify_if_was_check(&board, &new_position);

                //TO DO
                if was_check {
                    loop {}
                }
            }
            show_board(&board);
        } else {
            println!("NAO E SUA VEZ DE JOGAR");
        }
    }
}
