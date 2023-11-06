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
        let min_cf = cfs.pop().unwrap();
        let mut curr_cfs: Vec<CountFile> = Vec::new();

        //keep popping until a greater value is found
        while cfs.len() > 0 {
            let cf = cfs.pop().unwrap();
            if cf <= min_cf {
                curr_cfs.push(cf);
            } else {
                cfs.push(cf);
            }
        }
        curr_cfs.push(min_cf);

        //Aggregate the CountFile objects to prepare for outpu
        //NOTE
        

        //Write the output to the out_path
        //NOTE

        //add back to the BinaryHeap if not spent
        for mut cf in curr_cfs {
            if cf.next() {
                cfs.push(cf);
            }
        }
    }

}

#[pymodule]
fn rust_utils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rs_merge, m)?)?;
    Ok(())
}
