use std::{
    fs::File,
    io::{BufReader, BufWriter, ErrorKind, Read, Write},
    time::Instant,
};

use bincode::{deserialize, serialize};
use miniz_oxide::{deflate::compress_to_vec, inflate::decompress_to_vec};
use pijersi_rs::search::openings::{Position, Response};

fn decode_position(file: &mut BufReader<File>) -> Option<Position> {
    let mut bytes: [u8; 54] = [0; 54];
    let result = file.read_exact(&mut bytes);
    match result {
        Ok(()) => (),
        Err(err) if err.kind() == ErrorKind::UnexpectedEof => return None,
        Err(_) => panic!(),
    }
    let position: Option<Position> = Some(deserialize(&bytes[..]).unwrap());
    position
}

fn encode_position(position: &Position) -> Vec<u8> {
    serialize(position).unwrap()
}

fn decode_response(file: &mut BufReader<File>) -> Option<Response> {
    let mut bytes: [u8; 70] = [0; 70];
    let result = file.read_exact(&mut bytes);
    match result {
        Ok(()) => (),
        Err(err) if err.kind() == ErrorKind::UnexpectedEof => return None,
        Err(_) => panic!(),
    }
    let response: Option<Response> = Some(deserialize(&bytes[..]).unwrap());
    response
}

fn encode_response(response: &Response) -> Vec<u8> {
    serialize(response).unwrap()
}

pub fn import_positions(file_path: &str) -> Vec<Position> {
    let mut reader = BufReader::new(File::open(file_path).expect("Cannot open file.txt"));

    let mut positions: Vec<Position> = vec![];

    println!("Reading positions from file {file_path}...");
    let start = Instant::now();

    while let Some(position) = decode_position(&mut reader) {
        positions.push(position);
    }
    println!(
        "{} positions loaded in {:?}.",
        positions.len(),
        start.elapsed()
    );
    positions
}

pub fn export_positions(positions: &[Position], save_path: &str, split: Option<u64>) {
    println!("Saving positions to file {save_path}...");
    let start = Instant::now();
    if let Some(n_files) = split {
        let chunk_size = positions.len() / n_files as usize + 1;
        let chunks = positions.chunks(chunk_size);
        for (index, chunk) in chunks.enumerate() {
            let mut writer = BufWriter::new(File::create(format!("{save_path}_{index}")).unwrap());
            for position in chunk {
                writer.write_all(&encode_position(position)).unwrap();
            }
        }
        println!(
            "Positions saved in {} chunks in {:?}",
            n_files,
            start.elapsed()
        );
    } else {
        let mut writer = BufWriter::new(File::create(save_path).unwrap());
        for position in positions {
            writer.write_all(&encode_position(position)).unwrap();
        }
        println!("Positions saved in {:?}", start.elapsed());
    }
}

pub fn import_responses(file_path: &str) -> Vec<Response> {
    let mut reader = BufReader::new(File::open(file_path).expect("Cannot open file."));

    let mut responses: Vec<Response> = vec![];

    println!("Reading responses from file {file_path}...");
    let start = Instant::now();

    while let Some(response) = decode_response(&mut reader) {
        responses.push(response);
    }

    println!(
        "{} responses loaded in {:?}.",
        responses.len(),
        start.elapsed()
    );
    responses
}

pub fn export_responses(responses: &[Response], save_path: &str) {
    println!("Saving responses to file {save_path}...");
    let start = Instant::now();
    let mut file = File::create(save_path).unwrap();
    for response in responses {
        file.write_all(&encode_response(response)).unwrap();
    }
    println!("Responses saved in {:?}.", start.elapsed());
}

pub fn compress_file(input_path: &str, output_path: &str) {
    let mut bytes: Vec<u8> = vec![];
    println!("Loading file {input_path}...");
    let start = Instant::now();
    File::open(input_path).unwrap().read_to_end(&mut bytes).unwrap();
    println!("File loaded in {:?}", start.elapsed());
    
    let old_size = bytes.len();
    println!("Compressing {}kB...", old_size as f64 / 1024.);
    let start = Instant::now();
    let compressed_bytes = compress_to_vec(&bytes, 10);
    let new_size = compressed_bytes.len();
    println!("Compressed to {}kB in {:?} (compression ratio: {})", new_size as f64 / 1024., start.elapsed(), new_size as f64 / old_size as f64);
    
    println!("Saving to {input_path}...");
    let start = Instant::now();
    let mut output_file = File::create(output_path).unwrap();
    output_file.write_all(&compressed_bytes).unwrap();
    println!("File saved in {:?}", start.elapsed());
}

pub fn decompress_file(input_path: &str, output_path: &str) {
    let mut bytes: Vec<u8> = vec![];
    println!("Loading file {input_path}...");
    let start = Instant::now();
    File::open(input_path).unwrap().read_to_end(&mut bytes).unwrap();
    println!("File loaded in {:?}", start.elapsed());
    
    let old_size = bytes.len();
    println!("Deompressing {}kB...", old_size as f64 / 1024.);
    let start = Instant::now();
    let compressed_bytes = decompress_to_vec(&bytes).unwrap();
    let new_size = compressed_bytes.len();
    println!("Deompressed to {}kB in {:?} (decompression ratio: {})", new_size as f64 / 1024., start.elapsed(), new_size as f64 / old_size as f64);
    
    println!("Saving to {input_path}...");
    let start = Instant::now();
    let mut output_file = File::create(output_path).unwrap();
    output_file.write_all(&compressed_bytes).unwrap();
    println!("File saved in {:?}", start.elapsed());
}
