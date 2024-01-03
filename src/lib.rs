use pyo3::prelude::*;

mod count_file;
use crate::count_file::CountFile;

use std::collections::BinaryHeap;

use std::io::{BufWriter, Write};
use std::fs::File;

/// Merge counts files
#[pyfunction]
fn rs_merge(count_paths: Vec<&str>, out_path: &str) {

    //Open the output file for writing
    let out_file = File::create(out_path).unwrap();
    let mut out_file = BufWriter::new(out_file);

    let mut cfs: BinaryHeap<CountFile> = count_paths
        .iter()
        .map(|x| CountFile::new(x))
        .collect();

    while !cfs.is_empty() {
        //Pop off the minimum position
        let mut min_cf = cfs.pop().unwrap();
        let mut locus_data = min_cf.data;

        //Add the min cf back to the heap if it's not exhausted
        if min_cf.next() {
            cfs.push(min_cf);
        }

        //Aggregate the counts at the current position
        while !cfs.is_empty() {
            let mut cf = cfs.pop().unwrap();
            let data = cf.data;

            //If the data is the same position as the min locus
            //then add it to the locus_data, call next on the pointer,
            //and then push back onto the heap
            if data == locus_data {
                locus_data += data;

                if cf.next() {
                    cfs.push(cf);
                }

            //Otherwise, push the pointer back without advancing it
            //and break out of the while loop
            } else {
                cfs.push(cf);
                break;
            }
        }

        //writeout the data at this locus
        writeln!(out_file, "{}", locus_data).unwrap();

    }

    //Flush the output file
    out_file.flush().unwrap();

}

#[pymodule]
fn rust_utils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rs_merge, m)?)?;
    Ok(())
}
