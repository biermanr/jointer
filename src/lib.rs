use pyo3::prelude::*;

use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, Write};
use std::path::Path;

struct CountFile {
    f: io::BufReader<File>,
    chrom: String,
    pos: u32,
    ref_base: char,
    alt_base: char,
    bef_base: char,
    aft_base: char,
    ref_count: u16,
    alt_count: u16,
    ref_indel: u16,
    alt_indel: u16,
    ref_fwd: u16,
    alt_fwd: u16,
    ref_rev: u16,
    alt_rev: u16,
    exhausted: bool,
}

/// Merge counts files
#[pyfunction]
fn rs_merge(count_paths: Vec<&str>, out_path: &str) {
    println!("{:?}", count_paths);
}

#[pymodule]
fn rust_utils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rs_merge, m)?)?;
    Ok(())
}
