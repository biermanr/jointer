use pyo3::prelude::*;

mod count_file;
use crate::count_file::CountFile;

use std::collections::BinaryHeap;

/// Merge counts files
#[pyfunction]
fn rs_merge(count_paths: Vec<&str>, out_path: &str) {
    let mut cfs: BinaryHeap<CountFile> = count_paths
        .iter()
        .map(|x| CountFile::new(x))
        .collect();

    

}

#[pymodule]
fn rust_utils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rs_merge, m)?)?;
    Ok(())
}
