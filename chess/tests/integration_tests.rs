use chess_lib::game::{Game, Player};
use chess_lib::board::{Board, Movement, Position, MovementError};
use chess_lib::pieces::Bishop;
use chess_lib::pieces::{Piece, Color::{self, Black, White}, Pawn, Rook, Queen, King, PieceFactory};
use chess_lib::{assert_ok, assert_err};

fn create_board() -> Board {
  let dimension = Position { x: 7, y: 7 };
  let pieces: Option<Vec<(Position, Box<dyn Piece>)>> = None;
  Board::new(dimension, pieces)
}

fn add_pieces_and_move<T: Piece + 'static>(
  game: &mut Game,
  init_position: Position,
  dest_position: Position,
  maybe_blocker: Option<(Position, Color)>,
) -> Result<(), MovementError> {
  let movement = Movement { from: init_position, to: dest_position };
  let mut pieces = vec![(movement.from, PieceFactory::create::<T>(White))];
  if let Some((blocker_position, blocker_color)) = maybe_blocker {
    pieces.push((blocker_position, PieceFactory::create::<T>(blocker_color)));
  }

  game.board.add_pieces(pieces);
  game.board.move_piece(White, &movement)
}

// HORIZONTAL movement helpers
fn test_horizontal_right<T: Piece + 'static>(game: &mut Game, max: i32, maybe_blocker: Option<(Position, Color)>, clean: bool) -> Result<(), MovementError> {
  let init_position = Position { x: 0, y: 0 };
  let dest_position =  Position {x: max, y: 0 };
  let result = add_pieces_and_move::<T>(game, init_position, dest_position, maybe_blocker);
  if clean { assert_ok!(game.board.clean()); };
  result
}

fn test_horizontal_left<T: Piece + 'static>(game: &mut Game, max: i32, maybe_blocker: Option<(Position, Color)>, clean: bool) -> Result<(), MovementError> {
  let init_position = Position { x: max, y: 0 };
  let dest_position =  Position {x: 0, y: 0 };
  let result = add_pieces_and_move::<T>(game, init_position, dest_position, maybe_blocker);
  if clean { assert_ok!(game.board.clean()); };
  result
}

fn test_vertical_forward<T: Piece + 'static>(game: &mut Game, max: i32, maybe_blocker: Option<(Position, Color)>, clean: bool) -> Result<(), MovementError> {
  let init_position = Position { x: 0, y: 0 };
  let dest_position =  Position {x: 0, y: max };
  let result = add_pieces_and_move::<T>(game, init_position, dest_position, maybe_blocker);
  if clean { assert_ok!(game.board.clean()); };
  result
}

fn test_vertical_backward<T: Piece + 'static>(game: &mut Game, max: i32, maybe_blocker: Option<(Position, Color)>, clean: bool) -> Result<(), MovementError> {
  let init_position = Position { x: 0, y: max };
  let dest_position =  Position {x: 0, y: 0 };
  let result = add_pieces_and_move::<T>(game, init_position, dest_position, maybe_blocker);
  if clean { assert_ok!(game.board.clean()); };
  result
}

fn horizontal_vertical_movements<T: Piece + 'static, F>(game: &mut Game, max_dimension_x: i32, max_dimension_y: i32, min_dimension: i32, assertion: F) where F: Fn(Result<(), MovementError>) -> () {
  (min_dimension..=max_dimension_x).into_iter().for_each(|i| {
    assertion(test_horizontal_right::<T>(game, i, None, true));
    assertion(test_horizontal_left::<T>(game, i, None, true));
  });

  (min_dimension..=max_dimension_y).into_iter().for_each(|i| {
    assertion(test_vertical_forward::<T>(game, i, None, true));
    assertion(test_vertical_backward::<T>(game, i, None, true));
  });
}

