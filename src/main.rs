use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Write},
    time::Instant,
};

use indicatif::{ProgressBar, ProgressStyle};

use pijersi_rs::{
    board::Board,
    logic::{
        movegen::available_player_actions,
        translate::string_to_cells,
        INDEX_WIDTH, MAX_PLAYER_ACTIONS,
    },
};

fn get_positions(board: &mut Board, exploration_depth: u64) -> HashSet<String> {
    // Saving state for unmake
    let (cells, player, half_moves, full_moves) = board.get_state();

    // Early return on win/draw
    if board.is_win() || board.is_draw() {
        return HashSet::new();
    }

    let mut positions: HashSet<String> = HashSet::new();
    let actions = available_player_actions(&board.cells, board.current_player);

    match exploration_depth {
        0 => {
            positions.insert(board.get_string_state());
            positions
        },
        1 => {
            for &action in actions
                .iter()
                .take(actions[MAX_PLAYER_ACTIONS - 1] as usize)
            {
                board.play(action).unwrap();
                if !board.is_win() {
                    positions.insert(board.get_string_state());
                }
                board
                    .set_state(&cells, player, half_moves, full_moves)
                    .unwrap()
            }
            positions
        }
        _ => {
            for &action in actions
                .iter()
                .take(actions[MAX_PLAYER_ACTIONS - 1] as usize)
            {
                board.play(action).unwrap();
                positions.extend(get_positions(board, exploration_depth - 1));
                board
                    .set_state(&cells, player, half_moves, full_moves)
                    .unwrap()
            }
            positions
        }
    }
}

// 5, 2
fn get_responses_at_depth(
    search_depth: u64,
    exploration_depth: u64,
) -> Vec<(String, u64, i64)> {
    let mut board: Board = Board::new();
    board.options.verbose = false;
    board.options.use_book = false;
    board.init();

    println!("Searching all available positions at depth {exploration_depth}");
    let start = Instant::now();
    let positions = get_positions(&mut board, exploration_depth);
    println!(
        "{} positions found in {:?}",
        positions.len(),
        start.elapsed()
    );

    let mut responses: Vec<(String, u64, i64)> = vec![];
    let progress_bar = ProgressBar::new(positions.len() as u64);
    progress_bar.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] [{wide_bar}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );
    let start = Instant::now();
    for state_string in positions {
        board.set_string_state(&state_string).unwrap();
        if let Some((action, score)) = board.search_to_depth(search_depth, None) {
            let action = action | (search_depth << (3 * INDEX_WIDTH));
            responses.push((state_string, action, score));
        } else {
            panic!();
        }
        progress_bar.inc(1);
    }
    println!(
        "Responses calculated at depth {search_depth} in {:?}",
        start.elapsed()
    );

    responses
}

fn backtrack_responses(
    responses: &[(String, u64, i64)],
    search_depth: u64,
    exploration_depth: u64,
) -> Vec<(String, u64, i64)> {
    let mut board: Board = Board::new();
    board.options.verbose = false;
    board.options.use_book = false;
    board.init();

    let responses_map: HashMap<[u8; 45], (u64, i64)> =
        HashMap::from_iter(responses.iter().map(|(state, action, score)| {
            (
                string_to_cells(state.split(' ').next().unwrap()).unwrap(),
                (*action, *score),
            )
        }));

    let positions = get_positions(&mut board, exploration_depth - 1);
    let mut lower_responses: Vec<(String, u64, i64)> = vec![];
    for state_string in positions {
        board.set_string_state(&state_string).unwrap();
        let (cells, player, half_moves, full_moves) = board.get_state();
        let actions = available_player_actions(&board.cells, board.current_player);
        let mut best_score = i64::MIN;
        let mut best_action: u64 = 0;
        for &action in actions
            .iter()
            .take(actions[MAX_PLAYER_ACTIONS - 1] as usize)
        {
            board.play(action).unwrap();
            let &(_action, score) = responses_map.get(&board.cells).unwrap();
            let score = -score;
            if score > best_score {
                best_score = score;
                best_action = action;
            }
            board
                .set_state(&cells, player, half_moves, full_moves)
                .unwrap();
        }
        lower_responses.push((
            state_string,
            best_action | ((search_depth + 1) << (3 * INDEX_WIDTH)),
            best_score,
        ));
    }
    lower_responses
}

fn import_responses(file_path: &str) -> Vec<(String, u64, i64)> {
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file.txt"));
    let lines = reader.lines();

    let mut responses: Vec<(String, u64, i64)> = vec![];

    println!("Reading responses from file");
    let start = Instant::now();
    for line in lines {
        if let [fen, action_str, score_str] = line.unwrap().split(';').collect::<Vec<&str>>()[..] {
            let action: u64 = action_str.parse().unwrap();
            let score: i64 = score_str.parse().unwrap();
            responses.push((fen.to_owned(), action, score));
        }
    }
    println!(
        "{} responses found in {:?}",
        responses.len(),
        start.elapsed()
    );
    responses
}

fn export_responses(responses: Vec<(String, u64, i64)>, save_path: &str) {
    let start = Instant::now();
    let mut file = File::create(save_path).unwrap();
    for (state_string, action, score) in responses {
        file.write_all(format!("{state_string};{action};{score}\n").as_bytes())
            .unwrap();
    }
    println!("Positions saved in {:?}", start.elapsed());
}

fn main() {
    // let mut responses = import_responses("_openings.txt");
    // let lower_responses = backtrack_responses(&responses, 5, 2);
    // let lowest_responses = backtrack_responses(&lower_responses, 6, 1);
    // responses.extend(lower_responses);
    // responses.extend(lowest_responses);
    // export_responses(responses, "openings.txt");
}
