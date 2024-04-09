use pyo3::prelude::*;

#[pyfunction]
fn rs_merge(count_paths: Vec<&str>, out_path: &str) {
    println!("{:?}", count_paths);
}

#[pymodule]
fn rust_utils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rs_merge, m)?)?;
    Ok(())
}
