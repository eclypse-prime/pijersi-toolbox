use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use indicatif::{ProgressBar, ProgressStyle};
use pijersi_rs::{
    board::Board,
    logic::{movegen::available_player_actions, translate::action_to_string, INDEX_WIDTH},
    search::eval::MAX_SCORE,
};

use crate::structs::{Position, Response};

pub fn get_positions(exploration_depth: u64) -> Vec<Position> {
    let mut board: Board = Board::new();
    board.options.verbose = false;
    board.options.use_book = false;
    board.init();
    println!("Searching all available positions at depth {exploration_depth}...");
    let start = Instant::now();
    let positions = _get_positions(&mut board, exploration_depth);
    println!(
        "{} positions found in {:?}",
        positions.len(),
        start.elapsed()
    );
    positions.into_iter().collect()
}

fn _get_positions(board: &mut Board, exploration_depth: u64) -> HashSet<Position> {
    // Saving state for unmake
    let (cells, player, half_moves, full_moves) = board.get_state();

    // Early return on win/draw
    if board.is_win() || board.is_draw() {
        return HashSet::new();
    }

    let mut positions: HashSet<Position> = HashSet::new();
    let (actions, n_actions) = available_player_actions(&board.cells, board.current_player);

    match exploration_depth {
        0 => {
            positions.insert(Position::new(board));
            positions
        }
        1 => {
            for &action in actions.iter().take(n_actions) {
                board.play(action).unwrap();
                if !board.is_win() {
                    positions.insert(Position::new(board));
                }
                board
                    .set_state(&cells, player, half_moves, full_moves)
                    .unwrap()
            }
            positions
        }
        _ => {
            for &action in actions.iter().take(n_actions) {
                board.play(action).unwrap();
                positions.extend(_get_positions(board, exploration_depth - 1));
                board
                    .set_state(&cells, player, half_moves, full_moves)
                    .unwrap()
            }
            positions
        }
    }
}

pub fn get_responses_at_depth(positions: &[Position], search_depth: u64) -> Vec<Response> {
    let mut board: Board = Board::new();
    board.options.verbose = false;
    board.options.use_book = false;
    board.init();

    println!("Calculating responses at depth {search_depth}...");
    let start = Instant::now();
    let mut responses: Vec<Response> = vec![];
    let progress_bar = ProgressBar::new(positions.len() as u64);
    progress_bar.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] [{wide_bar}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );
    for position in positions {
        board
            .set_state(&position.cells, position.current_player, 0, 0)
            .unwrap();
        if let Some((action, score)) = board.search_to_depth(search_depth, None) {
            let action = action | (search_depth << (3 * INDEX_WIDTH));
            responses.push(Response::new(position.to_owned(), action, score));
        } else {
            panic!();
        }
        progress_bar.inc(1);
    }
    println!(
        "Responses calculated at depth {search_depth} in {:?}.",
        start.elapsed()
    );

    responses
}

pub fn backtrack_responses(
    responses: &[Response],
    search_depth: u64,
    exploration_depth: u64,
) -> Vec<Response> {
    let mut board: Board = Board::new();
    board.options.verbose = false;
    board.options.use_book = false;
    board.init();

    println!("Backtracking responses at exploration depth {exploration_depth} and search depth {search_depth}...");
    let start = Instant::now();
    let responses_map: HashMap<[u8; 45], (u64, i64)> =
        HashMap::from_iter(responses.iter().map(|response| {
            (
                response.position.cells.to_owned(),
                (response.action, response.score),
            )
        }));

    let lower_positions = get_positions(exploration_depth - 1);
    let mut lower_responses: Vec<Response> = vec![];
    for position in lower_positions {
        board
            .set_state(&position.cells, position.current_player, 0, 0)
            .unwrap();
        let (cells, player, half_moves, full_moves) = board.get_state();
        let (actions, n_actions) = available_player_actions(&board.cells, board.current_player);
        let mut best_score = i64::MIN;
        let mut best_action: u64 = 0;
        for &action in actions.iter().take(n_actions) {
            board.play(action).unwrap();
            if board.is_win() {
                best_score = MAX_SCORE;
                best_action = action;
                break;
            } else {
                let &(_action, score) = responses_map.get(&board.cells).unwrap();
                let score = -score;
                if score > best_score {
                    best_score = score;
                    best_action = action;
                }
            }
            board
                .set_state(&cells, player, half_moves, full_moves)
                .unwrap();
        }
        lower_responses.push(Response::new(
            position,
            best_action | ((search_depth + 1) << (3 * INDEX_WIDTH)),
            best_score,
        ));
    }
    println!(
        "Responses calculated at new exploration depth {} and search depth {} in {:?}.",
        exploration_depth - 1,
        search_depth + 1,
        start.elapsed()
    );
    lower_responses
}

pub fn inspect_position(position: &Position) {
    let mut board: Board = Board::new();
    board.options.verbose = false;
    board.options.use_book = false;
    board.init();

    board
        .set_state(&position.cells, position.current_player, 0, 0)
        .unwrap();
    board.print();
    println!();
}

pub fn inspect_response(response: &Response) {
    let mut board: Board = Board::new();
    board.options.verbose = false;
    board.options.use_book = false;
    board.init();

    board
        .set_state(
            &response.position.cells,
            response.position.current_player,
            0,
            0,
        )
        .unwrap();
    board.print();
    println!("{}", board.current_player);
    let action_string = action_to_string(&board.cells, response.action);
    println!("{action_string}");
    board.play(response.action).unwrap();
    board.print();
    println!();
}
