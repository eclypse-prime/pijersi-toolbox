use std::{
    collections::HashSet,
    fs::File,
    io::Write,
    time::Instant,
};

use indicatif::ProgressBar;

use pijersi_rs::{
    board::Board,
    logic::{
        movegen::available_player_actions, INDEX_WIDTH,
        MAX_PLAYER_ACTIONS,
    },
};

fn explore(board: &mut Board, exploration_depth: u64, positions: &mut HashSet<String>) {
    let (cells, player, half_moves, full_moves) = board.get_state();
    let actions = available_player_actions(&board.cells, board.current_player);
    match exploration_depth {
        0 => (),
        1 => {
            for &action in actions
                .iter()
                .take(actions[MAX_PLAYER_ACTIONS - 1] as usize)
            {
                board.play(action).unwrap();
                positions.insert(board.get_string_state());
                board
                    .set_state(&cells, player, half_moves, full_moves)
                    .unwrap()
            }
        }
        _ => {
            for &action in actions
                .iter()
                .take(actions[MAX_PLAYER_ACTIONS - 1] as usize)
            {
                board.play(action).unwrap();
                explore(board, exploration_depth - 1, positions);
                board
                    .set_state(&cells, player, half_moves, full_moves)
                    .unwrap()
            }
        }
    }
}

fn main() {
    let search_depth: u64 = 5;
    let exploration_depth: u64 = 2;

    let mut board: Board = Board::new();
    board.options.verbose = false;
    board.options.use_book = false;
    board.init();

    let mut positions: HashSet<String> = HashSet::new();
    println!("Searching all available positions at depth {exploration_depth}");
    let start = Instant::now();
    explore(&mut board, exploration_depth, &mut positions);
    println!(
        "{} positions found in {:?}",
        positions.len(),
        start.elapsed()
    );

    let mut actions: Vec<(String, u64, i64)> = vec![];
    let progress_bar = ProgressBar::new(positions.len() as u64);
    let start = Instant::now();
    for state_string in positions {
        board.set_string_state(&state_string).unwrap();
        if let Some((action, score)) = board.search_to_depth(search_depth, None) {
            let action = action | (search_depth << (3 * INDEX_WIDTH));
            actions.push((state_string, action, score));
        } else {
            panic!();
        }
        progress_bar.inc(1);
    }
    println!("Responses calculated at depth {search_depth} in {:?}", start.elapsed());

    let start = Instant::now();
    let mut file = File::create("openings.txt").unwrap();
    for (state_string, action, score) in actions {
        file.write_all(format!("{state_string}:{action};{score}\n").as_bytes())
            .unwrap();
    }
    println!("Positions saved in {:?}", start.elapsed())
}