fn horizontal_vertical_path_blocked<T: Piece + 'static, F, G>(
  game: &mut Game,
  max_dimension_x: i32,
  max_dimension_y: i32,
  blocker_color: Color,
  assert_blocked_path: F,
  assert_blocked_target: G,
) where F: Fn(Result<(), MovementError>), G: Fn(Result<(), MovementError>, &mut Game) {
  (1..=max_dimension_x).into_iter().for_each(|i| {
    let blocker = Some((Position { x: i, y: 0 }, blocker_color));
    if i != max_dimension_x {
      assert_blocked_path(test_horizontal_right::<T>(game, max_dimension_x, blocker, true));
    } else {
      assert_blocked_target(test_horizontal_right::<T>(game, max_dimension_x, blocker, false), game);
      assert_ok!(game.board.clean());
    }
  });

  (0..=max_dimension_x-1).into_iter().for_each(|i| {
    let blocker = Some((Position { x: i, y: 0 }, blocker_color));
    if i != 0 {
      assert_blocked_path(test_horizontal_left::<T>(game, max_dimension_x, blocker, true));
    } else {
      assert_blocked_target(test_horizontal_left::<T>(game, max_dimension_x, blocker, false), game);
      assert_ok!(game.board.clean());
    }
  });

  (1..=max_dimension_y).into_iter().for_each(|i| {
    let blocker = Some((Position { x: 0, y: i }, blocker_color));
    if i != max_dimension_y {
      assert_blocked_path(test_vertical_forward::<T>(game, max_dimension_y, blocker, true));
    } else {
      assert_blocked_target(test_vertical_forward::<T>(game, max_dimension_y, blocker, false), game);
      assert_ok!(game.board.clean());
    }
  });

  (0..=max_dimension_y-1).into_iter().for_each(|i| {
    let blocker = Some((Position { x: 0, y: i }, blocker_color));
    if i != 0 {
      assert_blocked_path(test_vertical_backward::<T>(game, max_dimension_y, blocker, true));
    } else {
      assert_blocked_target(test_vertical_backward::<T>(game, max_dimension_y, blocker, false), game);
      assert_ok!(game.board.clean());
    }
  });
}

fn horizontal_vertical_path_blocked_for_both_colors<T: Piece + 'static>(
  game: &mut Game,
  max_dimension_x: i32,
  max_dimension_y: i32,
) {
  // Same color blocker
  let mut blocker_piece_color = White;
  horizontal_vertical_path_blocked::<T, _, _>(
    game,
    max_dimension_x,
    max_dimension_y,
    blocker_piece_color,
    |res| { assert_err!(res, MovementError::BlockedPath); },
    |res, _| { assert_err!(res, MovementError::BlockedPath); },
  );

  // Rival color blocker
  blocker_piece_color = Black;
  let rival_dead_pieces_before_len = game.board.dead_pieces(blocker_piece_color).len();
  horizontal_vertical_path_blocked::<T, _, _>(
    game,
    max_dimension_x,
    max_dimension_y,
    blocker_piece_color,
    |res| { assert_err!(res, MovementError::BlockedPath); },
    |res, game_after| {
      // Rival piece is killed
      assert_ok!(res);
      let rival_dead_pieces_after = game_after.board.dead_pieces(blocker_piece_color);
      // Rival dead pieces set is incremented by one
      assert!(rival_dead_pieces_after.len() == rival_dead_pieces_before_len + 1);
      assert!(rival_dead_pieces_after.last().expect("There is a piece").as_any().downcast_ref::<T>().is_some());
    },
  );
}

// DIAGONAL movement helpers
fn test_diagonal_forward_right<T: Piece + 'static>(game: &mut Game, max: i32, maybe_blocker: Option<(Position, Color)>, clean: bool) -> Result<(), MovementError> {
  let init_position = Position { x: 0, y: 0 };
  let dest_position =  Position {x: max, y: max };
  let result = add_pieces_and_move::<T>(game, init_position, dest_position, maybe_blocker);
  if clean { assert_ok!(game.board.clean()); };
  result
}

fn test_diagonal_forward_left<T: Piece + 'static>(game: &mut Game, max: i32, maybe_blocker: Option<(Position, Color)>, clean: bool) -> Result<(), MovementError> {
  let init_position = Position { x: max, y: 0 };
  let dest_position =  Position {x: 0, y: max };
  let result = add_pieces_and_move::<T>(game, init_position, dest_position, maybe_blocker);
  if clean { assert_ok!(game.board.clean()); };
  result
}

fn test_diagonal_backward_right<T: Piece + 'static>(game: &mut Game, max: i32, maybe_blocker: Option<(Position, Color)>, clean: bool) -> Result<(), MovementError> {
  let init_position = Position { x: 0, y: max };
  let dest_position =  Position {x: max, y: 0 };
  let result = add_pieces_and_move::<T>(game, init_position, dest_position, maybe_blocker);
  if clean { assert_ok!(game.board.clean()); };
  result
}

fn test_diagonal_backward_left<T: Piece + 'static>(game: &mut Game, max: i32, maybe_blocker: Option<(Position, Color)>, clean: bool) -> Result<(), MovementError> {
  let init_position = Position { x: max, y: max };
  let dest_position =  Position {x: 0, y: 0 };
  let result = add_pieces_and_move::<T>(game, init_position, dest_position, maybe_blocker);
  if clean { assert_ok!(game.board.clean()); };
  result
}

