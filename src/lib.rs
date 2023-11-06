use pyo3::prelude::*;

//use std::collections::BinaryHeap;
//use std::cmp::Ordering;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;

struct CountFile {
    f: io::Lines<io::BufReader<fs::File>>,
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

impl CountFile {
    fn new(count_path: &str) -> CountFile {

        CountFile {
            f : Self::read_lines(count_path).expect(
                "Error, could not read file "),
            chrom : String::new(),
            pos : 0,
            ref_base : 'N',
            alt_base : 'N',
            bef_base : 'N',
            aft_base : 'N',
            ref_count : 0,
            alt_count : 0,
            ref_indel : 0,
            alt_indel : 0,
            ref_fwd : 0,
            alt_fwd : 0,
            ref_rev : 0,
            alt_rev : 0,
            exhausted : false,
        }
        //next???
    }

    //NOTE
    //fn next(&self) {
        //???
    //}

    ///Create buffered line reader of a file
    fn read_lines<P>(fname: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where P: AsRef<Path>, {
        let file = fs::File::open(fname)?;
        Ok(io::BufReader::new(file).lines())
    }
     
}


/// Merge counts files
#[pyfunction]
fn rs_merge(count_paths: Vec<&str>, out_path: &str) {
    let mut cfs: Vec<CountFile> = Vec::new();
    for p in count_paths {
        cfs.push(CountFile::new(&p));
    }
}

#[pymodule]
fn rust_utils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rs_merge, m)?)?;
    Ok(())
}
