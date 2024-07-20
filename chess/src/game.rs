use std::io::{self, Write};
use crate::pieces::Knight;
use super::ensure;

use super::board::{Board, Movement, MovementError, Position};
use super::pieces::{
  Color::{self, Black, White}, Piece, PieceFactory,
  Pawn, King, Queen, Rook, Bishop
};

pub struct Player<'a> {
	pub name: &'a str,
	pub color: Color,
}

#[derive(PartialEq, Eq, Debug)]
pub enum GameState {
	Ready,
	OnGoing,
  Paused,
	Ended,
}

#[derive(Debug)]
pub struct Game<'a> {
	pub board: &'a mut Board<'a>,
	pub state: GameState,
	// pub players: (Player<'a>, Player<'a>),
	pub turn: u32,
}

impl<'a> Game<'a> {
  pub fn new(board: &'a mut Board<'a>) -> Self {
    Game {
      board,
      state: GameState::Ready,
      turn: 0,
    }
  }

  pub fn ask_movement(&self) -> Result<Movement, MovementError> {
    print!("{:?}, your turn:", self.playing_color());
    // Flush the standard output to ensure the prompt is shown before reading input
    io::stdout().flush().unwrap();

    let mut movement_string = String::new();
    io::stdin().read_line(&mut movement_string).expect("Failed to read line");

    // Remove the newline character from the end of the input
    // movement_string.trim().to_string();
    self.translate_movement(movement_string.trim().to_string())
    // self.move_piece(movement)?;
    // Ok(())
  }

  fn translate_movement(&self, movement: String) -> Result<Movement, MovementError> {
    ensure!(movement.len() == 4, MovementError::WrongCommand(movement));

    let from = &movement[0..2];
    let to = &movement[2..4];

    let from_x_char = &from.split_at(1).0.chars().next().expect("exists");
    let from_y_char = &from.split_at(1).1.chars().next().expect("exists");
    let to_x_char = &to.split_at(1).0.chars().next().expect("exists");
    let to_y_char = &to.split_at(1).1.chars().next().expect("exists");

    let from_x  = match from_x_char {
        'a'..='z' => Some(*from_x_char as i32 - 'a' as i32),
        'A'..='Z' => Some(*from_x_char as i32 - 'A' as i32),
        _ => None, // Return None if the character is not in the range a-z or A-Z
    }.ok_or(MovementError::WrongCommand(movement.clone()))?;
    
    let to_x  = match to_x_char {
      'a'..='z' => Some(*to_x_char as i32 - 'a' as i32),
      'A'..='Z' => Some(*to_x_char as i32 - 'A' as i32),
      _ => None, // Return None if the character is not in the range a-z or A-Z
    }.ok_or(MovementError::WrongCommand(movement.clone()))?;

    let from_y = from_y_char.to_digit(10).ok_or(MovementError::WrongCommand(movement.clone()))? as i32;
    let to_y = to_y_char.to_digit(10).ok_or(MovementError::WrongCommand(movement))? as i32;

    Ok(
      Movement {
        from: Position { x: from_x, y: from_y - 1 },
        to: Position { x: to_x, y: to_y - 1 },
      }
    )

  }

  pub fn set_board(&mut self) {
    let mut pieces = self.build_pieces(White);
    pieces.extend(self.build_pieces(Black));
    self.board.add_pieces(pieces)
  }

  fn build_pieces(&mut self, color: Color) -> Vec<(Position, Box<dyn Piece>)> {
    let (row, offset) = match color {
      White => (0, 1),
      Black => (self.board.dimension.y, -1),
    };
    //King
    let king = (Position { x: 4, y: row }, PieceFactory::create::<King>(color));
    //King
    let queen = (Position { x: 3, y: row }, PieceFactory::create::<Queen>(color));
    // Rooks
    let rook_left = (Position { x: 0, y: row }, PieceFactory::create::<Rook>(color));
    let rook_right = (Position { x: 7, y: row }, PieceFactory::create::<Rook>(color));
    // Knights
    let knight_left = (Position { x: 1, y: row }, PieceFactory::create::<Knight>(color));
    let knight_right = (Position { x: 6, y: row }, PieceFactory::create::<Knight>(color));
    // Bishops
    let bishop_left = (Position { x: 2, y: row }, PieceFactory::create::<Bishop>(color));
    let bishop_right = (Position { x: 5, y: row }, PieceFactory::create::<Bishop>(color));

    let mut pieces = vec![king, queen, rook_left, rook_right, knight_left, knight_right, bishop_left, bishop_right];
    
    // Paws
    for x in 0..=self.board.dimension.x {
      let pawn = (Position { x, y: row + offset }, PieceFactory::create::<Pawn>(color));
      pieces.push(pawn)
    }
    pieces
  }

  pub fn start(&mut self) {
    self.state = GameState::OnGoing;
  }

  pub fn end(&mut self) {
    self.state = GameState::Ended;
  }

  pub fn is_ongoing(&self) -> bool {
    self.state == GameState::OnGoing
  }

  pub fn new_turn(&mut self) {
    self.turn+=1;
  }

  pub fn move_piece(&mut self, movement: Movement) -> Result<(), MovementError> {
    let res = self.board.move_piece(self.playing_color(), &movement);
    println!("{:?}", self.board.positions);
    res
  }

  fn playing_color(&self) -> Color {
    if self.turn%2 == 0 { White } else { Black }
  }

  pub fn print_board(&self) -> String {
    use colored::*;
    let mut result : Vec<String> = Vec::new();
    let x_max = self.board.dimension.x;
    let y_max = self.board.dimension.y;
    let mut board:Vec<Vec<char>> = Vec::new();
    let mut left_numbers: Vec<usize> = (0..=y_max as usize).collect();
    let mut bottom_letters: Vec<usize> = (0..=x_max as usize).collect();

    for y in 0..=y_max {
      let mut row = Vec::new();
      for x in 0..=x_max {
        let position = Position { x: x as i32, y: y as i32 };
        let piece = self.board.positions.get(&position).map_or(' ', |p| p.symbol() );
        row.push(piece);
      }
      board.push(row);
    }

    if self.playing_color() == White { 
      board.reverse();
    }
    if self.playing_color() == Black {
      left_numbers.reverse();
      bottom_letters.reverse();
      for y in 0..=y_max {
        board[y as usize].reverse();
      }
    }

    for y in 0..=y_max {
        // Print left numbers
        result.push(format!("{} ", left_numbers[(y_max - y) as usize] + 1));
        for x in 0..=x_max {
            let square = board[y as usize][x as usize];
            if (x + y) % 2 == 0 {
                result.push(format!("{}", format!(" {} ", square).white().on_black()));
            } else {
                result.push(format!("{}", format!(" {} ", square).black().on_white()));
            }
        }
        result.push(format!("\n"));
    }

    // Print bottom letters
    result.push(format!("  "));
    for x in bottom_letters {
        result.push(format!(" {} ", (b'a' + x as u8) as char));
    }
    result.push(format!("\n"));

    result.iter().flat_map(|s| s.chars()).collect()
  }
}
