use pyo3::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[pyfunction]
fn join(file_path1: &str, file_path2: &str) {
    let mut f1_lines = read_lines(file_path1).expect("Could not read lines from file 1");
    let mut f2_lines = read_lines(file_path2).expect("Could not read lines from file 2");

    let mut f1_line = f1_lines.next().expect("Could not iterate file1").unwrap();
    let mut f2_line = f2_lines.next().expect("Could not iterate file2").unwrap();

    let (mut f1_key,  mut _f1_remainder) = f1_line.split_once("\t").expect("Could not split line in f1");
    let (mut f2_key,  mut  f2_remainder) = f2_line.split_once("\t").expect("Could not split line in f2");

    loop {

        if f1_key == f2_key {
            println!("{}\t{}",f1_line, f2_remainder);
        }

        //Advance the iterators or break the loop if exhuasted
        let advance_f1 = f1_key <= f2_key;
        let advance_f2 = f2_key <= f1_key;

        if advance_f1 {
            f1_line = match f1_lines.next() {
                Some(Ok(l)) => l,
                _ => break,
            };
            (f1_key, _f1_remainder) = f1_line.split_once("\t").expect("Could not split line in f1");
        }

        if advance_f2 {
            f2_line = match f2_lines.next() {
                Some(Ok(l)) => l,
                _ => break,
            };
            (f2_key,  f2_remainder) = f2_line.split_once("\t").expect("Could not split line in f2");
        }
    }
}

#[pymodule]
fn rust_utils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(join, m)?)?;
    Ok(())
}

//Buffered read lines copied from "Rust By Example"
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
