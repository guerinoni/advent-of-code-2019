use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub trait Solver {
    fn new(input_file: &'static str) -> Self;
    fn solve(&self) -> String;
}

pub fn file_to_vec(path: &str) -> Result<Vec<i64>, std::io::Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut v = Vec::new();
    for line in br.lines() {
        let line = line?;
        let n = line
            .trim()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        v.push(n);
    }
    Ok(v)
}

pub fn file_with_comma_to_vec(path: &str) -> Vec<i64> {
    std::fs::read_to_string(path)
        .expect("file not found!")
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>()
}

pub fn file_to_vec_of_vec_string(path: &str) -> Vec<Vec<String>> {
    let file = File::open(path).expect("cannot open file");
    let br = BufReader::new(file);
    let mut v = Vec::new();
    for l in br.lines() {
        let line = l.unwrap();
        let vec = line.split(',').map(String::from).collect::<Vec<String>>();
        v.push(vec);
    }

    v
}

pub fn file_to_vec_of_string(path: &str) -> Vec<String> {
    let file = File::open(path).expect("cannot open file");
    let br = BufReader::new(file);
    let mut v = Vec::new();
    for l in br.lines() {
        let line = l.unwrap();
        v.push(line);
    }

    v
}