fn diagonal_movements<T: Piece + 'static, F>(game: &mut Game, max_dimension_x: i32, max_dimension_y: i32, min_dimension: i32, assertion: F) where F: Fn(Result<(), MovementError>) -> () {
  let dimension = max_dimension_x.min(max_dimension_y);

  (min_dimension..=dimension).into_iter().for_each(|i| {
    assertion(test_diagonal_forward_right::<T>(game, i, None, true));
    assertion(test_diagonal_forward_left::<T>(game, i, None, true));
    assertion(test_diagonal_backward_right::<T>(game, i, None, true));
    assertion(test_diagonal_backward_left::<T>(game, i, None, true));
  });
}

fn diagonal_path_blocked<T: Piece + 'static, F, G>(
  game: &mut Game,
  max_dimension_x: i32,
  max_dimension_y: i32,
  blocker_color: Color,
  assert_blocked_path: F,
  assert_blocked_target: G,
) where F: Fn(Result<(), MovementError>), G: Fn(Result<(), MovementError>, &mut Game) {
  let max_dimension = max_dimension_x.min(max_dimension_y);

  (1..=max_dimension).into_iter().for_each(|i| {
    let blocker_position = Some((Position { x: i, y: i }, blocker_color));
    if i != max_dimension {
      assert_blocked_path(test_diagonal_forward_right::<T>(game, max_dimension, blocker_position, true));
    } else {
      assert_blocked_target(test_diagonal_forward_right::<T>(game, max_dimension, blocker_position, false), game);
      assert_ok!(game.board.clean());
    }
  });

  (1..=max_dimension).into_iter().for_each(|i| {
    let blocker_position = Some((Position { x: max_dimension - i, y: i }, blocker_color));
    if i != max_dimension {
      assert_blocked_path(test_diagonal_forward_left::<T>(game, max_dimension, blocker_position, true));
    } else {
      assert_blocked_target(test_diagonal_forward_left::<T>(game, max_dimension, blocker_position, false), game);
      assert_ok!(game.board.clean());
    }
  });

  (1..=max_dimension).into_iter().for_each(|i| {
    let blocker_position = Some((Position { x: max_dimension - i, y: max_dimension - i }, blocker_color));
    if i != max_dimension {
      assert_blocked_path(test_diagonal_backward_left::<T>(game, max_dimension, blocker_position, true));
    } else {
      assert_blocked_target(test_diagonal_backward_left::<T>(game, max_dimension, blocker_position, false), game);
      assert_ok!(game.board.clean());
    }
  });

  (1..=max_dimension).into_iter().for_each(|i| {
    let blocker_position = Some((Position { x: i, y: max_dimension - i }, blocker_color));
    if i != max_dimension {
      assert_blocked_path(test_diagonal_backward_right::<T>(game, max_dimension, blocker_position, true));
    } else {
      assert_blocked_target(test_diagonal_backward_right::<T>(game, max_dimension, blocker_position, false), game);
      assert_ok!(game.board.clean());
    }
  });
}

fn diagonal_path_blocked_for_both_colors<T: Piece + 'static>(
  game: &mut Game,
  max_dimension_x: i32,
  max_dimension_y: i32,
) {
  // Same color blocker
  let mut blocker_piece_color = White;
  diagonal_path_blocked::<T, _, _>(
    game,
    max_dimension_x,
    max_dimension_y,
    blocker_piece_color,
    |res| { assert_err!(res, MovementError::BlockedPath); },
    |res, _| { assert_err!(res, MovementError::BlockedPath); },
  );

  // Rival color blocker
  blocker_piece_color = Black;
  let rival_dead_pieces_before_len = game.board.dead_pieces(blocker_piece_color).len();
  diagonal_path_blocked::<T, _, _>(
    game,
    max_dimension_x,
    max_dimension_y,
    blocker_piece_color,
    |res| { assert_err!(res, MovementError::BlockedPath); },
    |res, game_after| {
      // Rival piece is killed
      assert_ok!(res);
      let rival_dead_pieces_after = game_after.board.dead_pieces(blocker_piece_color);
      // Rival dead pieces set is incremented by one
      assert!(rival_dead_pieces_after.len() == rival_dead_pieces_before_len + 1);
      assert!(rival_dead_pieces_after.last().expect("There is a piece").as_any().downcast_ref::<T>().is_some());
    },
  );
}

