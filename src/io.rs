use std::{
    fs::File,
    io::{BufReader, BufWriter, ErrorKind, Read, Write},
    time::Instant,
};

use bincode::{deserialize, serialize};

use crate::structs::{Position, Response};

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
