use chess::game::{Game, Player};
use chess::board::{Board, Movement, Position, MovementError};
use chess::pieces::Bishop;
use chess::pieces::{Piece, Color::{self, Black, White}, Pawn, Rook, Queen, King, PieceFactory};
use chess::{assert_ok, assert_err};
use std::collections::{HashSet, HashMap};

fn create_board<'a>(
  positions: &'a mut HashMap<Position, Box<dyn Piece>>,
  pieces_set: &'a mut HashMap<Color, HashSet<Position>>,
  pieces_dead: &'a mut HashMap<Color, Vec<Box<dyn Piece>>>
) -> Board<'a> {

  let dimension = Position { x: 7, y: 7 };
  let pieces: Option<Vec<(Position, Box<dyn Piece>)>> = None;
  Board::new(dimension, pieces, positions, pieces_set, pieces_dead)
}

fn create_game<'a>(board: &'a mut Board<'a>) -> Game<'a> {
  let player_a = Player { name: "Nacho", color: White };
  let player_b = Player { name: "Pepe", color: Black };
  Game::new(board, (player_a, player_b))
}

fn add_pieces_move_and_clean<T: Piece + 'static>(game: &mut Game, init_position: Position, dest_position: Position, maybe_blocker: Option<Position>) -> Result<(), MovementError> {
  let movement = Movement { from: init_position, to: dest_position };
  let mut pieces = vec![(movement.from, PieceFactory::create::<T>(White))];
  if let Some(blocker_position) = maybe_blocker {
    pieces.push((blocker_position, PieceFactory::create::<T>(White)));
  }

  game.board.add_pieces(pieces);
  let result = game.board.move_piece(White, &movement);

  // Remove pieces from board
  assert_ok!(game.board.clean());

  result
}

// HORIZONTAL movement helpers
fn test_horizontal_right<T: Piece + 'static>(game: &mut Game, max: i32, maybe_blocker: Option<Position>) -> Result<(), MovementError> {
  let init_position = Position { x: 0, y: 0 };
  let dest_position =  Position {x: max, y: 0 };
  add_pieces_move_and_clean::<T>(game, init_position, dest_position, maybe_blocker)
}

fn test_horizontal_left<T: Piece + 'static>(game: &mut Game, max: i32, maybe_blocker: Option<Position>) -> Result<(), MovementError> {
  let init_position = Position { x: max, y: 0 };
  let dest_position =  Position {x: 0, y: 0 };
  add_pieces_move_and_clean::<T>(game, init_position, dest_position, maybe_blocker)
}

fn test_vertical_forward<T: Piece + 'static>(game: &mut Game, max: i32, maybe_blocker: Option<Position>) -> Result<(), MovementError> {
  let init_position = Position { x: 0, y: 0 };
  let dest_position =  Position {x: 0, y: max };
  add_pieces_move_and_clean::<T>(game, init_position, dest_position, maybe_blocker)
}

fn test_vertical_backward<T: Piece + 'static>(game: &mut Game, max: i32, maybe_blocker: Option<Position>) -> Result<(), MovementError> {
  let init_position = Position { x: 0, y: max };
  let dest_position =  Position {x: 0, y: 0 };
  add_pieces_move_and_clean::<T>(game, init_position, dest_position, maybe_blocker)
}

fn valid_horizontal_vertical_movements<T: Piece + 'static>(game: &mut Game, max_dimension_x: i32, max_dimension_y: i32) {
  (1..=max_dimension_x).into_iter().for_each(|i| {
    assert_ok!(test_horizontal_right::<T>(game, i, None));
    assert_ok!(test_horizontal_left::<T>(game, i, None));
  });

  (1..=max_dimension_y).into_iter().for_each(|i| {
    assert_ok!(test_vertical_forward::<T>(game, i, None));
    assert_ok!(test_vertical_backward::<T>(game, i, None));
  });
}

fn fail_if_horizontal_vertical_path_blocked<T: Piece + 'static>(game: &mut Game, max_dimension_x: i32, max_dimension_y: i32) {
  (1..=max_dimension_x).into_iter().for_each(|i| {
    let blocker_position = Some(Position { x: i, y: 0 });
    assert_err!(test_horizontal_right::<Rook>(game, max_dimension_x, blocker_position), MovementError::BlockedPath);
  });

  (0..=max_dimension_x-1).into_iter().for_each(|i| {
    let blocker_position = Some(Position { x: i, y: 0 });
    assert_err!(test_horizontal_left::<Rook>(game, max_dimension_x, blocker_position), MovementError::BlockedPath);
  });
  
  (1..=max_dimension_y).into_iter().for_each(|i| {
    let blocker_position = Some(Position { x: 0, y: i });
    assert_err!(test_vertical_forward::<Rook>(game, max_dimension_y, blocker_position), MovementError::BlockedPath);
  });

  (0..=max_dimension_y-1).into_iter().for_each(|i| {
    let blocker_position = Some(Position { x: 0, y: i });
    assert_err!(test_vertical_backward::<Rook>(game, max_dimension_y, blocker_position), MovementError::BlockedPath);
  });
}