#[test]
fn test_pawn_movements() {
  let board = create_board();
  let mut game = Game::new(board);

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
fn rook_movements() {
  let board = create_board();
  let mut game = Game::new(board);
  let max_dimension_x = game.board.dimension.x;
  let max_dimension_y = game.board.dimension.y;
  let min_dimension = 1;

  // Valid
  horizontal_vertical_movements::<Rook, _>(
    &mut game,
    max_dimension_x,
    max_dimension_y,
    min_dimension,
    |res| { assert_ok!(res); }
  );
  // Invalid
  diagonal_movements::<Rook, _>(
    &mut game,
    max_dimension_x,
    max_dimension_y,
    min_dimension,
    |res| { assert_err!(res, MovementError::IllegalMovement); }
  );
}

#[test]
fn rook_movements_path_blocked() {
  let board = create_board();
  let mut game = Game::new(board);

  let max_dimension_x = game.board.dimension.x;
  let max_dimension_y = game.board.dimension.y;

  horizontal_vertical_path_blocked_for_both_colors::<Queen>(&mut game, max_dimension_x, max_dimension_y);
}

#[test]
fn bishop_movements() {
  let board = create_board();
  let mut game = Game::new(board);

  let max_dimension_x = game.board.dimension.x;
  let max_dimension_y = game.board.dimension.y;
  let min_dimension = 1;

  // Valid
  diagonal_movements::<Bishop, _>(
    &mut game,
    max_dimension_x,
    max_dimension_y,
    min_dimension,
    |res| { assert_ok!(res); }
  );
  // Invalid
  horizontal_vertical_movements::<Bishop, _>(
    &mut game,
    max_dimension_x,
    max_dimension_y,
    min_dimension,
    |res| { assert_err!(res, MovementError::IllegalMovement); }
  );
}

#[test]
fn bishop_movements_path_blocked() {
  let board = create_board();
  let mut game = Game::new(board);

  let max_dimension_x = game.board.dimension.x;
  let max_dimension_y = game.board.dimension.y;

  diagonal_path_blocked_for_both_colors::<Bishop>(&mut game, max_dimension_x, max_dimension_y);
}

#[test]
fn queen_movements() {
  let board = create_board();
  let mut game = Game::new(board);

  let max_dimension_x = game.board.dimension.x;
  let max_dimension_y = game.board.dimension.y;
  let min_dimension = 1;

  // Valid
  horizontal_vertical_movements::<Queen, _>(
    &mut game,
    max_dimension_x,
    max_dimension_y,
    min_dimension,
    |res| { assert_ok!(res); }
  );
  diagonal_movements::<Queen, _>(
    &mut game,
    max_dimension_x,
    max_dimension_y,
    min_dimension,
    |res| { assert_ok!(res); }
  );
}

#[test]
fn queen_movements_path_blocked() {
  let board = create_board();
  let mut game = Game::new(board);

  let max_dimension_x = game.board.dimension.x;
  let max_dimension_y = game.board.dimension.y;

  horizontal_vertical_path_blocked_for_both_colors::<Queen>(&mut game, max_dimension_x, max_dimension_y);
  diagonal_path_blocked_for_both_colors::<Queen>(&mut game, max_dimension_x, max_dimension_y);
}

#[test]
fn king_movements() {
  let board = create_board();
  let mut game = Game::new(board);

  let mut max_dimension_x = 1;
  let mut max_dimension_y = 1;
  let mut min_dimension = 1;

  // Valid
  horizontal_vertical_movements::<King, _>(
    &mut game,
    max_dimension_x,
    max_dimension_y,
    min_dimension,
    |res| { assert_ok!(res); }
  );
  diagonal_movements::<King, _>(
    &mut game,
    max_dimension_x,
    max_dimension_y,
    min_dimension,
    |res| { assert_ok!(res); }
  );

  // Invalid
  max_dimension_x = game.board.dimension.x;
  max_dimension_y = game.board.dimension.y;
  min_dimension = 2;
  horizontal_vertical_movements::<King, _>(
    &mut game,
    max_dimension_x,
    max_dimension_y,
    min_dimension,
    |res| { assert_err!(res, MovementError::IllegalMovement); }
  );
  diagonal_movements::<King, _>(
    &mut game,
    max_dimension_x,
    max_dimension_y,
    min_dimension,
    |res| { assert_err!(res, MovementError::IllegalMovement); }
  );
}

#[test]
fn king_movements_path_blocked() {
  let board = create_board();
  let mut game = Game::new(board);

  let max_dimension_x = 1;
  let max_dimension_y = 1;

  horizontal_vertical_path_blocked_for_both_colors::<King>(&mut game, max_dimension_x, max_dimension_y);
  diagonal_path_blocked_for_both_colors::<King>(&mut game, max_dimension_x, max_dimension_y);
}
