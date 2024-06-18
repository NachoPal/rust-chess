use chess::game::{Game, Player};
use chess::board::{Board, Movement, Position};
use chess::pieces::Bishop;
use chess::pieces::{Piece, Color::{self, Black, White}, Pawn, Rook, PieceFactory};
use chess::assert_ok;
use std::collections::{HashSet, HashMap};

fn create_board<'a>(
  positions: &'a mut HashMap<Position, Box<dyn Piece>>,
  pieces_set: &'a mut HashMap<Color, HashSet<Position>>
) -> Board<'a> {

  let dimension = Position { x: 7, y: 7 };
  let pieces: Option<Vec<(Position, Box<dyn Piece>)>> = None;
  Board::new(dimension, pieces, positions, pieces_set)
}

fn create_game<'a>(board: &'a mut Board<'a>) -> Game<'a> {
  let player_a = Player { name: "Nacho", color: White };
  let player_b = Player { name: "Pepe", color: Black };
  let mut game = Game::new(board, (player_a, player_b));
  // game.set_board();
  game
}


#[test]
fn test_pawn_movements() {
  let mut positions = HashMap::new();
  let mut pieces_set = HashMap::new();

  let mut board = create_board(&mut positions, &mut pieces_set);
  let game = create_game(&mut board);

  // Test White
  let initial_position_a = Position { x: 0, y: 1 };
  let initial_position_b = Position { x: 1, y: 1 };

  let pawn_a = (initial_position_a, PieceFactory::create::<Pawn>(White));
  let pawn_b = (initial_position_b, PieceFactory::create::<Pawn>(White));

  game.board.add_pieces(vec![pawn_a, pawn_b]);

  // Test Forward(1)
  let mut end_position = Position { x: 0, y: 2 };
  let mut movement = Movement { from: initial_position_a, to: end_position };
  assert_ok!(game.board.move_piece(White, movement));

  // Test Forward(2)
  end_position = Position { x: 1, y: 2 };
  movement = Movement { from: initial_position_b, to: end_position };
  assert_ok!(game.board.move_piece(White, movement));

}

fn test_horizontal_vertical_movement<T: Piece + 'static>(game: &mut Game, max: i32) {
  let mut initial_position = Position { x: 0, y: 0 };
  let piece = (initial_position, PieceFactory::create::<T>(White));

  game.board.add_pieces(vec![piece]);

  // Test Right
  let mut end_position = Position { x: max, y: 0 };
  let mut movement = Movement { from: initial_position, to: end_position };
  assert_ok!(game.board.move_piece(White, movement));

  // Test Forward
  initial_position = end_position;
  end_position = Position { x: max, y: max };
  movement = Movement { from: initial_position, to: end_position };
  assert_ok!(game.board.move_piece(White, movement));

  // Test Left
  initial_position = end_position;
  end_position = Position { x: 0, y: max };
  movement = Movement { from: initial_position, to: end_position };
  assert_ok!(game.board.move_piece(White, movement));

  // Test Backwards
  initial_position = end_position;
  end_position = Position { x: 0, y: 0 };
  movement = Movement { from: initial_position, to: end_position };
  assert_ok!(game.board.move_piece(White, movement));
}

#[test]
fn test_rook_movements() {
  let mut positions = HashMap::new();
  let mut pieces_set = HashMap::new();

  let mut board = create_board(&mut positions, &mut pieces_set);
  let mut game = create_game(&mut board);
  let positions = 7;
  // let rook = (initial_position, PieceFactory::create::<Rook>(White));

  test_horizontal_vertical_movement::<Rook>(&mut game, positions)
}

#[test]
fn test_bishop_movements() {
  let mut positions = HashMap::new();
  let mut pieces_set = HashMap::new();

  let mut board = create_board(&mut positions, &mut pieces_set);
  let game = create_game(&mut board);

  let mut initial_position = Position { x: 0, y: 0 };
  let bishop = (initial_position, PieceFactory::create::<Bishop>(White));

  game.board.add_pieces(vec![bishop]);

  // Test Diagonal Forward Right
  let mut end_position = Position { x: 7, y: 7 };
  let mut movement = Movement { from: initial_position, to: end_position };
  assert_ok!(game.board.move_piece(White, movement));

  // Test Diagonal Backward Left
  initial_position = end_position;
  end_position = Position { x: 3, y: 3 };
  movement = Movement { from: initial_position, to: end_position };
  assert_ok!(game.board.move_piece(White, movement));

  // Test Diagonal Forward Left
  initial_position = end_position;
  end_position = Position { x: 1, y: 5 };
  movement = Movement { from: initial_position, to: end_position };
  assert_ok!(game.board.move_piece(White, movement));

  // Test Backwards Right
  initial_position = end_position;
  end_position = Position { x: 5, y: 1 };
  movement = Movement { from: initial_position, to: end_position };
  assert_ok!(game.board.move_piece(White, movement));
}
