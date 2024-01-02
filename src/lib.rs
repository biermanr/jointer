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
        let mut min_cf = cfs.pop().unwrap();
        let mut locus_data = min_cf.data;

        //Add the min cf back to the heap if it's not exhausted
        if min_cf.next() {
            cfs.push(min_cf);
        }

        //Aggregate the counts at the current position
        while cfs.len() > 0 {
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

        //TODO writeout aggregated values


    }

}

#[pymodule]
fn rust_utils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rs_merge, m)?)?;
    Ok(())
}