// DIAGONAL movement helpers
fn test_diagonal_forward_right<T: Piece + 'static>(game: &mut Game, max: i32, maybe_blocker: Option<Position>) -> Result<(), MovementError> {
  let init_position = Position { x: 0, y: 0 };
  let dest_position =  Position {x: max, y: max };
  add_pieces_move_and_clean::<T>(game, init_position, dest_position, maybe_blocker)
}

fn test_diagonal_forward_left<T: Piece + 'static>(game: &mut Game, max: i32, maybe_blocker: Option<Position>) -> Result<(), MovementError> {
  let init_position = Position { x: max, y: 0 };
  let dest_position =  Position {x: 0, y: max };
  add_pieces_move_and_clean::<T>(game, init_position, dest_position, maybe_blocker)
}

fn test_diagonal_backward_right<T: Piece + 'static>(game: &mut Game, max: i32, maybe_blocker: Option<Position>) -> Result<(), MovementError> {
  let init_position = Position { x: 0, y: max };
  let dest_position =  Position {x: max, y: 0 };
  add_pieces_move_and_clean::<T>(game, init_position, dest_position, maybe_blocker)
}

fn test_diagonal_backward_left<T: Piece + 'static>(game: &mut Game, max: i32, maybe_blocker: Option<Position>) -> Result<(), MovementError> {
  let init_position = Position { x: max, y: max };
  let dest_position =  Position {x: 0, y: 0 };
  add_pieces_move_and_clean::<T>(game, init_position, dest_position, maybe_blocker)
}

fn valid_diagonal_movements<T: Piece + 'static>(game: &mut Game, max_dimension_x: i32, max_dimension_y: i32) {
  let dimension = max_dimension_x.min(max_dimension_y);

  (1..=dimension).into_iter().for_each(|i| {
    assert_ok!(test_diagonal_forward_right::<T>(game, i, None));
    assert_ok!(test_diagonal_forward_left::<T>(game, i, None));
    assert_ok!(test_diagonal_backward_right::<T>(game, i, None));
    assert_ok!(test_diagonal_backward_left::<T>(game, i, None));
  });
}

fn fail_if_diagonal_path_blocked<T: Piece + 'static>(game: &mut Game, max_dimension_x: i32, max_dimension_y: i32) {
  let dimension = max_dimension_x.min(max_dimension_y);

  (1..=dimension).into_iter().for_each(|i| {
    let blocker_position = Some(Position { x: i, y: i });
    assert_err!(test_diagonal_forward_right::<Bishop>(game, dimension, blocker_position), MovementError::BlockedPath);
  });

  (1..=dimension).into_iter().for_each(|i| {
    let blocker_position = Some(Position { x: dimension - i, y: i });
    assert_err!(test_diagonal_forward_left::<Bishop>(game, dimension, blocker_position), MovementError::BlockedPath);
  });
  
  (1..=dimension).into_iter().for_each(|i| {
    let blocker_position = Some(Position { x: dimension - i, y: dimension - i });
    assert_err!(test_diagonal_backward_left::<Bishop>(game, dimension, blocker_position), MovementError::BlockedPath);
  });

  (1..=dimension).into_iter().for_each(|i| {
    let blocker_position = Some(Position { x: i, y: dimension - i });
    assert_err!(test_diagonal_backward_right::<Bishop>(game, dimension, blocker_position), MovementError::BlockedPath);
  });
}

#[test]
fn test_pawn_movements() {
  let mut positions = HashMap::new();
  let mut pieces_set = HashMap::new();
  let mut pieces_dead = HashMap::new();

  let mut board = create_board(&mut positions, &mut pieces_set, &mut pieces_dead);
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
  assert_ok!(game.board.move_piece(White, &movement));

  // Test Forward(2)
  end_position = Position { x: 1, y: 2 };
  movement = Movement { from: initial_position_b, to: end_position };
  assert_ok!(game.board.move_piece(White, &movement));

}

