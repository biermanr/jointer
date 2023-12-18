use pyo3::prelude::*;

mod count_file;
use crate::count_file::CountFile;

use std::collections::BinaryHeap;

/// Merge counts files
#[pyfunction]
fn rs_merge(count_paths: Vec<&str>, _out_path: &str) {
    let mut cfs: BinaryHeap<CountFile> = count_paths
        .iter()
        .map(|x| CountFile::new(x))
        .collect();

    while cfs.len() > 0 {
        //the first CountFile position will be the min
        //NOTE this is a bug! the "top" cf needs to be added
        //back to the heap!
        let mut locus_data = cfs.pop().unwrap().data;

        //Aggregate the counts at the current position
        //keep popping until a greater value is found
        //then add that back onto the heap
        while cfs.len() > 0 {
            let cf = cfs.pop().unwrap();
            let data = cf.data;

            if cf.next() {
                cfs.push(cf);
            }

            if data == locus_data {
                locus_data += data;
            } else {
                break;
            }
        }

        //TODO writeout aggregated values


    }

}

#[pymodule]
fn rust_utils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rs_merge, m)?)?;
    Ok(())
}