#[test]
fn valid_rook_movements() {
  let mut positions = HashMap::new();
  let mut pieces_set = HashMap::new();
  let mut pieces_dead = HashMap::new();

  let mut board = create_board(&mut positions, &mut pieces_set, &mut pieces_dead);
  let mut game = create_game(&mut board);
  let max_dimension_x = game.board.dimension.x;
  let max_dimension_y = game.board.dimension.y;

  valid_horizontal_vertical_movements::<Rook>(&mut game, max_dimension_x, max_dimension_y);
}

#[test]
fn valid_bishop_movements() {
  let mut positions = HashMap::new();
  let mut pieces_set = HashMap::new();
  let mut pieces_dead = HashMap::new();

  let mut board = create_board(&mut positions, &mut pieces_set, &mut pieces_dead);
  let mut game = create_game(&mut board);

  let max_dimension_x = game.board.dimension.x;
  let max_dimension_y = game.board.dimension.y;

  valid_diagonal_movements::<Bishop>(&mut game, max_dimension_x, max_dimension_y);
}

#[test]
fn valid_queen_movements() {
  let mut positions = HashMap::new();
  let mut pieces_set = HashMap::new();
  let mut pieces_dead = HashMap::new();

  let mut board = create_board(&mut positions, &mut pieces_set, &mut pieces_dead);
  let mut game = create_game(&mut board);

  let max_dimension_x = game.board.dimension.x;
  let max_dimension_y = game.board.dimension.y;

  valid_horizontal_vertical_movements::<Queen>(&mut game, max_dimension_x, max_dimension_y);
  valid_diagonal_movements::<Queen>(&mut game, max_dimension_x, max_dimension_y);
}

#[test]
fn valid_king_movements() {
  let mut positions = HashMap::new();
  let mut pieces_set = HashMap::new();
  let mut pieces_dead = HashMap::new();

  let mut board = create_board(&mut positions, &mut pieces_set, &mut pieces_dead);
  let mut game = create_game(&mut board);

  let max_dimension_x = 1;
  let max_dimension_y = 1;

  valid_horizontal_vertical_movements::<King>(&mut game, max_dimension_x, max_dimension_y);
  valid_diagonal_movements::<King>(&mut game, max_dimension_x, max_dimension_y);
}

#[test]
fn rook_fail_if_path_blocked() {
  let mut positions = HashMap::new();
  let mut pieces_set = HashMap::new();
  let mut pieces_dead = HashMap::new();

  let mut board = create_board(&mut positions, &mut pieces_set, &mut pieces_dead);
  let mut game = create_game(&mut board);

  let max_dimension_x = game.board.dimension.x;
  let max_dimension_y = game.board.dimension.y;

  fail_if_horizontal_vertical_path_blocked::<Rook>(&mut game, max_dimension_x, max_dimension_y);
}

#[test]
fn bishop_fail_if_path_blocked() {
  let mut positions = HashMap::new();
  let mut pieces_set = HashMap::new();
  let mut pieces_dead = HashMap::new();

  let mut board = create_board(&mut positions, &mut pieces_set, &mut pieces_dead);
  let mut game = create_game(&mut board);

  let max_dimension_x = game.board.dimension.x;
  let max_dimension_y = game.board.dimension.y;

  fail_if_diagonal_path_blocked::<Bishop>(&mut game, max_dimension_x, max_dimension_y);
}

#[test]
fn queen_fail_if_path_blocked() {
  let mut positions = HashMap::new();
  let mut pieces_set = HashMap::new();
  let mut pieces_dead = HashMap::new();

  let mut board = create_board(&mut positions, &mut pieces_set, &mut pieces_dead);
  let mut game = create_game(&mut board);

  let max_dimension_x = game.board.dimension.x;
  let max_dimension_y = game.board.dimension.y;

  fail_if_horizontal_vertical_path_blocked::<Queen>(&mut game, max_dimension_x, max_dimension_y);
  fail_if_diagonal_path_blocked::<Queen>(&mut game, max_dimension_x, max_dimension_y);
}

#[test]
fn king_fail_if_path_blocked() {
  let mut positions = HashMap::new();
  let mut pieces_set = HashMap::new();
  let mut pieces_dead = HashMap::new();

  let mut board = create_board(&mut positions, &mut pieces_set, &mut pieces_dead);
  let mut game = create_game(&mut board);

  let max_dimension_x = 1;
  let max_dimension_y = 1;

  fail_if_horizontal_vertical_path_blocked::<King>(&mut game, max_dimension_x, max_dimension_y);
  fail_if_diagonal_path_blocked::<King>(&mut game, max_dimension_x, max_dimension_y);
}
